# Task: Integration Tests

Metadata:
- Dependencies: task-12 (instruction unit tests)
- Provides: End-to-end flow test coverage
- Size: Small (1 file)

## Implementation Content
Write integration tests for complete user flows:
- Create market -> trade -> resolve -> claim
- Add liquidity -> trade -> remove liquidity

## Target Files
- [ ] `anchor/programs/prediction/tests/integration.rs`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Write full lifecycle test
- [ ] Write liquidity provider flow test

### 2. Green Phase
- [ ] Implement full market lifecycle test:

```rust
#[tokio::test]
async fn test_full_market_lifecycle() {
    // Setup
    let (svm, creator, trader, oracle) = setup_integration_test();

    // 1. Create market
    let market_id = 1u64;
    create_market(
        &svm,
        &creator,
        market_id,
        "Will BTC > 100k by March?".to_string(),
        "Bitcoin price prediction".to_string(),
        end_time,
        resolve_time,
        100_000_000, // 100 USDC
        oracle.pubkey(),
    ).await.unwrap();

    // Verify market created
    let market = get_market(&svm, market_id).await;
    assert_eq!(market.status, MarketStatus::Open);

    // 2. Trade - buy YES tokens
    let usdc_amount = 10_000_000; // 10 USDC
    trade(&svm, &trader, market_id, PoolSide::Yes, usdc_amount, 0).await.unwrap();

    // Verify trader has YES tokens
    let yes_balance = get_token_balance(&svm, &trader, market.yes_mint).await;
    assert!(yes_balance > 0);

    // 3. Lock market (advance time)
    advance_time(&svm, end_time + 1);
    lock_market(&svm, market_id).await.unwrap();

    let market = get_market(&svm, market_id).await;
    assert_eq!(market.status, MarketStatus::Locked);

    // 4. Resolve market
    resolve_market(&svm, &oracle, market_id, MarketResult::Yes).await.unwrap();

    let market = get_market(&svm, market_id).await;
    assert_eq!(market.status, MarketStatus::Resolved);
    assert_eq!(market.result, MarketResult::Yes);

    // 5. Claim winnings
    let initial_usdc = get_usdc_balance(&svm, &trader).await;
    claim_winnings(&svm, &trader, market_id).await.unwrap();

    // Verify payout
    let final_usdc = get_usdc_balance(&svm, &trader).await;
    assert!(final_usdc > initial_usdc);
}

#[tokio::test]
async fn test_liquidity_provider_flow() {
    let (svm, creator, lp, _oracle) = setup_integration_test();

    // Create market
    create_market(...).await.unwrap();

    // Add liquidity
    let lp_usdc = 50_000_000; // 50 USDC
    add_liquidity(&svm, &lp, market_id, PoolSide::Yes, lp_usdc).await.unwrap();

    // Verify LP tokens received
    let lp_balance = get_lp_balance(&svm, &lp, yes_pool).await;
    assert!(lp_balance > 0);

    // Remove liquidity
    remove_liquidity(&svm, &lp, market_id, PoolSide::Yes, lp_balance).await.unwrap();

    // Verify USDC returned (may differ due to trading)
    let final_usdc = get_usdc_balance(&svm, &lp).await;
    assert!(final_usdc > 0);
}
```

### 3. Refactor Phase
- [ ] Add more complex scenarios (multiple traders)
- [ ] Test edge cases in flows
- [ ] Ensure proper cleanup between tests

## Completion Criteria
- [ ] Full lifecycle test passes
- [ ] Liquidity flow test passes
- [ ] Tests verify all state transitions
- [ ] Operation verified: L2 (tests pass)

## Notes
- These tests validate end-to-end behavior
- Use real-ish amounts (100 USDC, etc.)
- Verify balances change correctly
