use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

declare_id!("F8hZjtco3YmBv3hhoPTHVXqQM2LD3JgkctQcaNCsw4FJ");

use mpl_core::{
    fetch_external_plugin_adapter_data_info, 
    fetch_plugin, 
    instructions::{
        CreateCollectionV2CpiBuilder, 
        CreateV2CpiBuilder, 
        WriteExternalPluginAdapterDataV1CpiBuilder, 
        UpdatePluginV1CpiBuilder
    }, 
    accounts::{BaseAssetV1, BaseCollectionV1}, 
    types::{
        AppDataInitInfo, Attribute, Attributes, 
        ExternalPluginAdapterInitInfo, ExternalPluginAdapterKey, 
        ExternalPluginAdapterSchema, PermanentBurnDelegate,
        PermanentFreezeDelegate, PermanentTransferDelegate, Plugin, 
        PluginAuthority, PluginAuthorityPair, PluginType
    }, 
};

pub mod instructions;
pub mod states;
pub mod errors;

pub use instructions::*;
pub use errors::*;

#[program]
pub mod nf_tickets {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,  name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }

    pub fn setup_manager(ctx: Context<SetupManager>) -> Result<()> {
        ctx.accounts.manager.bump = ctx.bumps.manager;
        Ok(())
    }

    pub fn create_event(ctx: Context<CreateEvent>, args: CreateEventArgs) -> Result<()> {
        // Add an Attribute Plugin that will hold the event details
        let mut collection_plugin: Vec<PluginAuthorityPair> = vec![];
    
        let attribute_list: Vec<Attribute> = vec![
            Attribute {
                key: "Category".to_string(),
                value: args.category
            },
            Attribute { 
                key: "City".to_string(), 
                value: args.city 
            },
            Attribute { 
                key: "Venue".to_string(), 
                value: args.venue 
            },
            Attribute { 
                key: "Artist".to_string(), 
                value: args.artist 
            },
            Attribute { 
                key: "Date".to_string(), 
                value: args.date 
            },
            Attribute { 
                key: "Time".to_string(), 
                value: args.time 
            },
            Attribute { 
                key: "Capacity".to_string(), 
                value: args.capacity.to_string() 
            },
            Attribute { 
                key: "IsTicketTransferable".to_string(), 
                value: args.is_ticket_transferable.to_string() 
            }
        ];
        
        collection_plugin.push(
            PluginAuthorityPair { 
                plugin: Plugin::Attributes(Attributes { attribute_list }), 
                authority: Some(PluginAuthority::UpdateAuthority) 
            }
        );
        
        // Create the Collection that will hold the tickets
        CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .collection(&ctx.accounts.event.to_account_info())
        .update_authority(Some(&ctx.accounts.manager.to_account_info()))
        .payer(&ctx.accounts.payer.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .name(args.name)
        .uri(args.uri)
        .plugins(collection_plugin)
        .invoke()?;
    
        Ok(())
    }

    pub fn create_ticket(ctx: Context<CreateTicket>, args: CreateTicketArgs) -> Result<()> {
        // Check that the maximum number of tickets has not been reached yet
        let (_, collection_attribute_list, _) = fetch_plugin::<BaseCollectionV1, Attributes>(
                &ctx.accounts.event.to_account_info(), 
                PluginType::Attributes
            )?;
    
        // Search for the Capacity attribute
        let capacity_attribute = collection_attribute_list
            .attribute_list
            .iter()
            .find(|attr| attr.key == "Capacity")
            .ok_or(TicketError::MissingAttribute)?;
    
        // Unwrap the Capacity attribute value
        let capacity = capacity_attribute
            .value
            .parse::<u32>()
            .map_err(|_| TicketError::NumericalOverflow)?;
    
        require!(
            ctx.accounts.event.num_minted < capacity, 
            TicketError::MaximumTicketsReached
        );

        let price_attribute = collection_attribute_list
            .attribute_list
            .iter()
            .find(|attr| attr.key == "Price")
            .ok_or(TicketError::MissingAttribute)?;

        let price = price_attribute
            .value
            .parse::<u64>()
            .map_err(|_| TicketError::NumericalOverflow)?;

        // Transfer funds from buyer to platform treasury using Anchor's transfer
        let transfer_cpi = Transfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        };

        transfer(CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_cpi), price)?;

        // Add an Attribute Plugin that will hold the ticket details
        let mut ticket_plugin: Vec<PluginAuthorityPair> = vec![];
        
        let mut attribute_list: Vec<Attribute> = vec![
            Attribute { 
                key: "Ticket Number".to_string(), 
                value: ctx.accounts.event.num_minted.checked_add(1).ok_or(TicketError::NumericalOverflow)?.to_string()
            },
            Attribute { 
                key: "Price".to_string(), 
                value: args.price.to_string() 
            }
        ];
        
        // Add Row attribute if provided
        if let Some(row) = args.row {
            attribute_list.push(Attribute {
                key: "Row".to_string(),
                value: row,
            });
        }

        // Add Seat attribute if provided
        if let Some(seat) = args.seat {
            attribute_list.push(Attribute {
                key: "Seat".to_string(),
                value: seat,
            });
        }

        // Add Screen attribute if provided
        if let Some(screen) = args.screen {
            attribute_list.push(Attribute {
                key: "Screen".to_string(),
                value: screen,
            });
        }
        
        ticket_plugin.push(
            PluginAuthorityPair { 
                plugin: Plugin::Attributes(Attributes { attribute_list }), 
                authority: Some(PluginAuthority::UpdateAuthority) 
            }
        );

        let is_ticket_transferable = collection_attribute_list
            .attribute_list
            .iter()
            .find(|attr| attr.key == "IsTicketTransferable")
            .map(|attr| attr.value.to_lowercase() == "true")
            .unwrap_or(false);
        
        ticket_plugin.push(
            PluginAuthorityPair { 
                plugin: Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate { frozen: !is_ticket_transferable }), 
                authority: Some(PluginAuthority::UpdateAuthority) 
            }
        );
        
        ticket_plugin.push(
            PluginAuthorityPair { 
                plugin: Plugin::PermanentBurnDelegate(PermanentBurnDelegate {}), 
                authority: Some(PluginAuthority::UpdateAuthority) 
            }
        );
        
        ticket_plugin.push(
            PluginAuthorityPair { 
                plugin: Plugin::PermanentTransferDelegate(PermanentTransferDelegate {}), 
                authority: Some(PluginAuthority::UpdateAuthority) 
            }
        );
    
        let mut ticket_external_plugin: Vec<ExternalPluginAdapterInitInfo> = vec![];
        
        ticket_external_plugin.push(ExternalPluginAdapterInitInfo::AppData(
            AppDataInitInfo {
                init_plugin_authority: Some(PluginAuthority::UpdateAuthority),
                data_authority: PluginAuthority::Address{ address: args.venue_authority },
                schema: Some(ExternalPluginAdapterSchema::Binary),
            }
        ));
    
        let signer_seeds = &[b"manager".as_ref(), &[ctx.accounts.manager.bump]];
    
        // Create the Ticket
        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.ticket.to_account_info())
            .collection(Some(&ctx.accounts.event.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .authority(Some(&ctx.accounts.manager.to_account_info()))
            .owner(Some(&ctx.accounts.signer.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .plugins(ticket_plugin)
            .external_plugin_adapters(ticket_external_plugin)
            .invoke_signed(&[signer_seeds])?;
    
        Ok(())
    }

    pub fn scan_ticket(ctx: Context<ScanTicket>) -> Result<()> {
        // Check if the ticket has already been scanned
        let (_, app_data_length) = fetch_external_plugin_adapter_data_info::<BaseAssetV1>(
            &ctx.accounts.ticket.to_account_info(), 
            None, 
            &ExternalPluginAdapterKey::AppData(
                PluginAuthority::Address { address: ctx.accounts.signer.key() }
            )
        )?;
    
        require!(app_data_length == 0, TicketError::TicketAlreadyScanned);
    
        // Prepare data to mark the ticket as scanned
        let data: Vec<u8> = "Scanned".as_bytes().to_vec();
    
        // Write the "Scanned" data to the ticket
        WriteExternalPluginAdapterDataV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.ticket.to_account_info())
            .collection(Some(&ctx.accounts.event.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .key(ExternalPluginAdapterKey::AppData(PluginAuthority::Address { address: ctx.accounts.signer.key() }))
            .data(data)
            .invoke()?;
    
        // Prepare signer seeds for the next operation
        let signer_seeds = &[b"manager".as_ref(), &[ctx.accounts.manager.bump]];
    
        // Update the plugin to freeze the ticket
        UpdatePluginV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.ticket.to_account_info())
            .collection(Some(&ctx.accounts.event.to_account_info()))
            .payer(&ctx.accounts.payer.to_account_info())
            .authority(Some(&ctx.accounts.manager.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .plugin(Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate { frozen: true }))
            .invoke_signed(&[signer_seeds])?;
    
        Ok(())
    }

    pub fn withdraw_from_treasury(ctx: Context<WithdrawFromTreasury>, amount: u64) -> Result<()> {
        let platform_key = ctx.accounts.platform.key();
        let seeds = &[
            b"treasury",
            platform_key.as_ref(),
            &[ctx.accounts.platform.treasury_bump],
        ];
        let signer_seeds = &[&seeds[..]];
    
        // Transfer funds from treasury to admin
        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.treasury.to_account_info(),
                    to: ctx.accounts.admin.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;
    
        Ok(())
    }

}