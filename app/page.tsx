"use client";

import Link from "next/link";
import { Header } from "@/app/components/Header";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { TrendingUp, Zap, Shield } from "lucide-react";

export default function Home() {
  return (
    <div className="min-h-screen bg-background">
      <Header />
      <main className="max-w-6xl mx-auto px-4 sm:px-6 py-12 md:py-20">
        <div className="space-y-16">
          {/* Hero */}
          <div className="text-center space-y-6">
            <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight">
              Decentralized
              <br />
              <span className="text-primary">Prediction Markets</span>
            </h1>
            <p className="text-lg md:text-xl text-muted-foreground max-w-2xl mx-auto leading-relaxed">
              Create binary markets, trade outcomes, and earn on your
              predictions. Fully trustless, on-chain settlement powered by
              Solana.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center pt-4">
              <Button
                asChild
                size="lg"
                className="bg-yes text-white hover:bg-yes/90"
              >
                <Link href="/markets">Start Trading</Link>
              </Button>
              <Button asChild variant="outline" size="lg">
                <Link href="/create">Create Market</Link>
              </Button>
            </div>
          </div>

          {/* Features */}
          <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <Card className="hover:shadow-md hover:border-border-strong transition-all">
              <CardHeader>
                <div className="h-10 w-10 rounded-lg bg-yes-bg flex items-center justify-center mb-2">
                  <TrendingUp className="h-5 w-5 text-yes" />
                </div>
                <CardTitle>Binary Markets</CardTitle>
                <CardDescription>
                  Simple YES/NO predictions. Easy to understand, quick to
                  resolve.
                </CardDescription>
              </CardHeader>
            </Card>

            <Card className="hover:shadow-md hover:border-border-strong transition-all">
              <CardHeader>
                <div className="h-10 w-10 rounded-lg bg-warning/20 flex items-center justify-center mb-2">
                  <Zap className="h-5 w-5 text-warning" />
                </div>
                <CardTitle>Instant Trading</CardTitle>
                <CardDescription>
                  AMM-powered markets with instant execution and transparent
                  pricing.
                </CardDescription>
              </CardHeader>
            </Card>

            <Card className="hover:shadow-md hover:border-border-strong transition-all sm:col-span-2 lg:col-span-1">
              <CardHeader>
                <div className="h-10 w-10 rounded-lg bg-info/20 flex items-center justify-center mb-2">
                  <Shield className="h-5 w-5 text-info" />
                </div>
                <CardTitle>Trustless Settlement</CardTitle>
                <CardDescription>
                  Smart contracts handle payouts automatically. No custody, no
                  counterparty risk.
                </CardDescription>
              </CardHeader>
            </Card>
          </div>

          {/* Stats */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-6 py-8 border-y border-border">
            <div className="text-center">
              <p className="text-3xl md:text-4xl font-bold font-mono text-primary">
                $0
              </p>
              <p className="text-sm text-muted-foreground mt-1">Total Volume</p>
            </div>
            <div className="text-center">
              <p className="text-3xl md:text-4xl font-bold font-mono text-primary">
                0
              </p>
              <p className="text-sm text-muted-foreground mt-1">
                Active Markets
              </p>
            </div>
            <div className="text-center">
              <p className="text-3xl md:text-4xl font-bold font-mono text-primary">
                0
              </p>
              <p className="text-sm text-muted-foreground mt-1">Traders</p>
            </div>
            <div className="text-center">
              <p className="text-3xl md:text-4xl font-bold font-mono text-yes">
                0.3%
              </p>
              <p className="text-sm text-muted-foreground mt-1">Trading Fee</p>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
