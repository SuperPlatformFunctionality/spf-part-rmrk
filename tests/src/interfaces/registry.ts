// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type {
  FinalityGrandpaEquivocationPrecommit,
  FinalityGrandpaEquivocationPrevote,
  FinalityGrandpaPrecommit,
  FinalityGrandpaPrevote,
  FrameSupportDispatchRawOrigin,
  FrameSupportTokensMiscBalanceStatus,
  FrameSupportWeightsDispatchClass,
  FrameSupportWeightsDispatchInfo,
  FrameSupportWeightsPays,
  FrameSupportWeightsPerDispatchClassU32,
  FrameSupportWeightsPerDispatchClassU64,
  FrameSupportWeightsPerDispatchClassWeightsPerClass,
  FrameSupportWeightsRuntimeDbWeight,
  FrameSystemAccountInfo,
  FrameSystemCall,
  FrameSystemError,
  FrameSystemEvent,
  FrameSystemEventRecord,
  FrameSystemExtensionsCheckGenesis,
  FrameSystemExtensionsCheckNonce,
  FrameSystemExtensionsCheckSpecVersion,
  FrameSystemExtensionsCheckTxVersion,
  FrameSystemExtensionsCheckWeight,
  FrameSystemLastRuntimeUpgradeInfo,
  FrameSystemLimitsBlockLength,
  FrameSystemLimitsBlockWeights,
  FrameSystemLimitsWeightsPerClass,
  FrameSystemPhase,
  PalletBalancesAccountData,
  PalletBalancesBalanceLock,
  PalletBalancesCall,
  PalletBalancesError,
  PalletBalancesEvent,
  PalletBalancesReasons,
  PalletBalancesReleases,
  PalletBalancesReserveData,
  PalletGrandpaCall,
  PalletGrandpaError,
  PalletGrandpaEvent,
  PalletGrandpaStoredPendingChange,
  PalletGrandpaStoredState,
  PalletRmrkCoreCall,
  PalletRmrkCoreError,
  PalletRmrkCoreEvent,
  PalletRmrkEquipCall,
  PalletRmrkEquipError,
  PalletRmrkEquipEvent,
  PalletRmrkMarketCall,
  PalletRmrkMarketError,
  PalletRmrkMarketEvent,
  PalletRmrkMarketListInfo,
  PalletRmrkMarketOffer,
  PalletSudoCall,
  PalletSudoError,
  PalletSudoEvent,
  PalletTemplateCall,
  PalletTemplateError,
  PalletTemplateEvent,
  PalletTimestampCall,
  PalletTransactionPaymentChargeTransactionPayment,
  PalletTransactionPaymentEvent,
  PalletTransactionPaymentReleases,
  PalletUniquesCall,
  PalletUniquesCollectionDetails,
  PalletUniquesCollectionMetadata,
  PalletUniquesDestroyWitness,
  PalletUniquesError,
  PalletUniquesEvent,
  PalletUniquesItemDetails,
  PalletUniquesItemMetadata,
  PalletUtilityCall,
  PalletUtilityError,
  PalletUtilityEvent,
  PhantomTypePhantomType,
  RmrkSubstrateRuntimeOriginCaller,
  RmrkSubstrateRuntimeRuntime,
  RmrkTraitsBaseBaseInfo,
  RmrkTraitsCollectionCollectionInfo,
  RmrkTraitsNftAccountIdOrCollectionNftTuple,
  RmrkTraitsNftNftChild,
  RmrkTraitsNftNftInfo,
  RmrkTraitsNftRoyaltyInfo,
  RmrkTraitsPartEquippableList,
  RmrkTraitsPartFixedPart,
  RmrkTraitsPartPartType,
  RmrkTraitsPartSlotPart,
  RmrkTraitsPropertyPropertyInfo,
  RmrkTraitsResourceBasicResource,
  RmrkTraitsResourceComposableResource,
  RmrkTraitsResourceResourceInfo,
  RmrkTraitsResourceResourceTypes,
  RmrkTraitsResourceSlotResource,
  RmrkTraitsTheme,
  RmrkTraitsThemeThemeProperty,
  SpConsensusAuraSr25519AppSr25519Public,
  SpCoreEcdsaSignature,
  SpCoreEd25519Public,
  SpCoreEd25519Signature,
  SpCoreSr25519Public,
  SpCoreSr25519Signature,
  SpCoreVoid,
  SpFinalityGrandpaAppPublic,
  SpFinalityGrandpaAppSignature,
  SpFinalityGrandpaEquivocation,
  SpFinalityGrandpaEquivocationProof,
  SpRuntimeArithmeticError,
  SpRuntimeDigest,
  SpRuntimeDigestDigestItem,
  SpRuntimeDispatchError,
  SpRuntimeModuleError,
  SpRuntimeMultiSignature,
  SpRuntimeTokenError,
  SpRuntimeTransactionalError,
  SpVersionRuntimeVersion,
} from "@polkadot/types/lookup";

declare module "@polkadot/types/types/registry" {
  export interface InterfaceTypes {
    FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
    FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
    FinalityGrandpaPrecommit: FinalityGrandpaPrecommit;
    FinalityGrandpaPrevote: FinalityGrandpaPrevote;
    FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
    FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
    FrameSupportWeightsDispatchClass: FrameSupportWeightsDispatchClass;
    FrameSupportWeightsDispatchInfo: FrameSupportWeightsDispatchInfo;
    FrameSupportWeightsPays: FrameSupportWeightsPays;
    FrameSupportWeightsPerDispatchClassU32: FrameSupportWeightsPerDispatchClassU32;
    FrameSupportWeightsPerDispatchClassU64: FrameSupportWeightsPerDispatchClassU64;
    FrameSupportWeightsPerDispatchClassWeightsPerClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
    FrameSupportWeightsRuntimeDbWeight: FrameSupportWeightsRuntimeDbWeight;
    FrameSystemAccountInfo: FrameSystemAccountInfo;
    FrameSystemCall: FrameSystemCall;
    FrameSystemError: FrameSystemError;
    FrameSystemEvent: FrameSystemEvent;
    FrameSystemEventRecord: FrameSystemEventRecord;
    FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
    FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
    FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
    FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
    FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
    FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
    FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
    FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
    FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
    FrameSystemPhase: FrameSystemPhase;
    PalletBalancesAccountData: PalletBalancesAccountData;
    PalletBalancesBalanceLock: PalletBalancesBalanceLock;
    PalletBalancesCall: PalletBalancesCall;
    PalletBalancesError: PalletBalancesError;
    PalletBalancesEvent: PalletBalancesEvent;
    PalletBalancesReasons: PalletBalancesReasons;
    PalletBalancesReleases: PalletBalancesReleases;
    PalletBalancesReserveData: PalletBalancesReserveData;
    PalletGrandpaCall: PalletGrandpaCall;
    PalletGrandpaError: PalletGrandpaError;
    PalletGrandpaEvent: PalletGrandpaEvent;
    PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
    PalletGrandpaStoredState: PalletGrandpaStoredState;
    PalletRmrkCoreCall: PalletRmrkCoreCall;
    PalletRmrkCoreError: PalletRmrkCoreError;
    PalletRmrkCoreEvent: PalletRmrkCoreEvent;
    PalletRmrkEquipCall: PalletRmrkEquipCall;
    PalletRmrkEquipError: PalletRmrkEquipError;
    PalletRmrkEquipEvent: PalletRmrkEquipEvent;
    PalletRmrkMarketCall: PalletRmrkMarketCall;
    PalletRmrkMarketError: PalletRmrkMarketError;
    PalletRmrkMarketEvent: PalletRmrkMarketEvent;
    PalletRmrkMarketListInfo: PalletRmrkMarketListInfo;
    PalletRmrkMarketOffer: PalletRmrkMarketOffer;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTemplateCall: PalletTemplateCall;
    PalletTemplateError: PalletTemplateError;
    PalletTemplateEvent: PalletTemplateEvent;
    PalletTimestampCall: PalletTimestampCall;
    PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
    PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
    PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
    PalletUniquesCall: PalletUniquesCall;
    PalletUniquesCollectionDetails: PalletUniquesCollectionDetails;
    PalletUniquesCollectionMetadata: PalletUniquesCollectionMetadata;
    PalletUniquesDestroyWitness: PalletUniquesDestroyWitness;
    PalletUniquesError: PalletUniquesError;
    PalletUniquesEvent: PalletUniquesEvent;
    PalletUniquesItemDetails: PalletUniquesItemDetails;
    PalletUniquesItemMetadata: PalletUniquesItemMetadata;
    PalletUtilityCall: PalletUtilityCall;
    PalletUtilityError: PalletUtilityError;
    PalletUtilityEvent: PalletUtilityEvent;
    PhantomTypePhantomType: PhantomTypePhantomType;
    RmrkSubstrateRuntimeOriginCaller: RmrkSubstrateRuntimeOriginCaller;
    RmrkSubstrateRuntimeRuntime: RmrkSubstrateRuntimeRuntime;
    RmrkTraitsBaseBaseInfo: RmrkTraitsBaseBaseInfo;
    RmrkTraitsCollectionCollectionInfo: RmrkTraitsCollectionCollectionInfo;
    RmrkTraitsNftAccountIdOrCollectionNftTuple: RmrkTraitsNftAccountIdOrCollectionNftTuple;
    RmrkTraitsNftNftChild: RmrkTraitsNftNftChild;
    RmrkTraitsNftNftInfo: RmrkTraitsNftNftInfo;
    RmrkTraitsNftRoyaltyInfo: RmrkTraitsNftRoyaltyInfo;
    RmrkTraitsPartEquippableList: RmrkTraitsPartEquippableList;
    RmrkTraitsPartFixedPart: RmrkTraitsPartFixedPart;
    RmrkTraitsPartPartType: RmrkTraitsPartPartType;
    RmrkTraitsPartSlotPart: RmrkTraitsPartSlotPart;
    RmrkTraitsPropertyPropertyInfo: RmrkTraitsPropertyPropertyInfo;
    RmrkTraitsResourceBasicResource: RmrkTraitsResourceBasicResource;
    RmrkTraitsResourceComposableResource: RmrkTraitsResourceComposableResource;
    RmrkTraitsResourceResourceInfo: RmrkTraitsResourceResourceInfo;
    RmrkTraitsResourceResourceTypes: RmrkTraitsResourceResourceTypes;
    RmrkTraitsResourceSlotResource: RmrkTraitsResourceSlotResource;
    RmrkTraitsTheme: RmrkTraitsTheme;
    RmrkTraitsThemeThemeProperty: RmrkTraitsThemeThemeProperty;
    SpConsensusAuraSr25519AppSr25519Public: SpConsensusAuraSr25519AppSr25519Public;
    SpCoreEcdsaSignature: SpCoreEcdsaSignature;
    SpCoreEd25519Public: SpCoreEd25519Public;
    SpCoreEd25519Signature: SpCoreEd25519Signature;
    SpCoreSr25519Public: SpCoreSr25519Public;
    SpCoreSr25519Signature: SpCoreSr25519Signature;
    SpCoreVoid: SpCoreVoid;
    SpFinalityGrandpaAppPublic: SpFinalityGrandpaAppPublic;
    SpFinalityGrandpaAppSignature: SpFinalityGrandpaAppSignature;
    SpFinalityGrandpaEquivocation: SpFinalityGrandpaEquivocation;
    SpFinalityGrandpaEquivocationProof: SpFinalityGrandpaEquivocationProof;
    SpRuntimeArithmeticError: SpRuntimeArithmeticError;
    SpRuntimeDigest: SpRuntimeDigest;
    SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
    SpRuntimeDispatchError: SpRuntimeDispatchError;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpRuntimeTransactionalError: SpRuntimeTransactionalError;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
  } // InterfaceTypes
} // declare module
