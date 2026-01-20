"use client";

import { useState } from "react";
import Link from "next/link";
import { use } from "react";
import { Header } from "@/app/components/Header";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { PriceDisplay } from "@/components/ui/price-display";
import { Separator } from "@/components/ui/separator";
import { ArrowLeft, Clock, TrendingUp, Droplets } from "lucide-react";

interface Market {
  id: number;
  question: string;
  description: string;
  yesPrice: number;
  noPrice: number;
  status: "open" | "locked" | "resolved";
  yesReserve: number;
  noReserve: number;
  totalVolume: number;
  endTime: string;
  resolveTime: string;
}

// Mock market details
const mockMarkets: Record<number, Market> = {
  1: {
    id: 1,
    question: "Will Bitcoin close above $100,000 on January 31, 2026?",
    description:
      "This market resolves YES if Bitcoin's price closes above $100,000 USD on January 31, 2026 at 11:59 PM UTC.",
    yesPrice: 65,
    noPrice: 35,
    status: "open",
    yesReserve: 3250,
    noReserve: 1750,
    totalVolume: 12000,
    endTime: "Jan 31, 2026",
    resolveTime: "Feb 1, 2026",
  },
  2: {
    id: 2,
    question: "Will it rain in San Francisco on February 14, 2026?",
    description:
      "This market resolves YES if there is measurable precipitation in San Francisco on February 14, 2026.",
    yesPrice: 42,
    noPrice: 58,
    status: "open",
    yesReserve: 840,
    noReserve: 1160,
    totalVolume: 5000,
    endTime: "Feb 14, 2026",
    resolveTime: "Feb 15, 2026",
  },
  3: {
    id: 3,
    question: "Will the S&P 500 be above 6,000 on March 1, 2026?",
    description:
      "This market resolves YES if the S&P 500 closing price is above 6,000 on March 1, 2026.",
    yesPrice: 72,
    noPrice: 28,
    status: "open",
    yesReserve: 6120,
    noReserve: 2380,
    totalVolume: 18000,
    endTime: "Mar 1, 2026",
    resolveTime: "Mar 2, 2026",
  },
  4: {
    id: 4,
    question: "Will Ethereum reach $5,000 by end of Q1 2026?",
    description:
      "This market resolves YES if Ethereum reaches $5,000 USD at any point by March 31, 2026.",
    yesPrice: 31,
    noPrice: 69,
    status: "locked",
    yesReserve: 465,
    noReserve: 1035,
    totalVolume: 3500,
    endTime: "Mar 31, 2026",
    resolveTime: "Apr 1, 2026",
  },
};

const statusVariant = {
  open: "default" as const,
  locked: "secondary" as const,
  resolved: "outline" as const,
};

export default function MarketDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const resolvedParams = use(params);
  const marketId = parseInt(resolvedParams.id);
  const market = mockMarkets[marketId];
  const [amount, setAmount] = useState("");
  const [selectedSide, setSelectedSide] = useState<"yes" | "no" | null>(null);

  if (!market) {
    return (
      <div className="min-h-screen bg-background">
        <Header />
        <main className="max-w-4xl mx-auto px-4 sm:px-6 py-8">
          <p className="text-lg mb-4">Market not found</p>
          <Button asChild variant="outline">
            <Link href="/markets">
              <ArrowLeft className="mr-2 h-4 w-4" />
              Back to markets
            </Link>
          </Button>
        </main>
      </div>
    );
  }

  const estimatedTokens = amount ? parseFloat(amount) : 0;

  return (
    <div className="min-h-screen bg-background">
      <Header />
      <main className="max-w-4xl mx-auto px-4 sm:px-6 py-8">
        <Button asChild variant="ghost" size="sm" className="mb-6">
          <Link href="/markets">
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to markets
          </Link>
        </Button>

        <div className="grid lg:grid-cols-3 gap-6">
          {/* Left: Market Info */}
          <div className="lg:col-span-2 space-y-6">
            <div>
              <div className="flex items-start gap-3 mb-3">
                <h1 className="text-2xl font-bold flex-1">{market.question}</h1>
                <Badge variant={statusVariant[market.status]}>
                  {market.status.charAt(0).toUpperCase() +
                    market.status.slice(1)}
                </Badge>
              </div>
              <p className="text-muted-foreground">{market.description}</p>
            </div>

            {/* Price Overview */}
            <Card>
              <CardHeader className="pb-4">
                <CardTitle className="text-base">Current Prices</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-2 gap-6">
                  <div className="text-center p-4 rounded-lg bg-yes-bg/50">
                    <p className="text-sm text-muted-foreground mb-2">YES</p>
                    <PriceDisplay
                      value={market.yesPrice}
                      side="yes"
                      size="lg"
                    />
                  </div>
                  <div className="text-center p-4 rounded-lg bg-no-bg/50">
                    <p className="text-sm text-muted-foreground mb-2">NO</p>
                    <PriceDisplay value={market.noPrice} side="no" size="lg" />
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* Pool Statistics */}
            <Card>
              <CardHeader className="pb-4">
                <CardTitle className="text-base">Pool Statistics</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
                  <div>
                    <p className="text-xs text-muted-foreground mb-1 flex items-center gap-1">
                      <TrendingUp className="h-3 w-3" />
                      YES Reserve
                    </p>
                    <p className="text-lg font-mono font-semibold">
                      ${market.yesReserve.toLocaleString()}
                    </p>
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground mb-1 flex items-center gap-1">
                      <TrendingUp className="h-3 w-3" />
                      NO Reserve
                    </p>
                    <p className="text-lg font-mono font-semibold">
                      ${market.noReserve.toLocaleString()}
                    </p>
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground mb-1 flex items-center gap-1">
                      <Droplets className="h-3 w-3" />
                      Total Volume
                    </p>
                    <p className="text-lg font-mono font-semibold">
                      ${market.totalVolume.toLocaleString()}
                    </p>
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground mb-1 flex items-center gap-1">
                      <Clock className="h-3 w-3" />
                      Ends
                    </p>
                    <p className="text-lg font-mono font-semibold">
                      {market.endTime}
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Right: Trading Panel */}
          <div className="lg:col-span-1">
            <Card className="sticky top-20">
              <CardHeader className="pb-4">
                <CardTitle className="text-base">Trade</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-2 gap-2">
                  <Button
                    variant={selectedSide === "yes" ? "default" : "outline"}
                    className={
                      selectedSide === "yes"
                        ? "bg-yes hover:bg-yes/90 text-white"
                        : "hover:bg-yes-bg hover:text-yes"
                    }
                    onClick={() => setSelectedSide("yes")}
                  >
                    YES
                  </Button>
                  <Button
                    variant={selectedSide === "no" ? "default" : "outline"}
                    className={
                      selectedSide === "no"
                        ? "bg-no hover:bg-no/90 text-white"
                        : "hover:bg-no-bg hover:text-no"
                    }
                    onClick={() => setSelectedSide("no")}
                  >
                    NO
                  </Button>
                </div>

                {selectedSide && (
                  <>
                    <div className="p-3 bg-secondary rounded-lg">
                      <p className="text-xs text-muted-foreground mb-1">
                        Current {selectedSide.toUpperCase()} Price
                      </p>
                      <PriceDisplay
                        value={
                          selectedSide === "yes"
                            ? market.yesPrice
                            : market.noPrice
                        }
                        side={selectedSide}
                        size="md"
                      />
                    </div>

                    <div className="space-y-2">
                      <Label htmlFor="amount">Amount (USDC)</Label>
                      <Input
                        id="amount"
                        type="number"
                        placeholder="0.00"
                        value={amount}
                        onChange={(e) => setAmount(e.target.value)}
                        min="0"
                        step="0.01"
                      />
                    </div>

                    {amount && parseFloat(amount) > 0 && (
                      <div className="p-3 bg-secondary rounded-lg space-y-2 text-sm">
                        <div className="flex justify-between">
                          <span className="text-muted-foreground">
                            You&apos;ll get:
                          </span>
                          <span className="font-semibold font-mono">
                            ~{estimatedTokens.toFixed(2)} tokens
                          </span>
                        </div>
                        <div className="flex justify-between">
                          <span className="text-muted-foreground">
                            Price impact:
                          </span>
                          <span className="font-semibold">~0.5%</span>
                        </div>
                        <Separator />
                        <div className="flex justify-between">
                          <span className="text-muted-foreground">Fee:</span>
                          <span className="font-semibold">0.3%</span>
                        </div>
                      </div>
                    )}

                    <Button
                      className={
                        selectedSide === "yes"
                          ? "w-full bg-yes hover:bg-yes/90 text-white"
                          : "w-full bg-no hover:bg-no/90 text-white"
                      }
                      disabled={
                        !amount ||
                        parseFloat(amount) <= 0 ||
                        market.status !== "open"
                      }
                    >
                      {market.status === "open"
                        ? `Buy ${selectedSide.toUpperCase()}`
                        : "Market Closed"}
                    </Button>
                  </>
                )}

                {!selectedSide && (
                  <p className="text-sm text-muted-foreground text-center py-4">
                    Select YES or NO to trade
                  </p>
                )}
              </CardContent>
            </Card>
          </div>
        </div>
      </main>
    </div>
  );
}
