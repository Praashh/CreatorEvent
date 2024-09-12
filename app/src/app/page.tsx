import Link from "next/link";

import { siteConfig } from "@/config/site";
import { cn } from "@/lib/utils";
import { Button, buttonVariants } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  CoinsIcon,
  FileTextIcon,
  ImageIcon,
  ShieldIcon,
  WalletIcon,
} from "lucide-react";
import ComingSoon from "@/components/coming-soon";

const actionCards: Array<{
  title: string;
  href: string;
  description: React.ReactNode;
  icon: React.ReactNode;
}> = [
    {
      title: "On-chain Memo",
      href: "/memo",
      description: "Send a simple message on-chain using an SPL Memo.",
      icon: <FileTextIcon className="size-12" />,
    },
    {
      title: "Staking SOL",
      href: "/stake",
      description:
        "Help secure the Solana network by staking SOL to a validator.",
      icon: <ShieldIcon className="size-12" />,
    },
    {
      title: "Transfer Native SOL",
      href: "/transfer-sol",
      description: "Easily transfer native SOL to any other Solana wallet.",
      icon: <WalletIcon className="size-12" />,
    },
  ];

export default async function Pages() {
  return (
    <>
      <section
        id="features"
        className={
          "container space-y-12 bg-slate-50 py-8 dark:bg-transparent md:py-12 lg:py-24"
        }
      >
        <div className="mx-auto flex max-w-[58rem] flex-col items-center space-y-6 text-center">
          <h2 className="font-heading text-3xl leading-[1.1] sm:text-3xl md:text-6xl">
            NFTs -
            New, Fairer
            Ticketing.
          </h2>
          <p className="max-w-[85%] leading-normal text-muted-foreground sm:text-lg sm:leading-7">
          Buy and sell NFT tickets using blinks and collectables on our transparent marketplace.
          </p>
        </div>
        <ComingSoon/>

        <div className="mx-auto text-center md:max-w-[58rem]">
          <p className="leading-normal text-muted-foreground sm:text-lg sm:leading-7">
            You can find the{" "}
            <Button variant={"link"} asChild>
              <Link href={siteConfig.links.github} target="_blank">
                full source code
              </Link>
            </Button>{" "}
            for this entire repo on GitHub.
          </p>
        </div>
      </section>
    </>
  );
}
