import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { FC, useEffect, useState } from "react";

export const BalanceDisplay: FC = () => {
  const [balance, setBalance] = useState(0);
  const { connection } = useConnection();
  const { publicKey } = useWallet();

confirm(`public key ${publicKey}`)
  useEffect(() => {
    if (!connection || !publicKey) {
        return;
    }

    connection.onAccountChange(
        publicKey,
        (updatedAccountInfo) => {
            setBalance(updatedAccountInfo.lamports / LAMPORTS_PER_SOL);
        },
        "confirmed",
    );

    connection.getAccountInfo(publicKey).then((info) => {
        if (info) {
            setBalance(info.lamports);
        }
    });
  }, [connection, publicKey]);

  return (
    <div>
      <p >{publicKey ? `Balance: ${balance / LAMPORTS_PER_SOL} SOL` : ""}</p>
    </div>
  );
};