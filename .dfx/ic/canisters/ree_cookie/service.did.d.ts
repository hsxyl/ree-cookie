import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface CoinBalance { 'id' : string, 'value' : bigint }
export type ExchangeError = { 'InvalidSignPsbtArgs' : string } |
  { 'InvalidNumeric' : null } |
  { 'ParseUtxoRuneBalanceError' : string } |
  { 'Overflow' : null } |
  { 'InvalidInput' : null } |
  { 'PoolAddressNotFound' : null } |
  { 'NatConvertError' : bigint } |
  { 'CookieBalanceInsufficient' : bigint } |
  { 'GamerAlreadyExist' : string } |
  { 'PoolStateExpired' : bigint } |
  { 'GamerNotFound' : string } |
  { 'GameNotEnd' : null } |
  { 'TooSmallFunds' : null } |
  { 'LastStateNotFound' : null } |
  { 'InvalidRuneId' : null } |
  { 'InvalidPool' : null } |
  { 'InvalidPsbt' : string } |
  { 'PoolAlreadyExists' : null } |
  { 'GamerCoolingDown' : [string, bigint] } |
  { 'InvalidTxid' : string } |
  { 'InvalidLiquidity' : null } |
  { 'DepositRuneBalanceIncorrect' : [string, string] } |
  { 'EmptyPool' : null } |
  { 'RuneIndexerResultError' : string } |
  { 'LpNotFound' : null } |
  { 'ChainKeyError' : null } |
  { 'FetchRuneIndexerError' : [RejectionCode, string] } |
  { 'CustomError' : string } |
  { 'InvalidState' : string } |
  { 'InsufficientFunds' : null } |
  { 'GamerWithdrawRepeatedly' : string } |
  { 'RuneIdNotMatch' : [string, string] };
export interface ExecuteTxArgs {
  'zero_confirmed_tx_queue_length' : number,
  'txid' : string,
  'intention_set' : IntentionSet,
  'intention_index' : number,
  'psbt_hex' : string,
}
export interface FinalizeTxArgs { 'txid' : string, 'pool_key' : string }
export interface GetMinimalTxValueArgs {
  'zero_confirmed_tx_queue_length' : number,
  'pool_address' : string,
}
export interface GetPoolInfoArgs { 'pool_address' : string }
export interface GetPoolListArgs { 'from' : [] | [string], 'limit' : number }
export interface InputCoin { 'coin' : CoinBalance, 'from' : string }
export interface Intention {
  'input_coins' : Array<InputCoin>,
  'output_coins' : Array<OutputCoin>,
  'action' : string,
  'exchange_id' : string,
  'pool_utxo_spend' : Array<string>,
  'action_params' : string,
  'nonce' : bigint,
  'pool_utxo_receive' : Array<string>,
  'pool_address' : string,
}
export interface IntentionSet {
  'initiator_address' : string,
  'intentions' : Array<Intention>,
}
export interface OutputCoin { 'to' : string, 'coin' : CoinBalance }
export interface PoolInfo {
  'key' : string,
  'name' : string,
  'btc_reserved' : bigint,
  'key_derivation_path' : Array<Uint8Array | number[]>,
  'coin_reserved' : Array<CoinBalance>,
  'attributes' : string,
  'address' : string,
  'nonce' : bigint,
  'utxos' : Array<Utxo>,
}
export interface RegisterInfo {
  'tweaked_key' : string,
  'utxo' : Utxo,
  'untweaked_key' : string,
  'address' : string,
  'nonce' : bigint,
  'register_fee' : bigint,
}
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export type Result = { 'Ok' : bigint } |
  { 'Err' : ExchangeError };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : ExchangeError };
export type Result_2 = { 'Ok' : string } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : string } |
  { 'Err' : ExchangeError };
export interface Utxo {
  'maybe_rune' : [] | [CoinBalance],
  'sats' : bigint,
  'txid' : string,
  'vout' : number,
}
export interface _SERVICE {
  'claim' : ActorMethod<[], Result>,
  'deposit' : ActorMethod<[Utxo, Utxo], Result_1>,
  'execute_tx' : ActorMethod<[ExecuteTxArgs], Result_2>,
  'finalize_tx' : ActorMethod<[FinalizeTxArgs], Result_3>,
  'get_minimal_tx_value' : ActorMethod<[GetMinimalTxValueArgs], bigint>,
  'get_pool_info' : ActorMethod<[GetPoolInfoArgs], [] | [PoolInfo]>,
  'get_pool_list' : ActorMethod<[GetPoolListArgs], Array<PoolInfo>>,
  'init_key' : ActorMethod<[], Result_4>,
  'query_register_info' : ActorMethod<[], RegisterInfo>,
  'query_rune_deposit_address' : ActorMethod<[], [] | [string]>,
  'rollback_tx' : ActorMethod<[FinalizeTxArgs], Result_3>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
