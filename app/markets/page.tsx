"use client";

import Link from "next/link";
import { Header } from "@/app/components/Header";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { PriceDisplay } from "@/components/ui/price-display";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Clock, Droplets, ArrowRight } from "lucide-react";

// Mock market data
const mockMarkets = [
  {
    id: 1,
    question: "Will Bitcoin close above $100,000 on January 31, 2026?",
    yesPrice: 65,
    noPrice: 35,
    timeLeft: "3 days",
    status: "open" as const,
    liquidity: 5000,
  },
  {
    id: 2,
    question: "Will it rain in San Francisco on February 14, 2026?",
    yesPrice: 42,
    noPrice: 58,
    timeLeft: "10 days",
    status: "open" as const,
    liquidity: 2000,
  },
  {
    id: 3,
    question: "Will the S&P 500 be above 6,000 on March 1, 2026?",
    yesPrice: 72,
    noPrice: 28,
    timeLeft: "25 days",
    status: "open" as const,
    liquidity: 8500,
  },
  {
    id: 4,
    question: "Will Ethereum reach $5,000 by end of Q1 2026?",
    yesPrice: 31,
    noPrice: 69,
    timeLeft: "Ended",
    status: "locked" as const,
    liquidity: 1500,
  },
];

const statusVariant = {
  open: "default" as const,
  locked: "secondary" as const,
  resolved: "outline" as const,
};

export default function MarketsPage() {
  return (
    <div className="min-h-screen bg-background">
      <Header />
      <main className="max-w-6xl mx-auto px-4 sm:px-6 py-8">
        <div className="flex items-center justify-between mb-8">
          <h1 className="text-3xl font-bold">Prediction Markets</h1>
          <Badge variant="outline" className="hidden sm:flex">
            {mockMarkets.length} Markets
          </Badge>
        </div>

        {/* Desktop Table View */}
        <div className="hidden md:block rounded-xl border border-border overflow-hidden">
          <Table>
            <TableHeader>
              <TableRow className="bg-secondary/50">
                <TableHead className="w-[40%]">Question</TableHead>
                <TableHead className="text-center">YES</TableHead>
                <TableHead className="text-center">NO</TableHead>
                <TableHead className="text-center">Time Left</TableHead>
                <TableHead className="text-center">Status</TableHead>
                <TableHead className="text-right">Liquidity</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {mockMarkets.map((market) => (
                <TableRow
                  key={market.id}
                  className="cursor-pointer hover:bg-secondary/30 transition"
                >
                  <TableCell className="font-medium">
                    <Link
                      href={`/markets/${market.id}`}
                      className="hover:text-primary transition block"
                    >
                      {market.question}
                    </Link>
                  </TableCell>
                  <TableCell className="text-center">
                    <PriceDisplay
                      value={market.yesPrice}
                      side="yes"
                      size="sm"
                    />
                  </TableCell>
                  <TableCell className="text-center">
                    <PriceDisplay value={market.noPrice} side="no" size="sm" />
                  </TableCell>
                  <TableCell className="text-center text-sm text-muted-foreground">
                    {market.timeLeft}
                  </TableCell>
                  <TableCell className="text-center">
                    <Badge variant={statusVariant[market.status]}>
                      {market.status.charAt(0).toUpperCase() +
                        market.status.slice(1)}
                    </Badge>
                  </TableCell>
                  <TableCell className="text-right font-mono text-sm">
                    ${market.liquidity.toLocaleString()}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>

        {/* Mobile Card View */}
        <div className="md:hidden space-y-4">
          {mockMarkets.map((market) => (
            <Link key={market.id} href={`/markets/${market.id}`}>
              <Card className="hover:shadow-md hover:border-border-strong transition-all cursor-pointer">
                <CardHeader className="pb-3">
                  <div className="flex items-start justify-between gap-2">
                    <CardTitle className="text-base leading-snug">
                      {market.question}
                    </CardTitle>
                    <Badge
                      variant={statusVariant[market.status]}
                      className="shrink-0"
                    >
                      {market.status.charAt(0).toUpperCase() +
                        market.status.slice(1)}
                    </Badge>
                  </div>
                </CardHeader>
                <CardContent className="pt-0">
                  <div className="grid grid-cols-2 gap-3 mb-4">
                    <div>
                      <p className="text-xs text-muted-foreground mb-1">YES</p>
                      <PriceDisplay
                        value={market.yesPrice}
                        side="yes"
                        size="md"
                      />
                    </div>
                    <div>
                      <p className="text-xs text-muted-foreground mb-1">NO</p>
                      <PriceDisplay
                        value={market.noPrice}
                        side="no"
                        size="md"
                      />
                    </div>
                  </div>
                  <div className="flex items-center justify-between text-sm text-muted-foreground border-t border-border pt-3">
                    <span className="flex items-center gap-1">
                      <Clock className="h-3.5 w-3.5" />
                      {market.timeLeft}
                    </span>
                    <span className="flex items-center gap-1">
                      <Droplets className="h-3.5 w-3.5" />$
                      {market.liquidity.toLocaleString()}
                    </span>
                    <ArrowRight className="h-4 w-4" />
                  </div>
                </CardContent>
              </Card>
            </Link>
          ))}
        </div>
      </main>
    </div>
  );
}
