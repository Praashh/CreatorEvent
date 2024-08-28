import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import Provider from "./walletcontextprovider"

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Fund Me",
  description: "Fund Me",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>  <Provider>{children}</Provider></body>
    </html>
  );
}
