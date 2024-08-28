"use client"

import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

export default function Home() {
  return (
    <div className='h-screen w-full flex justify-center items-center'>
    <WalletMultiButton/>
</div>
  );
}
