import { cn } from "@/lib/utils";

interface PriceDisplayProps {
  value: number;
  side: "yes" | "no";
  format?: "percentage" | "currency";
  size?: "sm" | "md" | "lg";
  showArrow?: boolean;
  previousValue?: number;
  className?: string;
}

export function PriceDisplay({
  value,
  side,
  format = "percentage",
  size = "md",
  showArrow = false,
  previousValue,
  className,
}: PriceDisplayProps) {
  const isYes = side === "yes";

  const sizeClasses = {
    sm: "text-sm px-2 py-1",
    md: "text-lg px-3 py-1.5",
    lg: "text-2xl px-4 py-2",
  };

  const displayValue =
    format === "percentage" ? `${value}%` : `$${value.toFixed(2)}`;

  const trend =
    previousValue !== undefined
      ? value > previousValue
        ? "up"
        : value < previousValue
          ? "down"
          : "flat"
      : null;

  return (
    <div
      className={cn(
        "rounded-md font-mono font-semibold inline-flex items-center gap-1",
        sizeClasses[size],
        isYes ? "bg-yes-bg text-yes" : "bg-no-bg text-no",
        className,
      )}
    >
      {showArrow && trend === "up" && <span>↑</span>}
      {showArrow && trend === "down" && <span>↓</span>}
      <span>{displayValue}</span>
    </div>
  );
}
