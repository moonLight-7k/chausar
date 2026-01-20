"use client";

import Link from "next/link";
import { useWalletConnection } from "@solana/react-hooks";
import { Button } from "@/components/ui/button";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { Menu, Wallet } from "lucide-react";

const navLinks = [
  { href: "/markets", label: "Markets" },
  { href: "/create", label: "Create" },
];

export function Header() {
  const { connectors, connect, disconnect, wallet, status } =
    useWalletConnection();

  const address = wallet?.account.address.toString();
  const shortAddress = address
    ? `${address.slice(0, 4)}...${address.slice(-4)}`
    : null;

  return (
    <header className="border-b border-border bg-card/80 backdrop-blur-sm sticky top-0 z-50">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 py-3 flex items-center justify-between">
        {/* Logo */}
        <Link
          href="/"
          className="text-xl font-bold text-primary hover:opacity-80 transition"
        >
          Chausar
        </Link>

        {/* Desktop Nav */}
        <nav className="hidden md:flex items-center gap-6">
          {navLinks.map((link) => (
            <Link
              key={link.href}
              href={link.href}
              className="text-sm font-medium text-muted-foreground hover:text-foreground transition"
            >
              {link.label}
            </Link>
          ))}
        </nav>

        {/* Right Side */}
        <div className="flex items-center gap-3">
          {/* Wallet Button */}
          {status === "connected" ? (
            <div className="flex items-center gap-3">
              <span className="text-sm font-mono bg-secondary px-3 py-1.5 rounded-lg hidden sm:inline">
                {shortAddress}
              </span>
              <Button variant="outline" size="sm" onClick={() => disconnect()}>
                Disconnect
              </Button>
            </div>
          ) : (
            <div className="flex gap-2">
              {connectors.slice(0, 2).map((connector) => (
                <Button
                  key={connector.id}
                  variant="default"
                  size="sm"
                  onClick={() => connect(connector.id)}
                  disabled={status === "connecting"}
                >
                  <Wallet className="h-4 w-4 sm:mr-1" />
                  <span className="hidden sm:inline">{connector.name}</span>
                </Button>
              ))}
            </div>
          )}

          {/* Mobile Menu */}
          <Sheet>
            <SheetTrigger asChild>
              <Button variant="ghost" size="icon" className="md:hidden">
                <Menu className="h-5 w-5" />
                <span className="sr-only">Open menu</span>
              </Button>
            </SheetTrigger>
            <SheetContent side="right" className="w-72">
              <SheetHeader>
                <SheetTitle>Menu</SheetTitle>
              </SheetHeader>
              <nav className="flex flex-col gap-4 mt-6">
                {navLinks.map((link) => (
                  <Link
                    key={link.href}
                    href={link.href}
                    className="text-lg font-medium text-foreground hover:text-primary transition py-2"
                  >
                    {link.label}
                  </Link>
                ))}
                {status === "connected" && (
                  <div className="pt-4 border-t border-border">
                    <p className="text-xs text-muted-foreground mb-2">
                      Connected
                    </p>
                    <p className="text-sm font-mono break-all">{address}</p>
                  </div>
                )}
              </nav>
            </SheetContent>
          </Sheet>
        </div>
      </div>
    </header>
  );
}
