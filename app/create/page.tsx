"use client";

import { useState } from "react";
import Link from "next/link";
import { Header } from "@/app/components/Header";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { ArrowLeft, Check, AlertCircle } from "lucide-react";

export default function CreateMarketPage() {
  const [formData, setFormData] = useState({
    question: "",
    description: "",
    endTime: "",
    resolveTime: "",
    liquidity: "",
  });

  const [submitted, setSubmitted] = useState(false);

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (
      formData.question &&
      formData.endTime &&
      formData.resolveTime &&
      formData.liquidity
    ) {
      setSubmitted(true);
      setTimeout(() => {
        setFormData({
          question: "",
          description: "",
          endTime: "",
          resolveTime: "",
          liquidity: "",
        });
        setSubmitted(false);
      }, 3000);
    }
  };

  const isFormValid =
    formData.question &&
    formData.endTime &&
    formData.resolveTime &&
    formData.liquidity &&
    parseFloat(formData.liquidity) >= 100;

  return (
    <div className="min-h-screen bg-background">
      <Header />
      <main className="max-w-2xl mx-auto px-4 sm:px-6 py-8">
        <Button asChild variant="ghost" size="sm" className="mb-6">
          <Link href="/markets">
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to markets
          </Link>
        </Button>

        <div className="mb-8">
          <h1 className="text-3xl font-bold mb-2">Create Market</h1>
          <p className="text-muted-foreground">
            Create a new binary prediction market. You&apos;ll provide initial
            liquidity to bootstrap the market.
          </p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Success Message */}
          {submitted && (
            <Card className="border-yes bg-yes-bg">
              <CardContent className="flex items-center gap-3 py-4">
                <div className="h-8 w-8 rounded-full bg-yes/20 flex items-center justify-center">
                  <Check className="h-4 w-4 text-yes" />
                </div>
                <div>
                  <p className="font-semibold text-yes">
                    Market created successfully!
                  </p>
                  <p className="text-sm text-muted-foreground">
                    (Mock - not deployed to chain)
                  </p>
                </div>
              </CardContent>
            </Card>
          )}

          {/* Market Details Section */}
          <Card>
            <CardHeader className="pb-4">
              <CardTitle className="text-base">Market Details</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="question">
                  Question <span className="text-destructive">*</span>
                </Label>
                <textarea
                  id="question"
                  name="question"
                  placeholder="e.g., Will Bitcoin close above $100,000 on January 31, 2026?"
                  value={formData.question}
                  onChange={handleChange}
                  rows={3}
                  required
                  minLength={10}
                  maxLength={280}
                  className="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 resize-none"
                />
                <p className="text-xs text-muted-foreground">
                  {formData.question.length}/280 characters
                </p>
              </div>

              <div className="space-y-2">
                <Label htmlFor="description">Description</Label>
                <textarea
                  id="description"
                  name="description"
                  placeholder="Additional context or resolution criteria..."
                  value={formData.description}
                  onChange={handleChange}
                  rows={3}
                  maxLength={1000}
                  className="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 resize-none"
                />
                <p className="text-xs text-muted-foreground">
                  Optional - {formData.description.length}/1000 characters
                </p>
              </div>
            </CardContent>
          </Card>

          {/* Timing Section */}
          <Card>
            <CardHeader className="pb-4">
              <CardTitle className="text-base">Timing</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid sm:grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="endTime">
                    Trading Ends <span className="text-destructive">*</span>
                  </Label>
                  <Input
                    id="endTime"
                    name="endTime"
                    type="datetime-local"
                    value={formData.endTime}
                    onChange={handleChange}
                    required
                  />
                  <p className="text-xs text-muted-foreground">
                    When trading closes
                  </p>
                </div>
                <div className="space-y-2">
                  <Label htmlFor="resolveTime">
                    Resolution Time <span className="text-destructive">*</span>
                  </Label>
                  <Input
                    id="resolveTime"
                    name="resolveTime"
                    type="datetime-local"
                    value={formData.resolveTime}
                    onChange={handleChange}
                    required
                  />
                  <p className="text-xs text-muted-foreground">
                    When outcome is determined
                  </p>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Liquidity Section */}
          <Card>
            <CardHeader className="pb-4">
              <CardTitle className="text-base">Initial Liquidity</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="liquidity">
                  Amount (USDC) <span className="text-destructive">*</span>
                </Label>
                <Input
                  id="liquidity"
                  name="liquidity"
                  type="number"
                  placeholder="100"
                  value={formData.liquidity}
                  onChange={handleChange}
                  min="100"
                  step="0.01"
                  required
                />
                <p className="text-xs text-muted-foreground">
                  Minimum 100 USDC required
                </p>
              </div>

              {formData.liquidity && parseFloat(formData.liquidity) < 100 && (
                <div className="flex items-center gap-2 p-3 rounded-lg bg-destructive/10 text-destructive text-sm">
                  <AlertCircle className="h-4 w-4 shrink-0" />
                  <span>Minimum liquidity is 100 USDC</span>
                </div>
              )}
            </CardContent>
          </Card>

          {/* Summary */}
          <Card className="bg-secondary/50">
            <CardHeader className="pb-4">
              <CardTitle className="text-base">Market Summary</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3 text-sm">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Question:</span>
                <span className="font-medium text-right max-w-[60%] truncate">
                  {formData.question || "(Not filled)"}
                </span>
              </div>
              <Separator />
              <div className="flex justify-between">
                <span className="text-muted-foreground">Trading Ends:</span>
                <span className="font-medium">
                  {formData.endTime
                    ? new Date(formData.endTime).toLocaleString()
                    : "(Not filled)"}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Resolves:</span>
                <span className="font-medium">
                  {formData.resolveTime
                    ? new Date(formData.resolveTime).toLocaleString()
                    : "(Not filled)"}
                </span>
              </div>
              <Separator />
              <div className="flex justify-between">
                <span className="text-muted-foreground">
                  Initial Liquidity:
                </span>
                <span className="font-mono font-semibold">
                  ${formData.liquidity || "0"} USDC
                </span>
              </div>
            </CardContent>
          </Card>

          {/* Actions */}
          <div className="flex flex-col sm:flex-row gap-3">
            <Button
              type="submit"
              className="flex-1 bg-yes hover:bg-yes/90 text-white"
              disabled={!isFormValid || submitted}
            >
              {submitted ? "Creating..." : "Create Market"}
            </Button>
            <Button asChild variant="outline" className="flex-1">
              <Link href="/markets">Cancel</Link>
            </Button>
          </div>
        </form>
      </main>
    </div>
  );
}
