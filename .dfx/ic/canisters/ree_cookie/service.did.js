export const idlFactory = ({ IDL }) => {
  const RejectionCode = IDL.Variant({
    'NoError' : IDL.Null,
    'CanisterError' : IDL.Null,
    'SysTransient' : IDL.Null,
    'DestinationInvalid' : IDL.Null,
    'Unknown' : IDL.Null,
    'SysFatal' : IDL.Null,
    'CanisterReject' : IDL.Null,
  });
  const ExchangeError = IDL.Variant({
    'InvalidSignPsbtArgs' : IDL.Text,
    'InvalidNumeric' : IDL.Null,
    'ParseUtxoRuneBalanceError' : IDL.Text,
    'Overflow' : IDL.Null,
    'InvalidInput' : IDL.Null,
    'PoolAddressNotFound' : IDL.Null,
    'NatConvertError' : IDL.Nat,
    'CookieBalanceInsufficient' : IDL.Nat,
    'GamerAlreadyExist' : IDL.Text,
    'PoolStateExpired' : IDL.Nat64,
    'GamerNotFound' : IDL.Text,
    'GameNotEnd' : IDL.Null,
    'TooSmallFunds' : IDL.Null,
    'LastStateNotFound' : IDL.Null,
    'InvalidRuneId' : IDL.Null,
    'InvalidPool' : IDL.Null,
    'InvalidPsbt' : IDL.Text,
    'PoolAlreadyExists' : IDL.Null,
    'GamerCoolingDown' : IDL.Tuple(IDL.Text, IDL.Nat64),
    'InvalidTxid' : IDL.Text,
    'InvalidLiquidity' : IDL.Null,
    'DepositRuneBalanceIncorrect' : IDL.Tuple(IDL.Text, IDL.Text),
    'EmptyPool' : IDL.Null,
    'RuneIndexerResultError' : IDL.Text,
    'LpNotFound' : IDL.Null,
    'ChainKeyError' : IDL.Null,
    'FetchRuneIndexerError' : IDL.Tuple(RejectionCode, IDL.Text),
    'CustomError' : IDL.Text,
    'InvalidState' : IDL.Text,
    'InsufficientFunds' : IDL.Null,
    'GamerWithdrawRepeatedly' : IDL.Text,
    'RuneIdNotMatch' : IDL.Tuple(IDL.Text, IDL.Text),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : ExchangeError });
  const CoinBalance = IDL.Record({ 'id' : IDL.Text, 'value' : IDL.Nat });
  const Utxo = IDL.Record({
    'maybe_rune' : IDL.Opt(CoinBalance),
    'sats' : IDL.Nat64,
    'txid' : IDL.Text,
    'vout' : IDL.Nat32,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : ExchangeError });
  const InputCoin = IDL.Record({ 'coin' : CoinBalance, 'from' : IDL.Text });
  const OutputCoin = IDL.Record({ 'to' : IDL.Text, 'coin' : CoinBalance });
  const Intention = IDL.Record({
    'input_coins' : IDL.Vec(InputCoin),
    'output_coins' : IDL.Vec(OutputCoin),
    'action' : IDL.Text,
    'exchange_id' : IDL.Text,
    'pool_utxo_spend' : IDL.Vec(IDL.Text),
    'action_params' : IDL.Text,
    'nonce' : IDL.Nat64,
    'pool_utxo_receive' : IDL.Vec(IDL.Text),
    'pool_address' : IDL.Text,
  });
  const IntentionSet = IDL.Record({
    'initiator_address' : IDL.Text,
    'intentions' : IDL.Vec(Intention),
  });
  const ExecuteTxArgs = IDL.Record({
    'zero_confirmed_tx_queue_length' : IDL.Nat32,
    'txid' : IDL.Text,
    'intention_set' : IntentionSet,
    'intention_index' : IDL.Nat32,
    'psbt_hex' : IDL.Text,
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const FinalizeTxArgs = IDL.Record({
    'txid' : IDL.Text,
    'pool_key' : IDL.Text,
  });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const GetMinimalTxValueArgs = IDL.Record({
    'zero_confirmed_tx_queue_length' : IDL.Nat32,
    'pool_address' : IDL.Text,
  });
  const GetPoolInfoArgs = IDL.Record({ 'pool_address' : IDL.Text });
  const PoolInfo = IDL.Record({
    'key' : IDL.Text,
    'name' : IDL.Text,
    'btc_reserved' : IDL.Nat64,
    'key_derivation_path' : IDL.Vec(IDL.Vec(IDL.Nat8)),
    'coin_reserved' : IDL.Vec(CoinBalance),
    'attributes' : IDL.Text,
    'address' : IDL.Text,
    'nonce' : IDL.Nat64,
    'utxos' : IDL.Vec(Utxo),
  });
  const GetPoolListArgs = IDL.Record({
    'from' : IDL.Opt(IDL.Text),
    'limit' : IDL.Nat32,
  });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : ExchangeError });
  const RegisterInfo = IDL.Record({
    'tweaked_key' : IDL.Text,
    'utxo' : Utxo,
    'untweaked_key' : IDL.Text,
    'address' : IDL.Text,
    'nonce' : IDL.Nat64,
    'register_fee' : IDL.Nat64,
  });
  return IDL.Service({
    'claim' : IDL.Func([], [Result], []),
    'deposit' : IDL.Func([Utxo, Utxo], [Result_1], []),
    'execute_tx' : IDL.Func([ExecuteTxArgs], [Result_2], []),
    'finalize_tx' : IDL.Func([FinalizeTxArgs], [Result_3], []),
    'get_minimal_tx_value' : IDL.Func(
        [GetMinimalTxValueArgs],
        [IDL.Nat64],
        ['query'],
      ),
    'get_pool_info' : IDL.Func([GetPoolInfoArgs], [IDL.Opt(PoolInfo)], []),
    'get_pool_list' : IDL.Func(
        [GetPoolListArgs],
        [IDL.Vec(PoolInfo)],
        ['query'],
      ),
    'init_key' : IDL.Func([], [Result_4], []),
    'query_register_info' : IDL.Func([], [RegisterInfo], ['query']),
    'query_rune_deposit_address' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'rollback_tx' : IDL.Func([FinalizeTxArgs], [Result_3], []),
  });
};
export const init = ({ IDL }) => {
  return [
    IDL.Nat64,
    IDL.Nat32,
    IDL.Text,
    IDL.Nat64,
    IDL.Nat64,
    IDL.Nat64,
    IDL.Nat,
    IDL.Nat,
    IDL.Principal,
    IDL.Principal,
  ];
};
