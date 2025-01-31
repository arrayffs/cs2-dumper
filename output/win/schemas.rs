// Generated using https://github.com/a2x/cs2-dumper
// 2024-03-27 13:29:16.446880100 UTC

pub mod cs2_dumper {
    pub mod schemas {
        // Module: animationsystem.dll
        // Classes count: 253
        // Enums count: 70
        pub mod animationsystem {
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum MoodTypeT {
                EMoodTypeHead = 0,
                EMoodTypeBody = 1
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum AnimationProcessingTypeT {
                AnimationProcessingServerSimulation = 0,
                AnimationProcessingClientSimulation = 1,
                AnimationProcessingClientPrediction = 2,
                AnimationProcessingClientInterpolation = 3,
                AnimationProcessingClientRender = 4,
                AnimationProcessingMax = 5
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum AnimationSnapshotTypeT {
                AnimationSnapshotServerSimulation = 0,
                AnimationSnapshotClientSimulation = 1,
                AnimationSnapshotClientPrediction = 2,
                AnimationSnapshotClientInterpolation = 3,
                AnimationSnapshotClientRender = 4,
                AnimationSnapshotFinalComposite = 5,
                AnimationSnapshotMax = 6
            }
            // Alignment: 4
            // Members count: 17
            #[repr(i32)]
            pub enum SeqCmdT {
                SeqCmdNop = 0,
                SeqCmdLinearDelta = 1,
                SeqCmdFetchFrameRange = 2,
                SeqCmdSlerp = 3,
                SeqCmdAdd = 4,
                SeqCmdSubtract = 5,
                SeqCmdScale = 6,
                SeqCmdCopy = 7,
                SeqCmdBlend = 8,
                SeqCmdWorldspace = 9,
                SeqCmdSequence = 10,
                SeqCmdFetchCycle = 11,
                SeqCmdFetchFrame = 12,
                SeqCmdIkLockInPlace = 13,
                SeqCmdIkRestoreAll = 14,
                SeqCmdReverseSequence = 15,
                SeqCmdTransform = 16
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum SeqPoseSettingT {
                SeqPoseSettingConstant = 0,
                SeqPoseSettingRotation = 1,
                SeqPoseSettingPosition = 2,
                SeqPoseSettingVelocity = 3
            }
            // Alignment: 4
            // Members count: 18
            #[repr(i32)]
            pub enum ParticleAttachmentT {
                PattachInvalid = -1,
                PattachAbsorigin = 0,
                PattachAbsoriginFollow = 1,
                PattachCustomorigin = 2,
                PattachCustomoriginFollow = 3,
                PattachPoint = 4,
                PattachPointFollow = 5,
                PattachEyesFollow = 6,
                PattachOverheadFollow = 7,
                PattachWorldorigin = 8,
                PattachRootboneFollow = 9,
                PattachRenderoriginFollow = 10,
                PattachMainView = 11,
                PattachWaterwake = 12,
                PattachCenterFollow = 13,
                PattachCustomGameState1 = 14,
                PattachHealthbar = 15,
                MaxPattachTypes = 16
            }
            // Alignment: 1
            // Members count: 9
            #[repr(i8)]
            pub enum AnimParamTypeT {
                AnimparamUnknown = 0,
                AnimparamBool = 1,
                AnimparamEnum = 2,
                AnimparamInt = 3,
                AnimparamFloat = 4,
                AnimparamVector = 5,
                AnimparamQuaternion = 6,
                AnimparamGlobalsymbol = 7,
                AnimparamCount = 8
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum BoneTransformSpaceT {
                BoneTransformSpaceInvalid = -1,
                BoneTransformSpaceParent = 0,
                BoneTransformSpaceModel = 1,
                BoneTransformSpaceWorld = 2
            }
            // Alignment: 4
            // Members count: 13
            #[repr(i32)]
            pub enum AnimParamButtonT {
                AnimparamButtonNone = 0,
                AnimparamButtonDpadUp = 1,
                AnimparamButtonDpadRight = 2,
                AnimparamButtonDpadDown = 3,
                AnimparamButtonDpadLeft = 4,
                AnimparamButtonA = 5,
                AnimparamButtonB = 6,
                AnimparamButtonX = 7,
                AnimparamButtonY = 8,
                AnimparamButtonLeftShoulder = 9,
                AnimparamButtonRightShoulder = 10,
                AnimparamButtonLtrigger = 11,
                AnimparamButtonRtrigger = 12
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum AnimParamNetworkSetting {
                Auto = 0,
                AlwaysNetwork = 1,
                NeverNetwork = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum FootstepLandedFootSoundTypeT {
                FootsoundLeft = 0,
                FootsoundRight = 1,
                FootsoundUseOverrideSound = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum AnimPoseControl {
                NoPoseControl = 0,
                AbsolutePoseControl = 1,
                RelativePoseControl = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum RagdollPoseControl {
                Absolute = 0,
                Relative = 1
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum VPhysXBodyPartTVPhysXFlagEnumT {
                FlagStatic = 1,
                FlagKinematic = 2,
                FlagJoint = 4,
                FlagMass = 8,
                FlagAlwaysDynamicOnClient = 16
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum VPhysXConstraintParamsTEnumFlags0T {
                Flag0ShiftInterpenetrate = 0,
                Flag0ShiftConstrain = 1,
                Flag0ShiftBreakableForce = 2,
                Flag0ShiftBreakableTorque = 3
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum VPhysXJointTFlagsT {
                JointFlagsNone = 0,
                JointFlagsBody1Fixed = 1,
                JointFlagsUseBlockSolver = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum VPhysXAggregateDataTVPhysXFlagEnumT {
                FlagIsPolysoupGeometry = 1,
                FlagLevelCollision = 16,
                FlagIgnoreScaleObsoleteDoNotUse = 32
            }
            // Alignment: 4
            // Members count: 8
            #[repr(i32)]
            pub enum MeshDrawPrimitiveFlagsT {
                MeshDrawFlagsNone = 0,
                MeshDrawFlagsUseShadowFastPath = 1,
                MeshDrawFlagsUseCompressedNormalTangent = 2,
                MeshDrawInputLayoutIsNotMatchedToMaterial = 8,
                MeshDrawFlagsUseCompressedPerVertexLighting = 16,
                MeshDrawFlagsUseUncompressedPerVertexLighting = 32,
                MeshDrawFlagsCanBatchWithDynamicShaderConstants = 64,
                MeshDrawFlagsDrawLast = 128
            }
            // Alignment: 4
            // Members count: 22
            #[repr(i32)]
            pub enum ModelSkeletonDataTBoneFlagsT {
                FlagNoBoneFlags = 0,
                FlagBoneflexdriver = 4,
                FlagCloth = 8,
                FlagPhysics = 16,
                FlagAttachment = 32,
                FlagAnimation = 64,
                FlagMesh = 128,
                FlagHitbox = 256,
                FlagBoneUsedByVertexLod0 = 1024,
                FlagBoneUsedByVertexLod1 = 2048,
                FlagBoneUsedByVertexLod2 = 4096,
                FlagBoneUsedByVertexLod3 = 8192,
                FlagBoneUsedByVertexLod4 = 16384,
                FlagBoneUsedByVertexLod5 = 32768,
                FlagBoneUsedByVertexLod6 = 65536,
                FlagBoneUsedByVertexLod7 = 131072,
                FlagBoneMergeRead = 262144,
                FlagBoneMergeWrite = 524288,
                FlagAllBoneFlags = 1048575,
                BlendPrealigned = 1048576,
                FlagRigidlength = 2097152,
                FlagProcedural = 4194304
            }
            // Alignment: 4
            // Members count: 15
            #[repr(i32)]
            pub enum PermModelInfoTFlagEnum {
                FlagTranslucent = 1,
                FlagTranslucentTwoPass = 2,
                FlagModelIsRuntimeCombined = 4,
                FlagSource1Import = 8,
                FlagModelPartChild = 16,
                FlagNavGenNone = 32,
                FlagNavGenHull = 64,
                FlagNoForcedFade = 2048,
                FlagHasSkinnedMeshes = 1024,
                FlagDoNotCastShadows = 131072,
                FlagForcePhonemeCrossfade = 4096,
                FlagNoAnimEvents = 1048576,
                FlagAnimationDrivenFlexes = 2097152,
                FlagImplicitBindPoseSequence = 4194304,
                FlagModelDoc = 8388608
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ModelBoneFlexComponentT {
                ModelBoneFlexInvalid = -1,
                ModelBoneFlexTx = 0,
                ModelBoneFlexTy = 1,
                ModelBoneFlexTz = 2
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum ModelConfigAttachmentTypeT {
                ModelConfigAttachmentInvalid = -1,
                ModelConfigAttachmentBoneOrAttachment = 0,
                ModelConfigAttachmentRootRelative = 1,
                ModelConfigAttachmentBonemerge = 2,
                ModelConfigAttachmentCount = 3
            }
            // Alignment: 4
            // Members count: 26
            #[repr(i32)]
            pub enum FlexOpCodeT {
                FlexOpConst = 1,
                FlexOpFetch1 = 2,
                FlexOpFetch2 = 3,
                FlexOpAdd = 4,
                FlexOpSub = 5,
                FlexOpMul = 6,
                FlexOpDiv = 7,
                FlexOpNeg = 8,
                FlexOpExp = 9,
                FlexOpOpen = 10,
                FlexOpClose = 11,
                FlexOpComma = 12,
                FlexOpMax = 13,
                FlexOpMin = 14,
                FlexOp2way0 = 15,
                FlexOp2way1 = 16,
                FlexOpNway = 17,
                FlexOpCombo = 18,
                FlexOpDominate = 19,
                FlexOpDmeLowerEyelid = 20,
                FlexOpDmeUpperEyelid = 21,
                FlexOpSqrt = 22,
                FlexOpRemapvalclamped = 23,
                FlexOpSin = 24,
                FlexOpCos = 25,
                FlexOpAbs = 26
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum MorphFlexControllerRemapTypeT {
                MorphFlexcontrollerRemapPassthru = 0,
                MorphFlexcontrollerRemap2way = 1,
                MorphFlexcontrollerRemapNway = 2,
                MorphFlexcontrollerRemapEyelid = 3
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum MorphBundleTypeT {
                MorphBundleTypeNone = 0,
                MorphBundleTypePositionSpeed = 1,
                MorphBundleTypeNormalWrinkle = 2,
                MorphBundleTypeCount = 3
            }
            // Alignment: 1
            // Members count: 5
            #[repr(i8)]
            pub enum MovementGaitT {
                EInvalid = -1,
                EWalk = 0,
                EJog = 1,
                ERun = 2,
                ESprint = 3
            }
            // Alignment: 1
            // Members count: 3
            #[repr(i8)]
            pub enum CNmBoneMaskWeightInfoT {
                Zero = 0,
                Mixed = 1,
                One = 2
            }
            // Alignment: 1
            // Members count: 4
            #[repr(i8)]
            pub enum NmFootPhaseT {
                LeftFootDown = 0,
                RightFootPassing = 1,
                RightFootDown = 2,
                LeftFootPassing = 3
            }
            // Alignment: 1
            // Members count: 6
            #[repr(i8)]
            pub enum NmFootPhaseConditionT {
                LeftFootDown = 0,
                LeftFootPassing = 1,
                LeftPhase = 4,
                RightFootDown = 2,
                RightFootPassing = 3,
                RightPhase = 5
            }
            // Alignment: 1
            // Members count: 3
            #[repr(i8)]
            pub enum NmTransitionRuleT {
                AllowTransition = 0,
                ConditionallyAllowTransition = 1,
                BlockTransition = 2
            }
            // Alignment: 1
            // Members count: 4
            #[repr(i8)]
            pub enum NmTransitionRuleConditionT {
                AnyAllowed = 0,
                FullyAllowed = 1,
                ConditionallyAllowed = 2,
                Blocked = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum NmFrameSnapEventModeT {
                Floor = 0,
                Round = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum IkChannelMode {
                TwoBone = 0,
                TwoBoneTranslate = 1,
                OneBone = 2,
                OneBoneTranslate = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum EDemoBoneSelectionMode {
                CaptureAllBones = 0,
                CaptureSelectedBones = 1
            }
            // Alignment: 4
            // Members count: 37
            #[repr(i32)]
            pub enum AnimValueSource {
                MoveHeading = 0,
                MoveSpeed = 1,
                ForwardSpeed = 2,
                StrafeSpeed = 3,
                FacingHeading = 4,
                ManualFacingHeading = 5,
                LookHeading = 6,
                LookPitch = 7,
                LookDistance = 8,
                Parameter = 9,
                WayPointHeading = 10,
                WayPointDistance = 11,
                BoundaryRadius = 12,
                TargetMoveHeading = 13,
                TargetMoveSpeed = 14,
                AccelerationHeading = 15,
                AccelerationSpeed = 16,
                SlopeHeading = 17,
                SlopeAngle = 18,
                SlopePitch = 19,
                SlopeYaw = 20,
                GoalDistance = 21,
                AccelerationLeftRight = 22,
                AccelerationFrontBack = 23,
                RootMotionSpeed = 24,
                RootMotionTurnSpeed = 25,
                MoveHeadingRelativeToLookHeading = 26,
                MaxMoveSpeed = 27,
                FingerCurlThumb = 28,
                FingerCurlIndex = 29,
                FingerCurlMiddle = 30,
                FingerCurlRing = 31,
                FingerCurlPinky = 32,
                FingerSplayThumbIndex = 33,
                FingerSplayIndexMiddle = 34,
                FingerSplayMiddleRing = 35,
                FingerSplayRingPinky = 36
            }
            // Alignment: 4
            // Members count: 13
            #[repr(i32)]
            pub enum AnimVectorSource {
                MoveDirection = 0,
                FacingDirection = 1,
                LookDirection = 2,
                VectorParameter = 3,
                WayPointDirection = 4,
                Acceleration = 5,
                SlopeNormal = 6,
                SlopeNormalWorldSpace = 7,
                LookTarget = 8,
                LookTargetWorldSpace = 9,
                WayPointPosition = 10,
                GoalPosition = 11,
                RootMotionVelocity = 12
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum DampingSpeedFunction {
                NoDamping = 0,
                Constant = 1,
                Spring = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum AnimNodeNetworkMode {
                ServerAuthoritative = 0,
                ClientSimulate = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum StateActionBehavior {
                StatetagbehaviorActiveWhileCurrent = 0,
                StatetagbehaviorFireOnEnter = 1,
                StatetagbehaviorFireOnExit = 2,
                StatetagbehaviorFireOnEnterAndExit = 3
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum FieldNetworkOption {
                Auto = 0,
                ForceEnable = 1,
                ForceDisable = 2
            }
            // Alignment: 4
            // Members count: 8
            #[repr(i32)]
            pub enum FootFallTagFootT {
                Foot1 = 0,
                Foot2 = 1,
                Foot3 = 2,
                Foot4 = 3,
                Foot5 = 4,
                Foot6 = 5,
                Foot7 = 6,
                Foot8 = 7
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum MatterialAttributeTagTypeT {
                MaterialAttributeTagValue = 0,
                MaterialAttributeTagColor = 1
            }
            // Alignment: 1
            // Members count: 3
            #[repr(i8)]
            pub enum VelocityMetricMode {
                DirectionOnly = 0,
                MagnitudeOnly = 1,
                DirectionAndMagnitude = 2
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum AimMatrixBlendMode {
                AimMatrixBlendModeNone = 0,
                AimMatrixBlendModeAdditive = 1,
                AimMatrixBlendModeModelSpaceAdditive = 2,
                AimMatrixBlendModeBoneMask = 3
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum BoneMaskBlendSpace {
                BlendSpaceParent = 0,
                BlendSpaceModel = 1,
                BlendSpaceModelRotationOnly = 2,
                BlendSpaceModelTranslationOnly = 3
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum JiggleBoneSimSpace {
                SimSpaceLocal = 0,
                SimSpaceModel = 1,
                SimSpaceWorld = 2
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum SolveIkChainAnimNodeDebugSetting {
                SolveikchainanimnodedebugsettingNone = 0,
                SolveikchainanimnodedebugsettingXAxisCircle = 1,
                SolveikchainanimnodedebugsettingYAxisCircle = 2,
                SolveikchainanimnodedebugsettingZAxisCircle = 3,
                SolveikchainanimnodedebugsettingForward = 4,
                SolveikchainanimnodedebugsettingUp = 5,
                SolveikchainanimnodedebugsettingLeft = 6
            }
            // Alignment: 2
            // Members count: 3
            #[repr(i16)]
            pub enum AnimScriptType {
                AnimscriptTypeInvalid = -1,
                AnimscriptFuseGeneral = 0,
                AnimscriptFuseStatemachine = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum BinaryNodeTiming {
                UseChild1 = 0,
                UseChild2 = 1,
                SyncChildren = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum BinaryNodeChildOption {
                Child1 = 0,
                Child2 = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum BlendKeyType {
                BlendKeyUserValue = 0,
                BlendKeyVelocity = 1,
                BlendKeyDistance = 2,
                BlendKeyRemainingDistance = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum Blend2DMode {
                Blend2DModeGeneral = 0,
                Blend2DModeDirectional = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ChoiceMethod {
                WeightedRandom = 0,
                WeightedRandomNoRepeat = 1,
                Iterate = 2,
                IterateRandom = 3
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ChoiceChangeMethod {
                OnReset = 0,
                OnCycleEnd = 1,
                OnResetOrCycleEnd = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ChoiceBlendMethod {
                SingleBlendTime = 0,
                PerChoiceBlendTimes = 1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum FootLockSubVisualization {
                FootlocksubvisualizationReachabilityAnalysis = 0,
                FootlocksubvisualizationIkSolve = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum FootPinningTimingSource {
                FootMotion = 0,
                Tag = 1,
                Parameter = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum StepPhase {
                StepPhaseOnGround = 0,
                StepPhaseInAir = 1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum JumpCorrectionMethod {
                ScaleMotion = 0,
                AddCorrectionDelta = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum SelectorTagBehaviorT {
                SelectorTagBehaviorOnWhileCurrent = 0,
                SelectorTagBehaviorOffWhenFinished = 1,
                SelectorTagBehaviorOffBeforeFinished = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum StanceOverrideMode {
                Sequence = 0,
                Node = 1
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum ResetCycleOption {
                Beginning = 0,
                SameCycleAsSource = 1,
                InverseSourceCycle = 2,
                FixedValue = 3,
                SameTimeAsSource = 4
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum IkEndEffectorType {
                IkEndEffectorAttachment = 0,
                IkEndEffectorBone = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum IkTargetType {
                IkTargetAttachment = 0,
                IkTargetBone = 1,
                IkTargetParameterModelSpace = 2,
                IkTargetParameterWorldSpace = 3
            }
            // Alignment: 1
            // Members count: 3
            #[repr(i8)]
            pub enum PoseTypeT {
                PosetypeStatic = 0,
                PosetypeDynamic = 1,
                PosetypeInvalid = 255
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum CAnimationGraphVisualizerPrimitiveType {
                AnimationgraphvisualizerprimitivetypeText = 0,
                AnimationgraphvisualizerprimitivetypeSphere = 1,
                AnimationgraphvisualizerprimitivetypeLine = 2,
                AnimationgraphvisualizerprimitivetypePie = 3,
                AnimationgraphvisualizerprimitivetypeAxis = 4
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum FacingMode {
                FacingModeManual = 0,
                FacingModePath = 1,
                FacingModeLookTarget = 2
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum IkSolverType {
                IksolverPerlin = 0,
                IksolverTwoBone = 1,
                IksolverFabrik = 2,
                IksolverDogLeg3Bone = 3,
                IksolverCcd = 4,
                IksolverCount = 5
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum IkTargetSource {
                IktargetsourceBone = 0,
                IktargetsourceAnimgraphParameter = 1,
                IktargetsourceCount = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum IkTargetCoordinateSystem {
                IktargetcoordinatesystemWorldSpace = 0,
                IktargetcoordinatesystemModelSpace = 1,
                IktargetcoordinatesystemCount = 2
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyArrayElementNameKey
            pub mod mood_animation_t {
                pub const M_S_NAME: usize = 0x0; // CUtlString
                pub const M_FL_WEIGHT: usize = 0x8; // float32
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyArrayElementNameKey
            pub mod mood_animation_layer_t {
                pub const M_S_NAME: usize = 0x0; // CUtlString
                pub const M_B_ACTIVE_LISTENING: usize = 0x8; // bool
                pub const M_B_ACTIVE_TALKING: usize = 0x9; // bool
                pub const M_LAYER_ANIMATIONS: usize = 0x10; // CUtlVector< MoodAnimation_t >
                pub const M_FL_INTENSITY: usize = 0x28; // CRangeFloat
                pub const M_FL_DURATION_SCALE: usize = 0x30; // CRangeFloat
                pub const M_B_SCALE_WITH_INTS: usize = 0x38; // bool
                pub const M_FL_NEXT_START: usize = 0x3C; // CRangeFloat
                pub const M_FL_START_OFFSET: usize = 0x44; // CRangeFloat
                pub const M_FL_END_OFFSET: usize = 0x4C; // CRangeFloat
                pub const M_FL_FADE_IN: usize = 0x54; // float32
                pub const M_FL_FADE_OUT: usize = 0x58; // float32
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod animation_decode_debug_dump_element_t {
                pub const M_N_ENTITY_INDEX: usize = 0x0; // int32
                pub const M_MODEL_NAME: usize = 0x8; // CUtlString
                pub const M_POSE_PARAMS: usize = 0x10; // CUtlVector< CUtlString >
                pub const M_DECODE_OPS: usize = 0x28; // CUtlVector< CUtlString >
                pub const M_INTERNAL_OPS: usize = 0x40; // CUtlVector< CUtlString >
                pub const M_DECODED_ANIMS: usize = 0x58; // CUtlVector< CUtlString >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod animation_decode_debug_dump_t {
                pub const M_PROCESSING_TYPE: usize = 0x0; // AnimationProcessingType_t
                pub const M_ELEMS: usize = 0x8; // CUtlVector< AnimationDecodeDebugDumpElement_t >
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod animation_snapshot_base_t {
                pub const M_FL_REAL_TIME: usize = 0x0; // float32
                pub const M_ROOT_TO_WORLD: usize = 0x10; // matrix3x4a_t
                pub const M_B_BONES_IN_WORLD_SPACE: usize = 0x40; // bool
                pub const M_BONE_SETUP_MASK: usize = 0x48; // CUtlVector< uint32 >
                pub const M_BONE_TRANSFORMS: usize = 0x60; // CUtlVector< matrix3x4a_t >
                pub const M_FLEX_CONTROLLERS: usize = 0x78; // CUtlVector< float32 >
                pub const M_SNAPSHOT_TYPE: usize = 0x90; // AnimationSnapshotType_t
                pub const M_B_HAS_DECODE_DUMP: usize = 0x94; // bool
                pub const M_DECODE_DUMP: usize = 0x98; // AnimationDecodeDebugDumpElement_t
            }
            // Parent: animation_snapshot_base_t
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod animation_snapshot_t {
                pub const M_N_ENT_INDEX: usize = 0x110; // int32
                pub const M_MODEL_NAME: usize = 0x118; // CUtlString
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_bone_difference {
                pub const M_NAME: usize = 0x0; // CBufferString
                pub const M_PARENT: usize = 0x10; // CBufferString
                pub const M_POS_ERROR: usize = 0x20; // 
                pub const M_B_HAS_ROTATION: usize = 0x2C; // bool
                pub const M_B_HAS_MOVEMENT: usize = 0x2D; // bool
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_morph_difference {
                pub const M_NAME: usize = 0x0; // CBufferString
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_user_difference {
                pub const M_NAME: usize = 0x0; // CBufferString
                pub const M_N_TYPE: usize = 0x10; // int32
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_encode_difference {
                pub const M_BONE_ARRAY: usize = 0x0; // CUtlVector< CAnimBoneDifference >
                pub const M_MORPH_ARRAY: usize = 0x18; // CUtlVector< CAnimMorphDifference >
                pub const M_USER_ARRAY: usize = 0x30; // CUtlVector< CAnimUserDifference >
                pub const M_B_HAS_ROTATION_BIT_ARRAY: usize = 0x48; // CUtlVector< uint8 >
                pub const M_B_HAS_MOVEMENT_BIT_ARRAY: usize = 0x60; // CUtlVector< uint8 >
                pub const M_B_HAS_MORPH_BIT_ARRAY: usize = 0x78; // CUtlVector< uint8 >
                pub const M_B_HAS_USER_BIT_ARRAY: usize = 0x90; // CUtlVector< uint8 >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_event_definition {
                pub const M_N_FRAME: usize = 0x8; // int32
                pub const M_FL_CYCLE: usize = 0xC; // float32
                pub const M_EVENT_DATA: usize = 0x10; // KeyValues3
                pub const M_S_LEGACY_OPTIONS: usize = 0x20; // CBufferString
                pub const M_S_EVENT_NAME: usize = 0x30; // CGlobalSymbol
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_movement {
                pub const ENDFRAME: usize = 0x0; // int32
                pub const MOTIONFLAGS: usize = 0x4; // int32
                pub const V0: usize = 0x8; // float32
                pub const V1: usize = 0xC; // float32
                pub const ANGLE: usize = 0x10; // float32
                pub const VECTOR: usize = 0x14; // 
                pub const POSITION: usize = 0x20; // 
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_local_hierarchy {
                pub const M_S_BONE: usize = 0x0; // CBufferString
                pub const M_S_NEW_PARENT: usize = 0x10; // CBufferString
                pub const M_N_START_FRAME: usize = 0x20; // int32
                pub const M_N_PEAK_FRAME: usize = 0x24; // int32
                pub const M_N_TAIL_FRAME: usize = 0x28; // int32
                pub const M_N_END_FRAME: usize = 0x2C; // int32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_decoder {
                pub const M_SZ_NAME: usize = 0x0; // CBufferString
                pub const M_N_VERSION: usize = 0x10; // int32
                pub const M_N_TYPE: usize = 0x14; // int32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_frame_segment {
                pub const M_N_UNIQUE_FRAME_INDEX: usize = 0x0; // int32
                pub const M_N_LOCAL_ELEMENT_MASKS: usize = 0x4; // uint32
                pub const M_N_LOCAL_CHANNEL: usize = 0x8; // int32
                pub const M_CONTAINER: usize = 0x10; // CUtlBinaryBlock
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_frame_block_anim {
                pub const M_N_START_FRAME: usize = 0x0; // int32
                pub const M_N_END_FRAME: usize = 0x4; // int32
                pub const M_SEGMENT_INDEX_ARRAY: usize = 0x8; // CUtlVector< int32 >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_encoded_frames {
                pub const M_FILE_NAME: usize = 0x0; // CBufferString
                pub const M_N_FRAMES: usize = 0x10; // int32
                pub const M_N_FRAMES_PER_BLOCK: usize = 0x14; // int32
                pub const M_FRAMEBLOCK_ARRAY: usize = 0x18; // CUtlVector< CAnimFrameBlockAnim >
                pub const M_USAGE_DIFFERENCES: usize = 0x30; // CAnimEncodeDifference
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_desc_flag {
                pub const M_B_LOOPING: usize = 0x0; // bool
                pub const M_B_ALL_ZEROS: usize = 0x1; // bool
                pub const M_B_HIDDEN: usize = 0x2; // bool
                pub const M_B_DELTA: usize = 0x3; // bool
                pub const M_B_LEGACY_WORLDSPACE: usize = 0x4; // bool
                pub const M_B_MODEL_DOC: usize = 0x5; // bool
                pub const M_B_IMPLICIT_SEQ_IGNORE_DELTA: usize = 0x6; // bool
                pub const M_B_ANIM_GRAPH_ADDITIVE: usize = 0x7; // bool
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_sequence_params {
                pub const M_FL_FADE_IN_TIME: usize = 0x0; // float32
                pub const M_FL_FADE_OUT_TIME: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_desc {
                pub const M_NAME: usize = 0x0; // CBufferString
                pub const M_FLAGS: usize = 0x10; // CAnimDesc_Flag
                pub const FPS: usize = 0x18; // float32
                pub const M_DATA: usize = 0x20; // CAnimEncodedFrames
                pub const M_MOVEMENT_ARRAY: usize = 0xF8; // CUtlVector< CAnimMovement >
                pub const M_EVENT_ARRAY: usize = 0x110; // CUtlVector< CAnimEventDefinition >
                pub const M_ACTIVITY_ARRAY: usize = 0x128; // CUtlVector< CAnimActivity >
                pub const M_HIERARCHY_ARRAY: usize = 0x140; // CUtlVector< CAnimLocalHierarchy >
                pub const FRAMESTALLTIME: usize = 0x158; // float32
                pub const M_VEC_ROOT_MIN: usize = 0x15C; // 
                pub const M_VEC_ROOT_MAX: usize = 0x168; // 
                pub const M_VEC_BONE_WORLD_MIN: usize = 0x178; // CUtlVector< Vector >
                pub const M_VEC_BONE_WORLD_MAX: usize = 0x190; // CUtlVector< Vector >
                pub const M_SEQUENCE_PARAMS: usize = 0x1A8; // CAnimSequenceParams
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_activity {
                pub const M_NAME: usize = 0x0; // CBufferString
                pub const M_N_ACTIVITY: usize = 0x10; // int32
                pub const M_N_FLAGS: usize = 0x14; // int32
                pub const M_N_WEIGHT: usize = 0x18; // int32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_data {
                pub const M_NAME: usize = 0x10; // CBufferString
                pub const M_ANIM_ARRAY: usize = 0x20; // CUtlVector< CAnimDesc >
                pub const M_DECODER_ARRAY: usize = 0x38; // CUtlVector< CAnimDecoder >
                pub const M_N_MAX_UNIQUE_FRAME_INDEX: usize = 0x50; // int32
                pub const M_SEGMENT_ARRAY: usize = 0x58; // CUtlVector< CAnimFrameSegment >
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_bone {
                pub const M_NAME: usize = 0x0; // CBufferString
                pub const M_PARENT: usize = 0x10; // int32
                pub const M_POS: usize = 0x14; // 
                pub const M_QUAT: usize = 0x20; // QuaternionStorage
                pub const M_SCALE: usize = 0x30; // float32
                pub const M_Q_ALIGNMENT: usize = 0x34; // QuaternionStorage
                pub const M_FLAGS: usize = 0x44; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_user {
                pub const M_NAME: usize = 0x0; // CBufferString
                pub const M_N_TYPE: usize = 0x10; // int32
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_data_channel_desc {
                pub const M_SZ_CHANNEL_CLASS: usize = 0x0; // CBufferString
                pub const M_SZ_VARIABLE_NAME: usize = 0x10; // CBufferString
                pub const M_N_FLAGS: usize = 0x20; // int32
                pub const M_N_TYPE: usize = 0x24; // int32
                pub const M_SZ_GROUPING: usize = 0x28; // CBufferString
                pub const M_SZ_DESCRIPTION: usize = 0x38; // CBufferString
                pub const M_SZ_ELEMENT_NAME_ARRAY: usize = 0x48; // CUtlVector< CBufferString >
                pub const M_N_ELEMENT_INDEX_ARRAY: usize = 0x60; // CUtlVector< int32 >
                pub const M_N_ELEMENT_MASK_ARRAY: usize = 0x78; // CUtlVector< uint32 >
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_key_data {
                pub const M_NAME: usize = 0x0; // CBufferString
                pub const M_BONE_ARRAY: usize = 0x10; // CUtlVector< CAnimBone >
                pub const M_USER_ARRAY: usize = 0x28; // CUtlVector< CAnimUser >
                pub const M_MORPH_ARRAY: usize = 0x40; // CUtlVector< CBufferString >
                pub const M_N_CHANNEL_ELEMENTS: usize = 0x58; // int32
                pub const M_DATA_CHANNEL_ARRAY: usize = 0x60; // CUtlVector< CAnimDataChannelDesc >
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_animation_group {
                pub const M_N_FLAGS: usize = 0x10; // uint32
                pub const M_NAME: usize = 0x18; // CBufferString
                pub const M_LOCAL_H_ANIM_ARRAY_HANDLE: usize = 0x60; // CUtlVector< CStrongHandle< InfoForResourceTypeCAnimData > >
                pub const M_INCLUDED_GROUP_ARRAY_HANDLE: usize = 0x78; // CUtlVector< CStrongHandle< InfoForResourceTypeCAnimationGroup > >
                pub const M_DIRECT_H_SEQ_GROUP_HANDLE: usize = 0x90; // CStrongHandle< InfoForResourceTypeCSequenceGroupData >
                pub const M_DECODE_KEY: usize = 0x98; // CAnimKeyData
                pub const M_SZ_SCRIPTS: usize = 0x110; // CUtlVector< CBufferString >
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_auto_layer_flag {
                pub const M_B_POST: usize = 0x0; // bool
                pub const M_B_SPLINE: usize = 0x1; // bool
                pub const M_B_X_FADE: usize = 0x2; // bool
                pub const M_B_NO_BLEND: usize = 0x3; // bool
                pub const M_B_LOCAL: usize = 0x4; // bool
                pub const M_B_POSE: usize = 0x5; // bool
                pub const M_B_FETCH_FRAME: usize = 0x6; // bool
                pub const M_B_SUBTRACT: usize = 0x7; // bool
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_auto_layer {
                pub const M_N_LOCAL_REFERENCE: usize = 0x0; // int16
                pub const M_N_LOCAL_POSE: usize = 0x2; // int16
                pub const M_FLAGS: usize = 0x4; // CSeqAutoLayerFlag
                pub const M_START: usize = 0xC; // float32
                pub const M_PEAK: usize = 0x10; // float32
                pub const M_TAIL: usize = 0x14; // float32
                pub const M_END: usize = 0x18; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_ik_lock {
                pub const M_FL_POS_WEIGHT: usize = 0x0; // float32
                pub const M_FL_ANGLE_WEIGHT: usize = 0x4; // float32
                pub const M_N_LOCAL_BONE: usize = 0x8; // int16
                pub const M_B_BONES_ORIENTED_ALONG_POSITIVE_X: usize = 0xA; // bool
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_bone_mask_list {
                pub const M_S_NAME: usize = 0x0; // CBufferString
                pub const M_N_LOCAL_BONE_ARRAY: usize = 0x10; // CUtlVector< int16 >
                pub const M_FL_BONE_WEIGHT_ARRAY: usize = 0x28; // CUtlVector< float32 >
                pub const M_FL_DEFAULT_MORPH_CTRL_WEIGHT: usize = 0x40; // float32
                pub const M_MORPH_CTRL_WEIGHT_ARRAY: usize = 0x48; // CUtlVector< CUtlPair< CBufferString, float32 > >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_scale_set {
                pub const M_S_NAME: usize = 0x0; // CBufferString
                pub const M_B_ROOT_OFFSET: usize = 0x10; // bool
                pub const M_V_ROOT_OFFSET: usize = 0x14; // 
                pub const M_N_LOCAL_BONE_ARRAY: usize = 0x20; // CUtlVector< int16 >
                pub const M_FL_BONE_SCALE_ARRAY: usize = 0x38; // CUtlVector< float32 >
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_multi_fetch_flag {
                pub const M_B_REALTIME: usize = 0x0; // bool
                pub const M_B_CYLEPOSE: usize = 0x1; // bool
                pub const M_B0_D: usize = 0x2; // bool
                pub const M_B1_D: usize = 0x3; // bool
                pub const M_B2_D: usize = 0x4; // bool
                pub const M_B2_D_TRI: usize = 0x5; // bool
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_multi_fetch {
                pub const M_FLAGS: usize = 0x0; // CSeqMultiFetchFlag
                pub const M_LOCAL_REFERENCE_ARRAY: usize = 0x8; // CUtlVector< int16 >
                pub const M_N_GROUP_SIZE: usize = 0x20; // int32[2]
                pub const M_N_LOCAL_POSE: usize = 0x28; // int32[2]
                pub const M_POSE_KEY_ARRAY0: usize = 0x30; // CUtlVector< float32 >
                pub const M_POSE_KEY_ARRAY1: usize = 0x48; // CUtlVector< float32 >
                pub const M_N_LOCAL_CYCLE_POSE_PARAMETER: usize = 0x60; // int32
                pub const M_B_CALCULATE_POSE_PARAMETERS: usize = 0x64; // bool
                pub const M_B_FIXED_BLEND_WEIGHT: usize = 0x65; // bool
                pub const M_FL_FIXED_BLEND_WEIGHT_VALS: usize = 0x68; // float32[2]
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_seq_desc_flag {
                pub const M_B_LOOPING: usize = 0x0; // bool
                pub const M_B_SNAP: usize = 0x1; // bool
                pub const M_B_AUTOPLAY: usize = 0x2; // bool
                pub const M_B_POST: usize = 0x3; // bool
                pub const M_B_HIDDEN: usize = 0x4; // bool
                pub const M_B_MULTI: usize = 0x5; // bool
                pub const M_B_LEGACY_DELTA: usize = 0x6; // bool
                pub const M_B_LEGACY_WORLDSPACE: usize = 0x7; // bool
                pub const M_B_LEGACY_CYCLEPOSE: usize = 0x8; // bool
                pub const M_B_LEGACY_REALTIME: usize = 0x9; // bool
                pub const M_B_MODEL_DOC: usize = 0xA; // bool
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_transition {
                pub const M_FL_FADE_IN_TIME: usize = 0x0; // float32
                pub const M_FL_FADE_OUT_TIME: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_s1_seq_desc {
                pub const M_S_NAME: usize = 0x0; // CBufferString
                pub const M_FLAGS: usize = 0x10; // CSeqSeqDescFlag
                pub const M_FETCH: usize = 0x20; // CSeqMultiFetch
                pub const M_N_LOCAL_WEIGHTLIST: usize = 0x90; // int32
                pub const M_AUTO_LAYER_ARRAY: usize = 0x98; // CUtlVector< CSeqAutoLayer >
                pub const M_IK_LOCK_ARRAY: usize = 0xB0; // CUtlVector< CSeqIKLock >
                pub const M_TRANSITION: usize = 0xC8; // CSeqTransition
                pub const M_SEQUENCE_KEYS: usize = 0xD0; // KeyValues3
                pub const M_LEGACY_KEY_VALUE_TEXT: usize = 0xE0; // CBufferString
                pub const M_ACTIVITY_ARRAY: usize = 0xF0; // CUtlVector< CAnimActivity >
                pub const M_FOOT_MOTION: usize = 0x108; // CUtlVector< CFootMotion >
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_synth_anim_desc {
                pub const M_S_NAME: usize = 0x0; // CBufferString
                pub const M_FLAGS: usize = 0x10; // CSeqSeqDescFlag
                pub const M_TRANSITION: usize = 0x1C; // CSeqTransition
                pub const M_N_LOCAL_BASE_REFERENCE: usize = 0x24; // int16
                pub const M_N_LOCAL_BONE_MASK: usize = 0x26; // int16
                pub const M_ACTIVITY_ARRAY: usize = 0x28; // CUtlVector< CAnimActivity >
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_cmd_layer {
                pub const M_CMD: usize = 0x0; // int16
                pub const M_N_LOCAL_REFERENCE: usize = 0x2; // int16
                pub const M_N_LOCAL_BONEMASK: usize = 0x4; // int16
                pub const M_N_DST_RESULT: usize = 0x6; // int16
                pub const M_N_SRC_RESULT: usize = 0x8; // int16
                pub const M_B_SPLINE: usize = 0xA; // bool
                pub const M_FL_VAR1: usize = 0xC; // float32
                pub const M_FL_VAR2: usize = 0x10; // float32
                pub const M_N_LINE_NUMBER: usize = 0x14; // int16
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_pose_setting {
                pub const M_S_POSE_PARAMETER: usize = 0x0; // CBufferString
                pub const M_S_ATTACHMENT: usize = 0x10; // CBufferString
                pub const M_S_REFERENCE_SEQUENCE: usize = 0x20; // CBufferString
                pub const M_FL_VALUE: usize = 0x30; // float32
                pub const M_B_X: usize = 0x34; // bool
                pub const M_B_Y: usize = 0x35; // bool
                pub const M_B_Z: usize = 0x36; // bool
                pub const M_E_TYPE: usize = 0x38; // int32
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_cmd_seq_desc {
                pub const M_S_NAME: usize = 0x0; // CBufferString
                pub const M_FLAGS: usize = 0x10; // CSeqSeqDescFlag
                pub const M_TRANSITION: usize = 0x1C; // CSeqTransition
                pub const M_N_FRAME_RANGE_SEQUENCE: usize = 0x24; // int16
                pub const M_N_FRAME_COUNT: usize = 0x26; // int16
                pub const M_FL_FPS: usize = 0x28; // float32
                pub const M_N_SUB_CYCLES: usize = 0x2C; // int16
                pub const M_NUM_LOCAL_RESULTS: usize = 0x2E; // int16
                pub const M_CMD_LAYER_ARRAY: usize = 0x30; // CUtlVector< CSeqCmdLayer >
                pub const M_EVENT_ARRAY: usize = 0x48; // CUtlVector< CAnimEventDefinition >
                pub const M_ACTIVITY_ARRAY: usize = 0x60; // CUtlVector< CAnimActivity >
                pub const M_POSE_SETTING_ARRAY: usize = 0x78; // CUtlVector< CSeqPoseSetting >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_seq_pose_param_desc {
                pub const M_S_NAME: usize = 0x0; // CBufferString
                pub const M_FL_START: usize = 0x10; // float32
                pub const M_FL_END: usize = 0x14; // float32
                pub const M_FL_LOOP: usize = 0x18; // float32
                pub const M_B_LOOPING: usize = 0x1C; // bool
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_sequence_group_data {
                pub const M_S_NAME: usize = 0x10; // CBufferString
                pub const M_N_FLAGS: usize = 0x20; // uint32
                pub const M_LOCAL_SEQUENCE_NAME_ARRAY: usize = 0x28; // CUtlVector< CBufferString >
                pub const M_LOCAL_S1_SEQ_DESC_ARRAY: usize = 0x40; // CUtlVector< CSeqS1SeqDesc >
                pub const M_LOCAL_MULTI_SEQ_DESC_ARRAY: usize = 0x58; // CUtlVector< CSeqS1SeqDesc >
                pub const M_LOCAL_SYNTH_ANIM_DESC_ARRAY: usize = 0x70; // CUtlVector< CSeqSynthAnimDesc >
                pub const M_LOCAL_CMD_SEQ_DESC_ARRAY: usize = 0x88; // CUtlVector< CSeqCmdSeqDesc >
                pub const M_LOCAL_BONE_MASK_ARRAY: usize = 0xA0; // CUtlVector< CSeqBoneMaskList >
                pub const M_LOCAL_SCALE_SET_ARRAY: usize = 0xB8; // CUtlVector< CSeqScaleSet >
                pub const M_LOCAL_BONE_NAME_ARRAY: usize = 0xD0; // CUtlVector< CBufferString >
                pub const M_LOCAL_NODE_NAME: usize = 0xE8; // CBufferString
                pub const M_LOCAL_POSE_PARAM_ARRAY: usize = 0xF8; // CUtlVector< CSeqPoseParamDesc >
                pub const M_KEY_VALUES: usize = 0x110; // KeyValues3
                pub const M_LOCAL_IK_AUTOPLAY_LOCK_ARRAY: usize = 0x120; // CUtlVector< CSeqIKLock >
            }
            // Parent: None
            // Fields count: 17
            pub mod c_compressor_group {
                pub const M_N_TOTAL_ELEMENT_COUNT: usize = 0x0; // int32
                pub const M_SZ_CHANNEL_CLASS: usize = 0x8; // CUtlVector< char* >
                pub const M_SZ_VARIABLE_NAME: usize = 0x20; // CUtlVector< char* >
                pub const M_N_TYPE: usize = 0x38; // CUtlVector< fieldtype_t >
                pub const M_N_FLAGS: usize = 0x50; // CUtlVector< int32 >
                pub const M_SZ_GROUPING: usize = 0x68; // CUtlVector< CUtlString >
                pub const M_N_COMPRESSOR_INDEX: usize = 0x80; // CUtlVector< int32 >
                pub const M_SZ_ELEMENT_NAMES: usize = 0x98; // CUtlVector< CUtlVector< char* > >
                pub const M_N_ELEMENT_UNIQUE_ID: usize = 0xB0; // CUtlVector< CUtlVector< int32 > >
                pub const M_N_ELEMENT_MASK: usize = 0xC8; // CUtlVector< uint32 >
                pub const M_VECTOR_COMPRESSOR: usize = 0xF8; // CUtlVector< CCompressor< Vector >* >
                pub const M_QUATERNION_COMPRESSOR: usize = 0x110; // CUtlVector< CCompressor< QuaternionStorage >* >
                pub const M_INT_COMPRESSOR: usize = 0x128; // CUtlVector< CCompressor< int32 >* >
                pub const M_BOOL_COMPRESSOR: usize = 0x140; // CUtlVector< CCompressor< bool >* >
                pub const M_COLOR_COMPRESSOR: usize = 0x158; // CUtlVector< CCompressor< Color >* >
                pub const M_VECTOR2_D_COMPRESSOR: usize = 0x170; // CUtlVector< CCompressor< Vector2D >* >
                pub const M_VECTOR4_D_COMPRESSOR: usize = 0x188; // CUtlVector< CCompressor< Vector4D >* >
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyEditClassAsString
            pub mod c_anim_enum {
                pub const M_VALUE: usize = 0x0; // uint8
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyEditClassAsString
            pub mod anim_node_id {
                pub const M_ID: usize = 0x0; // uint32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyEditClassAsString
            pub mod anim_node_output_id {
                pub const M_ID: usize = 0x0; // uint32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyEditClassAsString
            pub mod anim_state_id {
                pub const M_ID: usize = 0x0; // uint32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyEditClassAsString
            pub mod anim_param_id {
                pub const M_ID: usize = 0x0; // uint32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyEditClassAsString
            pub mod anim_tag_id {
                pub const M_ID: usize = 0x0; // uint32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyEditClassAsString
            pub mod anim_component_id {
                pub const M_ID: usize = 0x0; // uint32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyEditClassAsString
            pub mod anim_script_handle {
                pub const M_ID: usize = 0x0; // uint32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_attachment {
                pub const M_INFLUENCE_ROTATIONS: usize = 0x0; // Quaternion[3]
                pub const M_INFLUENCE_OFFSETS: usize = 0x30; // VectorAligned[3]
                pub const M_INFLUENCE_INDICES: usize = 0x60; // int32[3]
                pub const M_INFLUENCE_WEIGHTS: usize = 0x6C; // float32[3]
                pub const M_NUM_INFLUENCES: usize = 0x78; // uint8
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_physics2_shape_def_t {
                pub const M_SPHERES: usize = 0x0; // CUtlVector< RnSphereDesc_t >
                pub const M_CAPSULES: usize = 0x18; // CUtlVector< RnCapsuleDesc_t >
                pub const M_HULLS: usize = 0x30; // CUtlVector< RnHullDesc_t >
                pub const M_MESHES: usize = 0x48; // CUtlVector< RnMeshDesc_t >
                pub const M_COLLISION_ATTRIBUTE_INDICES: usize = 0x60; // CUtlVector< uint16 >
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_phys_x_body_part_t {
                pub const M_N_FLAGS: usize = 0x0; // uint32
                pub const M_FL_MASS: usize = 0x4; // float32
                pub const M_RN_SHAPE: usize = 0x8; // VPhysics2ShapeDef_t
                pub const M_N_COLLISION_ATTRIBUTE_INDEX: usize = 0x80; // uint16
                pub const M_N_RESERVED: usize = 0x82; // uint16
                pub const M_FL_INERTIA_SCALE: usize = 0x84; // float32
                pub const M_FL_LINEAR_DAMPING: usize = 0x88; // float32
                pub const M_FL_ANGULAR_DAMPING: usize = 0x8C; // float32
                pub const M_B_OVERRIDE_MASS_CENTER: usize = 0x90; // bool
                pub const M_V_MASS_CENTER_OVERRIDE: usize = 0x94; // 
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_phys_x_collision_attributes_t {
                pub const M_COLLISION_GROUP: usize = 0x0; // uint32
                pub const M_INTERACT_AS: usize = 0x8; // CUtlVector< uint32 >
                pub const M_INTERACT_WITH: usize = 0x20; // CUtlVector< uint32 >
                pub const M_INTERACT_EXCLUDE: usize = 0x38; // CUtlVector< uint32 >
                pub const M_COLLISION_GROUP_STRING: usize = 0x50; // CUtlString
                pub const M_INTERACT_AS_STRINGS: usize = 0x58; // CUtlVector< CUtlString >
                pub const M_INTERACT_WITH_STRINGS: usize = 0x70; // CUtlVector< CUtlString >
                pub const M_INTERACT_EXCLUDE_STRINGS: usize = 0x88; // CUtlVector< CUtlString >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_phys_x_range_t {
                pub const M_FL_MIN: usize = 0x0; // float32
                pub const M_FL_MAX: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 46
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_phys_x_constraint_params_t {
                pub const M_N_TYPE: usize = 0x0; // int8
                pub const M_N_TRANSLATE_MOTION: usize = 0x1; // int8
                pub const M_N_ROTATE_MOTION: usize = 0x2; // int8
                pub const M_N_FLAGS: usize = 0x3; // int8
                pub const M_ANCHOR: usize = 0x4; // Vector[2]
                pub const M_AXES: usize = 0x1C; // QuaternionStorage[2]
                pub const M_MAX_FORCE: usize = 0x3C; // float32
                pub const M_MAX_TORQUE: usize = 0x40; // float32
                pub const M_LINEAR_LIMIT_VALUE: usize = 0x44; // float32
                pub const M_LINEAR_LIMIT_RESTITUTION: usize = 0x48; // float32
                pub const M_LINEAR_LIMIT_SPRING: usize = 0x4C; // float32
                pub const M_LINEAR_LIMIT_DAMPING: usize = 0x50; // float32
                pub const M_TWIST_LOW_LIMIT_VALUE: usize = 0x54; // float32
                pub const M_TWIST_LOW_LIMIT_RESTITUTION: usize = 0x58; // float32
                pub const M_TWIST_LOW_LIMIT_SPRING: usize = 0x5C; // float32
                pub const M_TWIST_LOW_LIMIT_DAMPING: usize = 0x60; // float32
                pub const M_TWIST_HIGH_LIMIT_VALUE: usize = 0x64; // float32
                pub const M_TWIST_HIGH_LIMIT_RESTITUTION: usize = 0x68; // float32
                pub const M_TWIST_HIGH_LIMIT_SPRING: usize = 0x6C; // float32
                pub const M_TWIST_HIGH_LIMIT_DAMPING: usize = 0x70; // float32
                pub const M_SWING1_LIMIT_VALUE: usize = 0x74; // float32
                pub const M_SWING1_LIMIT_RESTITUTION: usize = 0x78; // float32
                pub const M_SWING1_LIMIT_SPRING: usize = 0x7C; // float32
                pub const M_SWING1_LIMIT_DAMPING: usize = 0x80; // float32
                pub const M_SWING2_LIMIT_VALUE: usize = 0x84; // float32
                pub const M_SWING2_LIMIT_RESTITUTION: usize = 0x88; // float32
                pub const M_SWING2_LIMIT_SPRING: usize = 0x8C; // float32
                pub const M_SWING2_LIMIT_DAMPING: usize = 0x90; // float32
                pub const M_GOAL_POSITION: usize = 0x94; // 
                pub const M_GOAL_ORIENTATION: usize = 0xA0; // QuaternionStorage
                pub const M_GOAL_ANGULAR_VELOCITY: usize = 0xB0; // 
                pub const M_DRIVE_SPRING_X: usize = 0xBC; // float32
                pub const M_DRIVE_SPRING_Y: usize = 0xC0; // float32
                pub const M_DRIVE_SPRING_Z: usize = 0xC4; // float32
                pub const M_DRIVE_DAMPING_X: usize = 0xC8; // float32
                pub const M_DRIVE_DAMPING_Y: usize = 0xCC; // float32
                pub const M_DRIVE_DAMPING_Z: usize = 0xD0; // float32
                pub const M_DRIVE_SPRING_TWIST: usize = 0xD4; // float32
                pub const M_DRIVE_SPRING_SWING: usize = 0xD8; // float32
                pub const M_DRIVE_SPRING_SLERP: usize = 0xDC; // float32
                pub const M_DRIVE_DAMPING_TWIST: usize = 0xE0; // float32
                pub const M_DRIVE_DAMPING_SWING: usize = 0xE4; // float32
                pub const M_DRIVE_DAMPING_SLERP: usize = 0xE8; // float32
                pub const M_SOLVER_ITERATION_COUNT: usize = 0xEC; // int32
                pub const M_PROJECTION_LINEAR_TOLERANCE: usize = 0xF0; // float32
                pub const M_PROJECTION_ANGULAR_TOLERANCE: usize = 0xF4; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_phys_x_constraint2_t {
                pub const M_N_FLAGS: usize = 0x0; // uint32
                pub const M_N_PARENT: usize = 0x4; // uint16
                pub const M_N_CHILD: usize = 0x6; // uint16
                pub const M_PARAMS: usize = 0x8; // VPhysXConstraintParams_t
            }
            // Parent: None
            // Fields count: 24
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_phys_x_joint_t {
                pub const M_N_TYPE: usize = 0x0; // uint16
                pub const M_N_BODY1: usize = 0x2; // uint16
                pub const M_N_BODY2: usize = 0x4; // uint16
                pub const M_N_FLAGS: usize = 0x6; // uint16
                pub const M_FRAME1: usize = 0x10; // CTransform
                pub const M_FRAME2: usize = 0x30; // CTransform
                pub const M_B_ENABLE_COLLISION: usize = 0x50; // bool
                pub const M_B_ENABLE_LINEAR_LIMIT: usize = 0x51; // bool
                pub const M_LINEAR_LIMIT: usize = 0x54; // VPhysXRange_t
                pub const M_B_ENABLE_LINEAR_MOTOR: usize = 0x5C; // bool
                pub const M_V_LINEAR_TARGET_VELOCITY: usize = 0x60; // 
                pub const M_FL_MAX_FORCE: usize = 0x6C; // float32
                pub const M_B_ENABLE_SWING_LIMIT: usize = 0x70; // bool
                pub const M_SWING_LIMIT: usize = 0x74; // VPhysXRange_t
                pub const M_B_ENABLE_TWIST_LIMIT: usize = 0x7C; // bool
                pub const M_TWIST_LIMIT: usize = 0x80; // VPhysXRange_t
                pub const M_B_ENABLE_ANGULAR_MOTOR: usize = 0x88; // bool
                pub const M_V_ANGULAR_TARGET_VELOCITY: usize = 0x8C; // 
                pub const M_FL_MAX_TORQUE: usize = 0x98; // float32
                pub const M_FL_LINEAR_FREQUENCY: usize = 0x9C; // float32
                pub const M_FL_LINEAR_DAMPING_RATIO: usize = 0xA0; // float32
                pub const M_FL_ANGULAR_FREQUENCY: usize = 0xA4; // float32
                pub const M_FL_ANGULAR_DAMPING_RATIO: usize = 0xA8; // float32
                pub const M_FL_FRICTION: usize = 0xAC; // float32
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod phys_softbody_desc_t {
                pub const M_PARTICLE_BONE_HASH: usize = 0x0; // CUtlVector< uint32 >
                pub const M_PARTICLES: usize = 0x18; // CUtlVector< RnSoftbodyParticle_t >
                pub const M_SPRINGS: usize = 0x30; // CUtlVector< RnSoftbodySpring_t >
                pub const M_CAPSULES: usize = 0x48; // CUtlVector< RnSoftbodyCapsule_t >
                pub const M_INIT_POSE: usize = 0x60; // CUtlVector< CTransform >
                pub const M_PARTICLE_BONE_NAME: usize = 0x78; // CUtlVector< CUtlString >
            }
            // Parent: None
            // Fields count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_phys_x_aggregate_data_t {
                pub const M_N_FLAGS: usize = 0x0; // uint16
                pub const M_N_REF_COUNTER: usize = 0x2; // uint16
                pub const M_BONES_HASH: usize = 0x8; // CUtlVector< uint32 >
                pub const M_BONE_NAMES: usize = 0x20; // CUtlVector< CUtlString >
                pub const M_INDEX_NAMES: usize = 0x38; // CUtlVector< uint16 >
                pub const M_INDEX_HASH: usize = 0x50; // CUtlVector< uint16 >
                pub const M_BIND_POSE: usize = 0x68; // CUtlVector< matrix3x4a_t >
                pub const M_PARTS: usize = 0x80; // CUtlVector< VPhysXBodyPart_t >
                pub const M_CONSTRAINTS2: usize = 0x98; // CUtlVector< VPhysXConstraint2_t >
                pub const M_JOINTS: usize = 0xB0; // CUtlVector< VPhysXJoint_t >
                pub const M_P_FE_MODEL: usize = 0xC8; // PhysFeModelDesc_t*
                pub const M_BONE_PARENTS: usize = 0xD0; // CUtlVector< uint16 >
                pub const M_SURFACE_PROPERTY_HASHES: usize = 0xE8; // CUtlVector< uint32 >
                pub const M_COLLISION_ATTRIBUTES: usize = 0x100; // CUtlVector< VPhysXCollisionAttributes_t >
                pub const M_DEBUG_PART_NAMES: usize = 0x118; // CUtlVector< CUtlString >
                pub const M_EMBEDDED_KEYVALUES: usize = 0x130; // CUtlString
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_phys_surface_properties_physics {
                pub const M_FRICTION: usize = 0x0; // float32
                pub const M_ELASTICITY: usize = 0x4; // float32
                pub const M_DENSITY: usize = 0x8; // float32
                pub const M_THICKNESS: usize = 0xC; // float32
                pub const M_SOFT_CONTACT_FREQUENCY: usize = 0x10; // float32
                pub const M_SOFT_CONTACT_DAMPING_RATIO: usize = 0x14; // float32
                pub const M_WHEEL_DRAG: usize = 0x18; // float32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_phys_surface_properties_audio {
                pub const M_REFLECTIVITY: usize = 0x0; // float32
                pub const M_HARDNESS_FACTOR: usize = 0x4; // float32
                pub const M_ROUGHNESS_FACTOR: usize = 0x8; // float32
                pub const M_ROUGH_THRESHOLD: usize = 0xC; // float32
                pub const M_HARD_THRESHOLD: usize = 0x10; // float32
                pub const M_HARD_VELOCITY_THRESHOLD: usize = 0x14; // float32
                pub const M_FL_STATIC_IMPACT_VOLUME: usize = 0x18; // float32
                pub const M_FL_OCCLUSION_FACTOR: usize = 0x1C; // float32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_phys_surface_properties_sound_names {
                pub const M_IMPACT_SOFT: usize = 0x0; // CUtlString
                pub const M_IMPACT_HARD: usize = 0x8; // CUtlString
                pub const M_SCRAPE_SMOOTH: usize = 0x10; // CUtlString
                pub const M_SCRAPE_ROUGH: usize = 0x18; // CUtlString
                pub const M_BULLET_IMPACT: usize = 0x20; // CUtlString
                pub const M_ROLLING: usize = 0x28; // CUtlString
                pub const M_BREAK: usize = 0x30; // CUtlString
                pub const M_STRAIN: usize = 0x38; // CUtlString
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_phys_surface_properties {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_NAME_HASH: usize = 0x8; // uint32
                pub const M_BASE_NAME_HASH: usize = 0xC; // uint32
                pub const M_B_HIDDEN: usize = 0x18; // bool
                pub const M_DESCRIPTION: usize = 0x20; // CUtlString
                pub const M_PHYSICS: usize = 0x28; // CPhysSurfacePropertiesPhysics
                pub const M_AUDIO_SOUNDS: usize = 0x48; // CPhysSurfacePropertiesSoundNames
                pub const M_AUDIO_PARAMS: usize = 0x88; // CPhysSurfacePropertiesAudio
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cv_phys_x_surface_properties_list {
                pub const M_SURFACE_PROPERTIES_LIST: usize = 0x0; // CUtlVector< CPhysSurfaceProperties* >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_group_t {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_MATERIALS: usize = 0x8; // CUtlVector< CStrongHandle< InfoForResourceTypeIMaterial2 > >
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod model_skeleton_data_t {
                pub const M_BONE_NAME: usize = 0x0; // CUtlVector< CUtlString >
                pub const M_N_PARENT: usize = 0x18; // CUtlVector< int16 >
                pub const M_BONE_SPHERE: usize = 0x30; // CUtlVector< float32 >
                pub const M_N_FLAG: usize = 0x48; // CUtlVector< uint32 >
                pub const M_BONE_POS_PARENT: usize = 0x60; // CUtlVector< Vector >
                pub const M_BONE_ROT_PARENT: usize = 0x78; // CUtlVector< QuaternionStorage >
                pub const M_BONE_SCALE_PARENT: usize = 0x90; // CUtlVector< float32 >
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod perm_model_info_t {
                pub const M_N_FLAGS: usize = 0x0; // uint32
                pub const M_V_HULL_MIN: usize = 0x4; // 
                pub const M_V_HULL_MAX: usize = 0x10; // 
                pub const M_V_VIEW_MIN: usize = 0x1C; // 
                pub const M_V_VIEW_MAX: usize = 0x28; // 
                pub const M_FL_MASS: usize = 0x34; // float32
                pub const M_V_EYE_POSITION: usize = 0x38; // 
                pub const M_FL_MAX_EYE_DEFLECTION: usize = 0x44; // float32
                pub const M_S_SURFACE_PROPERTY: usize = 0x48; // CUtlString
                pub const M_KEY_VALUE_TEXT: usize = 0x50; // CUtlString
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod perm_model_ext_part_t {
                pub const M_TRANSFORM: usize = 0x0; // CTransform
                pub const M_NAME: usize = 0x20; // CUtlString
                pub const M_N_PARENT: usize = 0x28; // int32
                pub const M_REF_MODEL: usize = 0x30; // CStrongHandle< InfoForResourceTypeCModel >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod model_bone_flex_driver_control_t {
                pub const M_N_BONE_COMPONENT: usize = 0x0; // ModelBoneFlexComponent_t
                pub const M_FLEX_CONTROLLER: usize = 0x8; // CUtlString
                pub const M_FLEX_CONTROLLER_TOKEN: usize = 0x10; // uint32
                pub const M_FL_MIN: usize = 0x14; // float32
                pub const M_FL_MAX: usize = 0x18; // float32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod model_bone_flex_driver_t {
                pub const M_BONE_NAME: usize = 0x0; // CUtlString
                pub const M_BONE_NAME_TOKEN: usize = 0x8; // uint32
                pub const M_CONTROLS: usize = 0x10; // CUtlVector< ModelBoneFlexDriverControl_t >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod perm_model_data_animated_material_attribute_t {
                pub const M_ATTRIBUTE_NAME: usize = 0x0; // CUtlString
                pub const M_N_NUM_CHANNELS: usize = 0x8; // int32
            }
            // Parent: None
            // Fields count: 23
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod perm_model_data_t {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_MODEL_INFO: usize = 0x8; // PermModelInfo_t
                pub const M_EXT_PARTS: usize = 0x60; // CUtlVector< PermModelExtPart_t >
                pub const M_REF_MESHES: usize = 0x78; // CUtlVector< CStrongHandle< InfoForResourceTypeCRenderMesh > >
                pub const M_REF_MESH_GROUP_MASKS: usize = 0x90; // CUtlVector< uint64 >
                pub const M_REF_PHYS_GROUP_MASKS: usize = 0xA8; // CUtlVector< uint64 >
                pub const M_REF_LOD_GROUP_MASKS: usize = 0xC0; // CUtlVector< uint8 >
                pub const M_LOD_GROUP_SWITCH_DISTANCES: usize = 0xD8; // CUtlVector< float32 >
                pub const M_REF_PHYSICS_DATA: usize = 0xF0; // CUtlVector< CStrongHandle< InfoForResourceTypeCPhysAggregateData > >
                pub const M_REF_PHYSICS_HITBOX_DATA: usize = 0x108; // CUtlVector< CStrongHandle< InfoForResourceTypeCPhysAggregateData > >
                pub const M_REF_ANIM_GROUPS: usize = 0x120; // CUtlVector< CStrongHandle< InfoForResourceTypeCAnimationGroup > >
                pub const M_REF_SEQUENCE_GROUPS: usize = 0x138; // CUtlVector< CStrongHandle< InfoForResourceTypeCSequenceGroupData > >
                pub const M_MESH_GROUPS: usize = 0x150; // CUtlVector< CUtlString >
                pub const M_MATERIAL_GROUPS: usize = 0x168; // CUtlVector< MaterialGroup_t >
                pub const M_N_DEFAULT_MESH_GROUP_MASK: usize = 0x180; // uint64
                pub const M_MODEL_SKELETON: usize = 0x188; // ModelSkeletonData_t
                pub const M_REMAPPING_TABLE: usize = 0x230; // CUtlVector< int16 >
                pub const M_REMAPPING_TABLE_STARTS: usize = 0x248; // CUtlVector< uint16 >
                pub const M_BONE_FLEX_DRIVERS: usize = 0x260; // CUtlVector< ModelBoneFlexDriver_t >
                pub const M_P_MODEL_CONFIG_LIST: usize = 0x278; // CModelConfigList*
                pub const M_BODY_GROUPS_HIDDEN_IN_TOOLS: usize = 0x280; // CUtlVector< CUtlString >
                pub const M_REF_ANIM_INCLUDE_MODELS: usize = 0x298; // CUtlVector< CStrongHandle< InfoForResourceTypeCModel > >
                pub const M_ANIMATED_MATERIAL_ATTRIBUTES: usize = 0x2B0; // CUtlVector< PermModelDataAnimatedMaterialAttribute_t >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element {
                pub const M_ELEMENT_NAME: usize = 0x8; // CUtlString
                pub const M_NESTED_ELEMENTS: usize = 0x10; // CUtlVector< CModelConfigElement* >
            }
            // Parent: c_model_config_element
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element_attached_model {
                pub const M_INSTANCE_NAME: usize = 0x48; // CUtlString
                pub const M_ENTITY_CLASS: usize = 0x50; // CUtlString
                pub const M_H_MODEL: usize = 0x58; // CStrongHandle< InfoForResourceTypeCModel >
                pub const M_V_OFFSET: usize = 0x60; // 
                pub const M_A_ANG_OFFSET: usize = 0x6C; // QAngle
                pub const M_ATTACHMENT_NAME: usize = 0x78; // CUtlString
                pub const M_LOCAL_ATTACHMENT_OFFSET_NAME: usize = 0x80; // CUtlString
                pub const M_ATTACHMENT_TYPE: usize = 0x88; // ModelConfigAttachmentType_t
                pub const M_B_BONE_MERGE_FLEX: usize = 0x8C; // bool
                pub const M_B_USER_SPECIFIED_COLOR: usize = 0x8D; // bool
                pub const M_B_USER_SPECIFIED_MATERIAL_GROUP: usize = 0x8E; // bool
                pub const M_B_ACCEPT_PARENT_MATERIAL_DRIVEN_DECALS: usize = 0x8F; // bool
                pub const M_BODYGROUP_ON_OTHER_MODELS: usize = 0x90; // CUtlString
                pub const M_MATERIAL_GROUP_ON_OTHER_MODELS: usize = 0x98; // CUtlString
            }
            // Parent: c_model_config_element
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element_user_pick {
                pub const M_CHOICES: usize = 0x48; // CUtlVector< CUtlString >
            }
            // Parent: c_model_config_element
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element_random_pick {
                pub const M_CHOICES: usize = 0x48; // CUtlVector< CUtlString >
                pub const M_CHOICE_WEIGHTS: usize = 0x60; // CUtlVector< float32 >
            }
            // Parent: c_model_config_element
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element_set_material_group {
                pub const M_MATERIAL_GROUP_NAME: usize = 0x48; // CUtlString
            }
            // Parent: c_model_config_element
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element_set_material_group_on_attached_models {
                pub const M_MATERIAL_GROUP_NAME: usize = 0x48; // CUtlString
            }
            // Parent: c_model_config_element
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element_set_render_color {
                pub const M_COLOR: usize = 0x48; // Color
            }
            // Parent: c_model_config_element
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element_random_color {
                pub const M_GRADIENT: usize = 0x48; // CColorGradient
            }
            // Parent: c_model_config_element
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element_set_bodygroup {
                pub const M_GROUP_NAME: usize = 0x48; // CUtlString
                pub const M_N_CHOICE: usize = 0x50; // int32
            }
            // Parent: c_model_config_element
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element_set_bodygroup_on_attached_models {
                pub const M_GROUP_NAME: usize = 0x48; // CUtlString
                pub const M_N_CHOICE: usize = 0x50; // int32
            }
            // Parent: c_model_config_element
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_element_command {
                pub const M_COMMAND: usize = 0x48; // CUtlString
                pub const M_ARGS: usize = 0x50; // KeyValues3
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config {
                pub const M_CONFIG_NAME: usize = 0x0; // CUtlString
                pub const M_ELEMENTS: usize = 0x8; // CUtlVector< CModelConfigElement* >
                pub const M_B_TOP_LEVEL: usize = 0x20; // bool
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_model_config_list {
                pub const M_B_HIDE_MATERIAL_GROUP_IN_TOOLS: usize = 0x0; // bool
                pub const M_B_HIDE_RENDER_COLOR_IN_TOOLS: usize = 0x1; // bool
                pub const M_CONFIGS: usize = 0x8; // CUtlVector< CModelConfig* >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_render_buffer_binding {
                pub const M_H_BUFFER: usize = 0x0; // uint64
                pub const M_N_BIND_OFFSET_BYTES: usize = 0x10; // uint32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod skeleton_bone_bounds_t {
                pub const M_VEC_CENTER: usize = 0x0; // 
                pub const M_VEC_SIZE: usize = 0xC; // 
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod render_skeleton_bone_t {
                pub const M_BONE_NAME: usize = 0x0; // CUtlString
                pub const M_PARENT_NAME: usize = 0x8; // CUtlString
                pub const M_INV_BIND_POSE: usize = 0x10; // matrix3x4_t
                pub const M_BBOX: usize = 0x40; // SkeletonBoneBounds_t
                pub const M_FL_SPHERE_RADIUS: usize = 0x58; // float32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_render_skeleton {
                pub const M_BONES: usize = 0x0; // CUtlVector< RenderSkeletonBone_t >
                pub const M_BONE_PARENTS: usize = 0x30; // CUtlVector< int32 >
                pub const M_N_BONE_WEIGHT_COUNT: usize = 0x48; // int32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_draw_culling_data {
                pub const M_V_CONE_APEX: usize = 0x0; // 
                pub const M_CONE_AXIS: usize = 0xC; // int8[3]
                pub const M_CONE_CUTOFF: usize = 0xF; // int8
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_material_draw_descriptor {
                pub const M_N_PRIMITIVE_TYPE: usize = 0x0; // RenderPrimitiveType_t
                pub const M_N_BASE_VERTEX: usize = 0x4; // int32
                pub const M_N_VERTEX_COUNT: usize = 0x8; // int32
                pub const M_N_START_INDEX: usize = 0xC; // int32
                pub const M_N_INDEX_COUNT: usize = 0x10; // int32
                pub const M_FL_UV_DENSITY: usize = 0x14; // float32
                pub const M_V_TINT_COLOR: usize = 0x18; // 
                pub const M_FL_ALPHA: usize = 0x24; // float32
                pub const M_N_FIRST_MESHLET: usize = 0x2C; // uint32
                pub const M_N_NUM_MESHLETS: usize = 0x30; // uint16
                pub const M_INDEX_BUFFER: usize = 0xB8; // CRenderBufferBinding
                pub const M_MATERIAL: usize = 0xE0; // CStrongHandle< InfoForResourceTypeIMaterial2 >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_meshlet_descriptor {
                pub const M_PACKED_AABB: usize = 0x0; // PackedAABB_t
                pub const M_CULLING_DATA: usize = 0x8; // CDrawCullingData
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_scene_object_data {
                pub const M_V_MIN_BOUNDS: usize = 0x0; // 
                pub const M_V_MAX_BOUNDS: usize = 0xC; // 
                pub const M_DRAW_CALLS: usize = 0x18; // CUtlVector< CMaterialDrawDescriptor >
                pub const M_DRAW_BOUNDS: usize = 0x30; // CUtlVector< AABB_t >
                pub const M_MESHLETS: usize = 0x48; // CUtlVector< CMeshletDescriptor >
                pub const M_V_TINT_COLOR: usize = 0x60; // Vector4D
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_attachment {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_INFLUENCE_NAMES: usize = 0x8; // CUtlString[3]
                pub const M_V_INFLUENCE_ROTATIONS: usize = 0x20; // Quaternion[3]
                pub const M_V_INFLUENCE_OFFSETS: usize = 0x50; // Vector[3]
                pub const M_INFLUENCE_WEIGHTS: usize = 0x74; // float32[3]
                pub const M_B_INFLUENCE_ROOT_TRANSFORM: usize = 0x80; // bool[3]
                pub const M_N_INFLUENCES: usize = 0x83; // uint8
                pub const M_B_IGNORE_ROTATION: usize = 0x84; // bool
            }
            // Parent: None
            // Fields count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_hit_box {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_S_SURFACE_PROPERTY: usize = 0x8; // CUtlString
                pub const M_S_BONE_NAME: usize = 0x10; // CUtlString
                pub const M_V_MIN_BOUNDS: usize = 0x18; // 
                pub const M_V_MAX_BOUNDS: usize = 0x24; // 
                pub const M_FL_SHAPE_RADIUS: usize = 0x30; // float32
                pub const M_N_BONE_NAME_HASH: usize = 0x34; // uint32
                pub const M_N_GROUP_ID: usize = 0x38; // int32
                pub const M_N_SHAPE_TYPE: usize = 0x3C; // uint8
                pub const M_B_TRANSLATION_ONLY: usize = 0x3D; // bool
                pub const M_CRC: usize = 0x40; // uint32
                pub const M_C_RENDER_COLOR: usize = 0x44; // Color
                pub const M_N_HIT_BOX_INDEX: usize = 0x48; // uint16
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_hit_box_set {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_N_NAME_HASH: usize = 0x8; // uint32
                pub const M_HIT_BOXES: usize = 0x10; // CUtlVector< CHitBox >
                pub const M_SOURCE_FILENAME: usize = 0x28; // CUtlString
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_hit_box_set_list {
                pub const M_HIT_BOX_SETS: usize = 0x0; // CUtlVector< CHitBoxSet >
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod dynamic_mesh_deform_params_t {
                pub const M_FL_TENSION_COMPRESS_SCALE: usize = 0x0; // float32
                pub const M_FL_TENSION_STRETCH_SCALE: usize = 0x4; // float32
                pub const M_B_RECOMPUTE_SMOOTH_NORMALS_AFTER_ANIMATION: usize = 0x8; // bool
                pub const M_B_COMPUTE_DYNAMIC_MESH_TENSION_AFTER_ANIMATION: usize = 0x9; // bool
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod render_hair_strand_info_t {
                pub const M_N_GUIDE_IDX: usize = 0x0; // uint32[3]
                pub const M_N_BASE_TRI_IDX: usize = 0xC; // uint32
                pub const M_V_GUIDE_BARYCENTRIC: usize = 0x10; // Vector2D
                pub const M_V_BASE_BARYCENTRIC: usize = 0x18; // Vector2D
                pub const M_V_ROOT_OFFSET_FL_LENGTH_SCALE: usize = 0x20; // uint16[4]
                pub const M_N_PACKED_BASE_MESH_UV: usize = 0x28; // uint32
                pub const M_N_PAD: usize = 0x2C; // uint32
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_render_groom {
                pub const M_N_SEGMENTS_PER_HAIR_STRAND: usize = 0x0; // int32
                pub const M_N_GUIDE_HAIR_COUNT: usize = 0x4; // int32
                pub const M_N_HAIR_COUNT: usize = 0x8; // int32
                pub const M_N_GROOM_GROUP_ID: usize = 0xC; // int32
                pub const M_N_ATTACH_BONE_IDX: usize = 0x10; // int32
                pub const M_HAIR_INFO_BUFFER_DATA: usize = 0x18; // CUtlBinaryBlock
                pub const M_HAIRS: usize = 0x30; // CUtlVector< RenderHairStrandInfo_t >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_render_mesh {
                pub const M_SCENE_OBJECTS: usize = 0x10; // CUtlVectorFixedGrowable< CSceneObjectData >
                pub const M_CONSTRAINTS: usize = 0xA0; // CUtlVector< CBaseConstraint* >
                pub const M_SKELETON: usize = 0xB8; // CRenderSkeleton
                pub const M_MESH_DEFORM_PARAMS: usize = 0x1E0; // DynamicMeshDeformParams_t
                pub const M_P_GROOM_DATA: usize = 0x1F0; // CRenderGroom*
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_constraint_target {
                pub const M_Q_OFFSET: usize = 0x20; // Quaternion
                pub const M_V_OFFSET: usize = 0x30; // 
                pub const M_N_BONE_HASH: usize = 0x3C; // uint32
                pub const M_S_NAME: usize = 0x40; // CUtlString
                pub const M_FL_WEIGHT: usize = 0x48; // float32
                pub const M_B_IS_ATTACHMENT: usize = 0x59; // bool
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_constraint_slave {
                pub const M_Q_BASE_ORIENTATION: usize = 0x0; // Quaternion
                pub const M_V_BASE_POSITION: usize = 0x10; // 
                pub const M_N_BONE_HASH: usize = 0x1C; // uint32
                pub const M_FL_WEIGHT: usize = 0x20; // float32
                pub const M_S_NAME: usize = 0x28; // CUtlString
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_bone_constraint_base {
            }
            // Parent: c_bone_constraint_base
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_base_constraint {
                pub const M_NAME: usize = 0x28; // CUtlString
                pub const M_V_UP_VECTOR: usize = 0x30; // 
                pub const M_SLAVES: usize = 0x40; // CUtlVector< CConstraintSlave >
                pub const M_TARGETS: usize = 0x58; // CUtlVector< CConstraintTarget >
            }
            // Parent: c_base_constraint
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_point_constraint {
            }
            // Parent: c_base_constraint
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_orient_constraint {
            }
            // Parent: c_base_constraint
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_aim_constraint {
                pub const M_Q_AIM_OFFSET: usize = 0x70; // Quaternion
                pub const M_N_UP_TYPE: usize = 0x80; // uint32
            }
            // Parent: c_base_constraint
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_twist_constraint {
                pub const M_B_INVERSE: usize = 0x70; // bool
                pub const M_Q_PARENT_BIND_ROTATION: usize = 0x80; // Quaternion
                pub const M_Q_CHILD_BIND_ROTATION: usize = 0x90; // Quaternion
            }
            // Parent: c_base_constraint
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_tilt_twist_constraint {
                pub const M_N_TARGET_AXIS: usize = 0x70; // int32
                pub const M_N_SLAVE_AXIS: usize = 0x74; // int32
            }
            // Parent: c_base_constraint
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_morph_constraint {
                pub const M_S_TARGET_MORPH: usize = 0x70; // CUtlString
                pub const M_N_SLAVE_CHANNEL: usize = 0x78; // int32
                pub const M_FL_MIN: usize = 0x7C; // float32
                pub const M_FL_MAX: usize = 0x80; // float32
            }
            // Parent: c_base_constraint
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_parent_constraint {
            }
            // Parent: c_bone_constraint_base
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_bone_constraint_pose_space_morph {
                pub const M_S_BONE_NAME: usize = 0x28; // CUtlString
                pub const M_S_ATTACHMENT_NAME: usize = 0x30; // CUtlString
                pub const M_OUTPUT_MORPH: usize = 0x38; // CUtlVector< CUtlString >
                pub const M_INPUT_LIST: usize = 0x50; // CUtlVector< CBoneConstraintPoseSpaceMorph::Input_t >
                pub const M_B_CLAMP: usize = 0x68; // bool
            }
            // Parent: None
            // Fields count: 2
            pub mod c_bone_constraint_pose_space_morph_input_t {
                pub const M_INPUT_VALUE: usize = 0x0; // 
                pub const M_OUTPUT_WEIGHT_LIST: usize = 0x10; // CUtlVector< float32 >
            }
            // Parent: c_base_constraint
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_bone_constraint_pose_space_bone {
                pub const M_INPUT_LIST: usize = 0x70; // CUtlVector< CBoneConstraintPoseSpaceBone::Input_t >
            }
            // Parent: None
            // Fields count: 2
            pub mod c_bone_constraint_pose_space_bone_input_t {
                pub const M_INPUT_VALUE: usize = 0x0; // 
                pub const M_OUTPUT_TRANSFORM_LIST: usize = 0x10; // CUtlVector< CTransform >
            }
            // Parent: c_bone_constraint_base
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_bone_constraint_dot_to_morph {
                pub const M_S_BONE_NAME: usize = 0x28; // CUtlString
                pub const M_S_TARGET_BONE_NAME: usize = 0x30; // CUtlString
                pub const M_S_MORPH_CHANNEL_NAME: usize = 0x38; // CUtlString
                pub const M_FL_REMAP: usize = 0x40; // float32[4]
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_flex_op {
                pub const M_OP_CODE: usize = 0x0; // FlexOpCode_t
                pub const M_DATA: usize = 0x4; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_flex_rule {
                pub const M_N_FLEX: usize = 0x0; // int32
                pub const M_FLEX_OPS: usize = 0x8; // CUtlVector< CFlexOp >
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_flex_desc {
                pub const M_SZ_FACS: usize = 0x0; // CUtlString
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_flex_controller {
                pub const M_SZ_NAME: usize = 0x0; // CUtlString
                pub const M_SZ_TYPE: usize = 0x8; // CUtlString
                pub const MIN: usize = 0x10; // float32
                pub const MAX: usize = 0x14; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_morph_bundle_data {
                pub const M_FL_U_LEFT_SRC: usize = 0x0; // float32
                pub const M_FL_V_TOP_SRC: usize = 0x4; // float32
                pub const M_OFFSETS: usize = 0x8; // CUtlVector< float32 >
                pub const M_RANGES: usize = 0x20; // CUtlVector< float32 >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_morph_rect_data {
                pub const M_N_X_LEFT_DST: usize = 0x0; // int16
                pub const M_N_Y_TOP_DST: usize = 0x2; // int16
                pub const M_FL_U_WIDTH_SRC: usize = 0x4; // float32
                pub const M_FL_V_HEIGHT_SRC: usize = 0x8; // float32
                pub const M_BUNDLE_DATAS: usize = 0x10; // CUtlVector< CMorphBundleData >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_morph_data {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_MORPH_RECT_DATAS: usize = 0x8; // CUtlVector< CMorphRectData >
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_morph_set_data {
                pub const M_N_WIDTH: usize = 0x10; // int32
                pub const M_N_HEIGHT: usize = 0x14; // int32
                pub const M_BUNDLE_TYPES: usize = 0x18; // CUtlVector< MorphBundleType_t >
                pub const M_MORPH_DATAS: usize = 0x30; // CUtlVector< CMorphData >
                pub const M_P_TEXTURE_ATLAS: usize = 0x48; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_FLEX_DESC: usize = 0x50; // CUtlVector< CFlexDesc >
                pub const M_FLEX_CONTROLLERS: usize = 0x68; // CUtlVector< CFlexController >
                pub const M_FLEX_RULES: usize = 0x80; // CUtlVector< CFlexRule >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_foot {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_V_BALL_OFFSET: usize = 0x8; // 
                pub const M_V_HEEL_OFFSET: usize = 0x14; // 
                pub const M_ANKLE_BONE_INDEX: usize = 0x20; // int32
                pub const M_TOE_BONE_INDEX: usize = 0x24; // int32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_skeleton {
                pub const M_LOCAL_SPACE_TRANSFORMS: usize = 0x10; // CUtlVector< CTransform >
                pub const M_MODEL_SPACE_TRANSFORMS: usize = 0x28; // CUtlVector< CTransform >
                pub const M_BONE_NAMES: usize = 0x40; // CUtlVector< CUtlString >
                pub const M_CHILDREN: usize = 0x58; // CUtlVector< CUtlVector< int32 > >
                pub const M_PARENTS: usize = 0x70; // CUtlVector< int32 >
                pub const M_FEET: usize = 0x88; // CUtlVector< CAnimFoot >
                pub const M_MORPH_NAMES: usize = 0xA0; // CUtlVector< CUtlString >
                pub const M_LOD_BONE_COUNTS: usize = 0xB8; // CUtlVector< int32 >
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_foot_definition {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_ANKLE_BONE_NAME: usize = 0x8; // CUtlString
                pub const M_TOE_BONE_NAME: usize = 0x10; // CUtlString
                pub const M_V_BALL_OFFSET: usize = 0x18; // 
                pub const M_V_HEEL_OFFSET: usize = 0x24; // 
                pub const M_FL_FOOT_LENGTH: usize = 0x30; // float32
                pub const M_FL_BIND_POSE_DIRECTION_MS: usize = 0x34; // float32
                pub const M_FL_TRACE_HEIGHT: usize = 0x38; // float32
                pub const M_FL_TRACE_RADIUS: usize = 0x3C; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_cycle_base {
                pub const M_FL_CYCLE: usize = 0x0; // float32
            }
            // Parent: c_cycle_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_cycle {
            }
            // Parent: c_cycle_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_foot_cycle {
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_foot_cycle_definition {
                pub const M_V_STANCE_POSITION_MS: usize = 0x0; // 
                pub const M_V_MIDPOINT_POSITION_MS: usize = 0xC; // 
                pub const M_FL_STANCE_DIRECTION_MS: usize = 0x18; // float32
                pub const M_V_TO_STRIDE_START_POS: usize = 0x1C; // 
                pub const M_STANCE_CYCLE: usize = 0x28; // CAnimCycle
                pub const M_FOOT_LIFT_CYCLE: usize = 0x2C; // CFootCycle
                pub const M_FOOT_OFF_CYCLE: usize = 0x30; // CFootCycle
                pub const M_FOOT_STRIKE_CYCLE: usize = 0x34; // CFootCycle
                pub const M_FOOT_LAND_CYCLE: usize = 0x38; // CFootCycle
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_foot_trajectory {
                pub const M_V_OFFSET: usize = 0x0; // 
                pub const M_FL_ROTATION_OFFSET: usize = 0xC; // float32
                pub const M_FL_PROGRESSION: usize = 0x10; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_foot_trajectories {
                pub const M_TRAJECTORIES: usize = 0x0; // CUtlVector< CFootTrajectory >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_foot_stride {
                pub const M_DEFINITION: usize = 0x0; // CFootCycleDefinition
                pub const M_TRAJECTORIES: usize = 0x40; // CFootTrajectories
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_foot_motion {
                pub const M_STRIDES: usize = 0x0; // CUtlVector< CFootStride >
                pub const M_NAME: usize = 0x18; // CUtlString
                pub const M_B_ADDITIVE: usize = 0x20; // bool
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod skeleton_anim_capture_t {
                pub const M_N_ENT_INDEX: usize = 0x0; // CEntityIndex
                pub const M_N_ENT_PARENT: usize = 0x4; // CEntityIndex
                pub const M_IMPORTED_COLLISION: usize = 0x8; // CUtlVector< CEntityIndex >
                pub const M_MODEL_NAME: usize = 0x20; // CUtlString
                pub const M_CAPTURE_NAME: usize = 0x28; // CUtlString
                pub const M_MODEL_BIND_POSE: usize = 0x30; // CUtlVector< SkeletonAnimCapture_t::Bone_t >
                pub const M_FE_MODEL_INIT_POSE: usize = 0x48; // CUtlVector< SkeletonAnimCapture_t::Bone_t >
                pub const M_N_FLEX_CONTROLLERS: usize = 0x60; // int32
                pub const M_B_PREDICTED: usize = 0x64; // bool
                pub const M_FRAMES: usize = 0xA8; // CUtlVector< SkeletonAnimCapture_t::Frame_t >
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod skeleton_anim_capture_t_bone_t {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_BIND_POSE: usize = 0x10; // CTransform
                pub const M_N_PARENT: usize = 0x30; // int32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod skeleton_anim_capture_t_frame_stamp_t {
                pub const M_FL_TIME: usize = 0x0; // float32
                pub const M_FL_ENTITY_SIM_TIME: usize = 0x4; // float32
                pub const M_B_TELEPORT_TICK: usize = 0x8; // bool
                pub const M_B_PREDICTED: usize = 0x9; // bool
                pub const M_FL_CUR_TIME: usize = 0xC; // float32
                pub const M_FL_REAL_TIME: usize = 0x10; // float32
                pub const M_N_FRAME_COUNT: usize = 0x14; // int32
                pub const M_N_TICK_COUNT: usize = 0x18; // int32
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod skeleton_anim_capture_t_frame_t {
                pub const M_FL_TIME: usize = 0x0; // float32
                pub const M_STAMP: usize = 0x4; // SkeletonAnimCapture_t::FrameStamp_t
                pub const M_TRANSFORM: usize = 0x20; // CTransform
                pub const M_B_TELEPORT: usize = 0x40; // bool
                pub const M_COMPOSITE_BONES: usize = 0x48; // CUtlVector< CTransform >
                pub const M_SIM_STATE_BONES: usize = 0x60; // CUtlVector< CTransform >
                pub const M_FE_MODEL_ANIMS: usize = 0x78; // CUtlVector< CTransform >
                pub const M_FE_MODEL_POS: usize = 0x90; // CUtlVector< VectorAligned >
                pub const M_FLEX_CONTROLLER_WEIGHTS: usize = 0xA8; // CUtlVector< float32 >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod skeleton_anim_capture_t_camera_t {
                pub const M_TM_CAMERA: usize = 0x0; // CTransform
                pub const M_FL_TIME: usize = 0x20; // float32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod skeleton_demo_db_t {
                pub const M_ANIM_CAPTURES: usize = 0x0; // CUtlVector< SkeletonAnimCapture_t* >
                pub const M_CAMERA_TRACK: usize = 0x18; // CUtlVector< SkeletonAnimCapture_t::Camera_t >
                pub const M_FL_RECORDING_TIME: usize = 0x30; // float32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_bone_mask {
                pub const M_ID: usize = 0x0; // CGlobalSymbol
                pub const M_WEIGHT_INFO: usize = 0x8; // CNmBoneMask::WeightInfo_t
                pub const M_WEIGHTS: usize = 0x18; // CUtlVector< float32 >
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_clip {
                pub const M_SKELETON: usize = 0x0; // CStrongHandle< InfoForResourceTypeCNmSkeleton >
                pub const M_N_NUM_FRAMES: usize = 0x8; // uint32
                pub const M_FL_DURATION: usize = 0xC; // float32
                pub const M_COMPRESSED_POSE_DATA: usize = 0x10; // CUtlBinaryBlock
                pub const M_TRACK_COMPRESSION_SETTINGS: usize = 0x28; // CUtlVector< NmCompressionSettings_t >
                pub const M_COMPRESSED_POSE_OFFSETS: usize = 0x40; // CUtlVector< uint32 >
                pub const M_SYNC_TRACK: usize = 0xA0; // CNmSyncTrack
                pub const M_ROOT_MOTION: usize = 0x150; // CNmRootMotionData
                pub const M_B_IS_ADDITIVE: usize = 0x1A0; // bool
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod nm_compression_settings_t {
                pub const M_TRANSLATION_RANGE_X: usize = 0x0; // NmCompressionSettings_t::QuantizationRange_t
                pub const M_TRANSLATION_RANGE_Y: usize = 0x8; // NmCompressionSettings_t::QuantizationRange_t
                pub const M_TRANSLATION_RANGE_Z: usize = 0x10; // NmCompressionSettings_t::QuantizationRange_t
                pub const M_SCALE_RANGE: usize = 0x18; // NmCompressionSettings_t::QuantizationRange_t
                pub const M_CONSTANT_ROTATION: usize = 0x20; // Quaternion
                pub const M_B_IS_ROTATION_STATIC: usize = 0x30; // bool
                pub const M_B_IS_TRANSLATION_STATIC: usize = 0x31; // bool
                pub const M_B_IS_SCALE_STATIC: usize = 0x32; // bool
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod nm_compression_settings_t_quantization_range_t {
                pub const M_FL_RANGE_START: usize = 0x0; // float32
                pub const M_FL_RANGE_LENGTH: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_event {
                pub const M_FL_START_TIME: usize = 0x8; // float32
                pub const M_FL_DURATION: usize = 0xC; // float32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_root_motion_data {
                pub const M_TRANSFORMS: usize = 0x0; // CUtlVector< CTransform >
                pub const M_N_NUM_FRAMES: usize = 0x18; // int32
                pub const M_FL_AVERAGE_LINEAR_VELOCITY: usize = 0x1C; // float32
                pub const M_FL_AVERAGE_ANGULAR_VELOCITY_RADIANS: usize = 0x20; // float32
                pub const M_TOTAL_DELTA: usize = 0x30; // CTransform
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_skeleton {
                pub const M_ID: usize = 0x0; // CGlobalSymbol
                pub const M_BONE_I_DS: usize = 0x8; // CUtlLeanVector< CGlobalSymbol >
                pub const M_PARENT_INDICES: usize = 0x18; // CUtlVector< int32 >
                pub const M_PARENT_SPACE_REFERENCE_POSE: usize = 0x30; // CUtlVector< CTransform >
                pub const M_MODEL_SPACE_REFERENCE_POSE: usize = 0x48; // CUtlVector< CTransform >
                pub const M_NUM_BONES_TO_SAMPLE_AT_LOW_LOD: usize = 0x60; // int32
                pub const M_BONE_MASKS: usize = 0x68; // CUtlLeanVector< CNmBoneMask >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod nm_sync_track_time_t {
                pub const M_N_EVENT_IDX: usize = 0x0; // int32
                pub const M_PERCENTAGE_THROUGH: usize = 0x4; // NmPercent_t
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod nm_sync_track_time_range_t {
                pub const M_START_TIME: usize = 0x0; // NmSyncTrackTime_t
                pub const M_END_TIME: usize = 0x8; // NmSyncTrackTime_t
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_sync_track {
                pub const M_SYNC_EVENTS: usize = 0x0; // CUtlLeanVectorFixedGrowable< CNmSyncTrack::Event_t >
                pub const M_N_START_EVENT_OFFSET: usize = 0xA8; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_sync_track_event_marker_t {
                pub const M_START_TIME: usize = 0x0; // NmPercent_t
                pub const M_ID: usize = 0x8; // CGlobalSymbol
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_sync_track_event_t {
                pub const M_ID: usize = 0x0; // CGlobalSymbol
                pub const M_START_TIME: usize = 0x8; // NmPercent_t
                pub const M_DURATION: usize = 0xC; // NmPercent_t
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod nm_percent_t {
                pub const M_FL_VALUE: usize = 0x0; // float32
            }
            // Parent: c_nm_event
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_legacy_event {
            }
            // Parent: c_nm_event
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_foot_event {
                pub const M_PHASE: usize = 0x10; // NmFootPhase_t
            }
            // Parent: c_nm_event
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_id_event {
                pub const M_ID: usize = 0x10; // CGlobalSymbol
            }
            // Parent: c_nm_event
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_transition_event {
                pub const M_RULE: usize = 0x10; // NmTransitionRule_t
                pub const M_ID: usize = 0x18; // CGlobalSymbol
            }
            // Parent: c_nm_event
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_frame_snap_event {
                pub const M_FRAME_SNAP_MODE: usize = 0x10; // NmFrameSnapEventMode_t
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_graph_data_set {
                pub const M_VARIATION_ID: usize = 0x0; // CGlobalSymbol
                pub const M_SKELETON: usize = 0x8; // CStrongHandle< InfoForResourceTypeCNmSkeleton >
                pub const M_RESOURCES: usize = 0x10; // CUtlVector< CStrongHandleVoid >
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_graph {
                pub const M_PERSISTENT_NODE_INDICES: usize = 0x0; // CUtlVector< int16 >
                pub const M_INSTANCE_NODE_START_OFFSETS: usize = 0x18; // CUtlVector< uint32 >
                pub const M_INSTANCE_REQUIRED_MEMORY: usize = 0x30; // uint32
                pub const M_INSTANCE_REQUIRED_ALIGNMENT: usize = 0x34; // uint32
                pub const M_ROOT_NODE_IDX: usize = 0x38; // int16
                pub const M_CONTROL_PARAMETER_I_DS: usize = 0x40; // CUtlVector< CGlobalSymbol >
                pub const M_VIRTUAL_PARAMETER_I_DS: usize = 0x58; // CUtlVector< CGlobalSymbol >
                pub const M_VIRTUAL_PARAMETER_NODE_INDICES: usize = 0x70; // CUtlVector< int16 >
                pub const M_CHILD_GRAPH_SLOTS: usize = 0x88; // CUtlVector< CNmGraph::ChildGraphSlot_t >
                pub const M_EXTERNAL_GRAPH_SLOTS: usize = 0xA0; // CUtlVector< CNmGraph::ExternalGraphSlot_t >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_graph_external_graph_slot_t {
                pub const M_N_NODE_IDX: usize = 0x0; // int16
                pub const M_SLOT_ID: usize = 0x8; // CGlobalSymbol
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_graph_child_graph_slot_t {
                pub const M_N_NODE_IDX: usize = 0x0; // int16
                pub const M_DATA_SLOT_IDX: usize = 0x2; // int16
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nm_graph_variation {
                pub const M_GRAPH: usize = 0x0; // CStrongHandle< InfoForResourceTypeCNmGraph >
                pub const M_DATA_SET: usize = 0x8; // CStrongHandle< InfoForResourceTypeCNmGraphDataSet >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod bone_demo_capture_settings_t {
                pub const M_BONE_NAME: usize = 0x0; // CUtlString
                pub const M_FL_CHAIN_LENGTH: usize = 0x8; // float32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod ik_demo_capture_settings_t {
                pub const M_PARENT_BONE_NAME: usize = 0x0; // CUtlString
                pub const M_E_MODE: usize = 0x8; // IKChannelMode
                pub const M_IK_CHAIN_NAME: usize = 0x10; // CUtlString
                pub const M_ONE_BONE_START: usize = 0x18; // CUtlString
                pub const M_ONE_BONE_END: usize = 0x20; // CUtlString
            }
            // Parent: None
            // Fields count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_demo_capture_settings {
                pub const M_RANGE_BONE_CHAIN_LENGTH: usize = 0x0; // Vector2D
                pub const M_RANGE_MAX_SPLINE_ERROR_ROTATION: usize = 0x8; // Vector2D
                pub const M_FL_MAX_SPLINE_ERROR_TRANSLATION: usize = 0x10; // float32
                pub const M_FL_MAX_SPLINE_ERROR_SCALE: usize = 0x14; // float32
                pub const M_FL_IK_ROTATION_MAX_SPLINE_ERROR: usize = 0x18; // float32
                pub const M_FL_IK_TRANSLATION_MAX_SPLINE_ERROR: usize = 0x1C; // float32
                pub const M_FL_MAX_QUANTIZATION_ERROR_ROTATION: usize = 0x20; // float32
                pub const M_FL_MAX_QUANTIZATION_ERROR_TRANSLATION: usize = 0x24; // float32
                pub const M_FL_MAX_QUANTIZATION_ERROR_SCALE: usize = 0x28; // float32
                pub const M_FL_IK_ROTATION_MAX_QUANTIZATION_ERROR: usize = 0x2C; // float32
                pub const M_FL_IK_TRANSLATION_MAX_QUANTIZATION_ERROR: usize = 0x30; // float32
                pub const M_BASE_SEQUENCE: usize = 0x38; // CUtlString
                pub const M_N_BASE_SEQUENCE_FRAME: usize = 0x40; // int32
                pub const M_BONE_SELECTION_MODE: usize = 0x44; // EDemoBoneSelectionMode
                pub const M_BONES: usize = 0x48; // CUtlVector< BoneDemoCaptureSettings_t >
                pub const M_IK_CHAINS: usize = 0x60; // CUtlVector< IKDemoCaptureSettings_t >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_replay_frame {
                pub const M_INPUT_DATA_BLOCKS: usize = 0x10; // CUtlVector< CUtlBinaryBlock >
                pub const M_INSTANCE_DATA: usize = 0x28; // CUtlBinaryBlock
                pub const M_STARTING_LOCAL_TO_WORLD_TRANSFORM: usize = 0x40; // CTransform
                pub const M_LOCAL_TO_WORLD_TRANSFORM: usize = 0x60; // CTransform
                pub const M_TIME_STAMP: usize = 0x80; // float32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_graph_debug_replay {
                pub const M_ANIM_GRAPH_FILE_NAME: usize = 0x40; // CUtlString
                pub const M_FRAME_LIST: usize = 0x48; // CUtlVector< CSmartPtr< CAnimReplayFrame > >
                pub const M_START_INDEX: usize = 0x60; // int32
                pub const M_WRITE_INDEX: usize = 0x64; // int32
                pub const M_FRAME_COUNT: usize = 0x68; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_graph_model_binding {
                pub const M_MODEL_NAME: usize = 0x8; // CUtlString
                pub const M_P_SHARED_DATA: usize = 0x10; // CSmartPtr< CAnimUpdateSharedData >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_anim_input_damping {
                pub const M_SPEED_FUNCTION: usize = 0x8; // DampingSpeedFunction
                pub const M_F_SPEED_SCALE: usize = 0xC; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_param_handle {
                pub const M_TYPE: usize = 0x0; // AnimParamType_t
                pub const M_INDEX: usize = 0x1; // uint8
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_param_handle_map {
                pub const M_LIST: usize = 0x0; // CUtlHashtable< uint16, int16 >
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_parameter_manager_updater {
                pub const M_PARAMETERS: usize = 0x18; // CUtlVector< CSmartPtr< CAnimParameterBase > >
                pub const M_ID_TO_INDEX_MAP: usize = 0x30; // CUtlHashtable< AnimParamID, int32 >
                pub const M_NAME_TO_INDEX_MAP: usize = 0x50; // CUtlHashtable< CUtlString, int32 >
                pub const M_INDEX_TO_HANDLE: usize = 0x70; // CUtlVector< CAnimParamHandle >
                pub const M_AUTO_RESET_PARAMS: usize = 0x88; // CUtlVector< CUtlPair< CAnimParamHandle, CAnimVariant > >
                pub const M_AUTO_RESET_MAP: usize = 0xA0; // CUtlHashtable< CAnimParamHandle, int16 >
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_update_shared_data {
                pub const M_NODES: usize = 0x10; // CUtlVector< CSmartPtr< CAnimUpdateNodeBase > >
                pub const M_NODE_INDEX_MAP: usize = 0x28; // CUtlHashtable< CAnimNodePath, int32 >
                pub const M_COMPONENTS: usize = 0x48; // CUtlVector< CSmartPtr< CAnimComponentUpdater > >
                pub const M_P_PARAM_LIST_UPDATER: usize = 0x60; // CSmartPtr< CAnimParameterManagerUpdater >
                pub const M_P_TAG_MANAGER_UPDATER: usize = 0x68; // CSmartPtr< CAnimTagManagerUpdater >
                pub const M_SCRIPT_MANAGER: usize = 0x70; // CSmartPtr< CAnimScriptManager >
                pub const M_SETTINGS: usize = 0x78; // CAnimGraphSettingsManager
                pub const M_P_STATIC_POSE_CACHE: usize = 0xA8; // CSmartPtr< CStaticPoseCacheBuilder >
                pub const M_P_SKELETON: usize = 0xB0; // CSmartPtr< CAnimSkeleton >
                pub const M_ROOT_NODE_PATH: usize = 0xB8; // CAnimNodePath
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_blend_curve {
                pub const M_FL_CONTROL_POINT1: usize = 0x0; // float32
                pub const M_FL_CONTROL_POINT2: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod param_span_sample_t {
                pub const M_VALUE: usize = 0x0; // CAnimVariant
                pub const M_FL_CYCLE: usize = 0x14; // float32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod param_span_t {
                pub const M_SAMPLES: usize = 0x0; // CUtlVector< ParamSpanSample_t >
                pub const M_H_PARAM: usize = 0x18; // CAnimParamHandle
                pub const M_E_PARAM_TYPE: usize = 0x1A; // AnimParamType_t
                pub const M_FL_START_CYCLE: usize = 0x1C; // float32
                pub const M_FL_END_CYCLE: usize = 0x20; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_param_span_updater {
                pub const M_SPANS: usize = 0x0; // CUtlVector< ParamSpan_t >
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_graph_settings_group {
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_graph_settings_manager {
                pub const M_SETTINGS_GROUPS: usize = 0x18; // CUtlVector< CSmartPtr< CAnimGraphSettingsGroup > >
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_cached_pose {
                pub const M_TRANSFORMS: usize = 0x8; // CUtlVector< CTransform >
                pub const M_MORPH_WEIGHTS: usize = 0x20; // CUtlVector< float32 >
                pub const M_H_SEQUENCE: usize = 0x38; // HSequence
                pub const M_FL_CYCLE: usize = 0x3C; // float32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_static_pose_cache {
                pub const M_POSES: usize = 0x10; // CUtlVector< CCachedPose >
                pub const M_N_BONE_COUNT: usize = 0x28; // int32
                pub const M_N_MORPH_COUNT: usize = 0x2C; // int32
            }
            // Parent: c_static_pose_cache
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_static_pose_cache_builder {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_action_updater {
            }
            // Parent: c_anim_action_updater
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_emit_tag_action_updater {
                pub const M_N_TAG_INDEX: usize = 0x18; // int32
                pub const M_B_IS_ZERO_DURATION: usize = 0x1C; // bool
            }
            // Parent: c_anim_action_updater
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_set_parameter_action_updater {
                pub const M_H_PARAM: usize = 0x18; // CAnimParamHandle
                pub const M_VALUE: usize = 0x1A; // CAnimVariant
            }
            // Parent: c_anim_action_updater
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_toggle_component_action_updater {
                pub const M_COMPONENT_ID: usize = 0x18; // AnimComponentID
                pub const M_B_SET_ENABLED: usize = 0x1C; // bool
            }
            // Parent: c_anim_action_updater
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_expression_action_updater {
                pub const M_H_PARAM: usize = 0x18; // CAnimParamHandle
                pub const M_E_PARAM_TYPE: usize = 0x1A; // AnimParamType_t
                pub const M_H_SCRIPT: usize = 0x1C; // AnimScriptHandle
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_tag_base {
                pub const M_NAME: usize = 0x18; // CGlobalSymbol
                pub const M_GROUP: usize = 0x20; // CGlobalSymbol
                pub const M_TAG_ID: usize = 0x28; // AnimTagID
                pub const M_B_IS_REFERENCED: usize = 0x40; // bool
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_tag_manager_updater {
                pub const M_TAGS: usize = 0x18; // CUtlVector< CSmartPtr< CAnimTagBase > >
            }
            // Parent: c_anim_tag_base
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_audio_anim_tag {
                pub const M_CLIP_NAME: usize = 0x50; // CUtlString
                pub const M_ATTACHMENT_NAME: usize = 0x58; // CUtlString
                pub const M_FL_VOLUME: usize = 0x60; // float32
                pub const M_B_STOP_WHEN_TAG_ENDS: usize = 0x64; // bool
                pub const M_B_STOP_WHEN_GRAPH_ENDS: usize = 0x65; // bool
                pub const M_B_PLAY_ON_SERVER: usize = 0x66; // bool
                pub const M_B_PLAY_ON_CLIENT: usize = 0x67; // bool
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyElementNameFn
            pub mod c_body_group_setting {
                pub const M_BODY_GROUP_NAME: usize = 0x0; // CUtlString
                pub const M_N_BODY_GROUP_OPTION: usize = 0x8; // int32
            }
            // Parent: c_anim_tag_base
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_body_group_anim_tag {
                pub const M_N_PRIORITY: usize = 0x50; // int32
                pub const M_BODY_GROUP_SETTINGS: usize = 0x58; // CUtlVector< CBodyGroupSetting >
            }
            // Parent: c_anim_tag_base
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_cloth_settings_anim_tag {
                pub const M_FL_STIFFNESS: usize = 0x50; // float32
                pub const M_FL_EASE_IN: usize = 0x54; // float32
                pub const M_FL_EASE_OUT: usize = 0x58; // float32
                pub const M_N_VERTEX_SET: usize = 0x60; // CUtlString
            }
            // Parent: c_anim_tag_base
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_foot_fall_anim_tag {
                pub const M_FOOT: usize = 0x50; // FootFallTagFoot_t
            }
            // Parent: c_anim_tag_base
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_footstep_landed_anim_tag {
                pub const M_FOOTSTEP_TYPE: usize = 0x50; // FootstepLandedFootSoundType_t
                pub const M_OVERRIDE_SOUND_NAME: usize = 0x58; // CUtlString
                pub const M_DEBUG_ANIM_SOURCE_STRING: usize = 0x60; // CUtlString
                pub const M_BONE_NAME: usize = 0x68; // CUtlString
            }
            // Parent: c_anim_tag_base
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_material_attribute_anim_tag {
                pub const M_ATTRIBUTE_NAME: usize = 0x50; // CUtlString
                pub const M_ATTRIBUTE_TYPE: usize = 0x58; // MatterialAttributeTagType_t
                pub const M_FL_VALUE: usize = 0x5C; // float32
                pub const M_COLOR: usize = 0x60; // Color
            }
            // Parent: c_anim_tag_base
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_particle_anim_tag {
                pub const M_H_PARTICLE_SYSTEM: usize = 0x50; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
                pub const M_PARTICLE_SYSTEM_NAME: usize = 0x58; // CUtlString
                pub const M_CONFIG_NAME: usize = 0x60; // CUtlString
                pub const M_B_DETACH_FROM_OWNER: usize = 0x68; // bool
                pub const M_B_STOP_WHEN_TAG_ENDS: usize = 0x69; // bool
                pub const M_B_TAG_END_STOP_IS_INSTANT: usize = 0x6A; // bool
                pub const M_ATTACHMENT_NAME: usize = 0x70; // CUtlString
                pub const M_ATTACHMENT_TYPE: usize = 0x78; // ParticleAttachment_t
                pub const M_ATTACHMENT_CP1_NAME: usize = 0x80; // CUtlString
                pub const M_ATTACHMENT_CP1_TYPE: usize = 0x88; // ParticleAttachment_t
            }
            // Parent: c_anim_tag_base
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_ragdoll_anim_tag {
                pub const M_N_POSE_CONTROL: usize = 0x50; // AnimPoseControl
                pub const M_FL_FREQUENCY: usize = 0x54; // float32
                pub const M_FL_DAMPING_RATIO: usize = 0x58; // float32
                pub const M_FL_DECAY_DURATION: usize = 0x5C; // float32
                pub const M_FL_DECAY_BIAS: usize = 0x60; // float32
                pub const M_B_DESTROY: usize = 0x64; // bool
            }
            // Parent: c_anim_tag_base
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_sequence_finished_anim_tag {
                pub const M_SEQUENCE_NAME: usize = 0x50; // CUtlString
            }
            // Parent: c_anim_tag_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_string_anim_tag {
            }
            // Parent: c_anim_tag_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod c_task_status_anim_tag {
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_component_updater {
                pub const M_NAME: usize = 0x18; // CUtlString
                pub const M_ID: usize = 0x20; // AnimComponentID
                pub const M_NETWORK_MODE: usize = 0x24; // AnimNodeNetworkMode
                pub const M_B_START_ENABLED: usize = 0x28; // bool
            }
            // Parent: c_anim_component_updater
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_script_component_updater {
                pub const M_H_SCRIPT: usize = 0x30; // AnimScriptHandle
            }
            // Parent: c_anim_component_updater
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod ccpp_script_component_updater {
                pub const M_SCRIPTS_TO_RUN: usize = 0x30; // CUtlVector< CGlobalSymbol >
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_damped_value_update_item {
                pub const M_DAMPING: usize = 0x0; // CAnimInputDamping
                pub const M_H_PARAM_IN: usize = 0x18; // CAnimParamHandle
                pub const M_H_PARAM_OUT: usize = 0x1A; // CAnimParamHandle
            }
            // Parent: c_anim_component_updater
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_damped_value_component_updater {
                pub const M_ITEMS: usize = 0x30; // CUtlVector< CDampedValueUpdateItem >
            }
            // Parent: c_anim_component_updater
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_demo_settings_component_updater {
                pub const M_SETTINGS: usize = 0x30; // CAnimDemoCaptureSettings
            }
            // Parent: c_anim_component_updater
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod clod_component_updater {
                pub const M_N_SERVER_LOD: usize = 0x30; // int32
            }
            // Parent: c_anim_component_updater
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_look_component_updater {
                pub const M_H_LOOK_HEADING: usize = 0x34; // CAnimParamHandle
                pub const M_H_LOOK_HEADING_VELOCITY: usize = 0x36; // CAnimParamHandle
                pub const M_H_LOOK_PITCH: usize = 0x38; // CAnimParamHandle
                pub const M_H_LOOK_DISTANCE: usize = 0x3A; // CAnimParamHandle
                pub const M_H_LOOK_DIRECTION: usize = 0x3C; // CAnimParamHandle
                pub const M_H_LOOK_TARGET: usize = 0x3E; // CAnimParamHandle
                pub const M_H_LOOK_TARGET_WORLD_SPACE: usize = 0x40; // CAnimParamHandle
                pub const M_B_NETWORK_LOOK_TARGET: usize = 0x42; // bool
            }
            // Parent: c_anim_component_updater
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_movement_component_updater {
                pub const M_MOTORS: usize = 0x30; // CUtlVector< CSmartPtr< CAnimMotorUpdaterBase > >
                pub const M_FACING_DAMPING: usize = 0x48; // CAnimInputDamping
                pub const M_E_DEFAULT_FACING_MODE: usize = 0x58; // FacingMode
                pub const M_N_DEFAULT_MOTOR_INDEX: usize = 0x64; // int32
                pub const M_FL_DEFAULT_RUN_SPEED: usize = 0x68; // float32
                pub const M_B_MOVE_VARS_DISABLED: usize = 0x6C; // bool
                pub const M_B_NETWORK_PATH: usize = 0x6D; // bool
                pub const M_B_NETWORK_FACING: usize = 0x6E; // bool
                pub const M_PARAM_HANDLES: usize = 0x6F; // CAnimParamHandle[30]
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod weight_list {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_WEIGHTS: usize = 0x8; // CUtlVector< float32 >
            }
            // Parent: c_anim_component_updater
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_ragdoll_component_updater {
                pub const M_RAGDOLL_NODE_PATHS: usize = 0x30; // CUtlVector< CAnimNodePath >
                pub const M_BONE_INDICES: usize = 0x48; // CUtlVector< int32 >
                pub const M_BONE_NAMES: usize = 0x60; // CUtlVector< CUtlString >
                pub const M_WEIGHT_LISTS: usize = 0x78; // CUtlVector< WeightList >
                pub const M_FL_SPRING_FREQUENCY_MIN: usize = 0x90; // float32
                pub const M_FL_SPRING_FREQUENCY_MAX: usize = 0x94; // float32
                pub const M_FL_MAX_STRETCH: usize = 0x98; // float32
                pub const M_B_SOLID_COLLISION_AT_ZERO_WEIGHT: usize = 0x9C; // bool
            }
            // Parent: c_anim_component_updater
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_slope_component_updater {
                pub const M_FL_TRACE_DISTANCE: usize = 0x34; // float32
                pub const M_H_SLOPE_ANGLE: usize = 0x38; // CAnimParamHandle
                pub const M_H_SLOPE_ANGLE_FRONT: usize = 0x3A; // CAnimParamHandle
                pub const M_H_SLOPE_ANGLE_SIDE: usize = 0x3C; // CAnimParamHandle
                pub const M_H_SLOPE_HEADING: usize = 0x3E; // CAnimParamHandle
                pub const M_H_SLOPE_NORMAL: usize = 0x40; // CAnimParamHandle
                pub const M_H_SLOPE_NORMAL_WORLD_SPACE: usize = 0x42; // CAnimParamHandle
            }
            // Parent: c_anim_component_updater
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_state_machine_component_updater {
                pub const M_STATE_MACHINE: usize = 0x30; // CAnimStateMachineUpdater
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_motion_data_set {
                pub const M_GROUPS: usize = 0x0; // CUtlVector< CMotionGraphGroup >
                pub const M_N_DIMENSION_COUNT: usize = 0x18; // int32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_motion_graph_group {
                pub const M_SEARCH_DB: usize = 0x0; // CMotionSearchDB
                pub const M_MOTION_GRAPHS: usize = 0xB8; // CUtlVector< CSmartPtr< CMotionGraph > >
                pub const M_MOTION_GRAPH_CONFIGS: usize = 0xD0; // CUtlVector< CMotionGraphConfig >
                pub const M_SAMPLE_TO_CONFIG: usize = 0xE8; // CUtlVector< int32 >
                pub const M_H_IS_ACTIVE_SCRIPT: usize = 0x100; // AnimScriptHandle
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod sample_code {
                pub const M_SUB_CODE: usize = 0x0; // uint8[8]
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod motion_db_index {
                pub const M_N_INDEX: usize = 0x0; // uint32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_vector_quantizer {
                pub const M_CENTROID_VECTORS: usize = 0x0; // CUtlVector< float32 >
                pub const M_N_CENTROIDS: usize = 0x18; // int32
                pub const M_N_DIMENSIONS: usize = 0x1C; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_product_quantizer {
                pub const M_SUB_QUANTIZERS: usize = 0x0; // CUtlVector< CVectorQuantizer >
                pub const M_N_DIMENSIONS: usize = 0x18; // int32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_motion_search_node {
                pub const M_CHILDREN: usize = 0x0; // CUtlVector< CMotionSearchNode* >
                pub const M_QUANTIZER: usize = 0x18; // CVectorQuantizer
                pub const M_SAMPLE_CODES: usize = 0x38; // CUtlVector< CUtlVector< SampleCode > >
                pub const M_SAMPLE_INDICES: usize = 0x50; // CUtlVector< CUtlVector< int32 > >
                pub const M_SELECTABLE_SAMPLES: usize = 0x68; // CUtlVector< int32 >
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_motion_search_db {
                pub const M_ROOT_NODE: usize = 0x0; // CMotionSearchNode
                pub const M_RESIDUAL_QUANTIZER: usize = 0x80; // CProductQuantizer
                pub const M_CODE_INDICES: usize = 0xA0; // CUtlVector< MotionDBIndex >
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_motion_graph {
                pub const M_PARAM_SPANS: usize = 0x10; // CParamSpanUpdater
                pub const M_TAGS: usize = 0x28; // CUtlVector< TagSpan_t >
                pub const M_P_ROOT_NODE: usize = 0x40; // CSmartPtr< CMotionNode >
                pub const M_N_PARAMETER_COUNT: usize = 0x48; // int32
                pub const M_N_CONFIG_START_INDEX: usize = 0x4C; // int32
                pub const M_N_CONFIG_COUNT: usize = 0x50; // int32
                pub const M_B_LOOP: usize = 0x54; // bool
            }
            // Parent: c_motion_graph
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_editable_motion_graph {
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_motion_node {
                pub const M_NAME: usize = 0x18; // CUtlString
                pub const M_ID: usize = 0x20; // AnimNodeID
            }
            // Parent: c_motion_node
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_motion_node_sequence {
                pub const M_TAGS: usize = 0x28; // CUtlVector< TagSpan_t >
                pub const M_H_SEQUENCE: usize = 0x40; // HSequence
                pub const M_FL_PLAYBACK_SPEED: usize = 0x44; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod motion_blend_item {
                pub const M_P_CHILD: usize = 0x0; // CSmartPtr< CMotionNode >
                pub const M_FL_KEY_VALUE: usize = 0x8; // float32
            }
            // Parent: c_motion_node
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_motion_node_blend1_d {
                pub const M_BLEND_ITEMS: usize = 0x28; // CUtlVector< MotionBlendItem >
                pub const M_N_PARAM_INDEX: usize = 0x40; // int32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_motion_metric_evaluator {
                pub const M_MEANS: usize = 0x18; // CUtlVector< float32 >
                pub const M_STANDARD_DEVIATIONS: usize = 0x30; // CUtlVector< float32 >
                pub const M_FL_WEIGHT: usize = 0x48; // float32
                pub const M_N_DIMENSION_START_INDEX: usize = 0x4C; // int32
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_block_selection_metric_evaluator {
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_bone_position_metric_evaluator {
                pub const M_N_BONE_INDEX: usize = 0x50; // int32
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_bone_velocity_metric_evaluator {
                pub const M_N_BONE_INDEX: usize = 0x50; // int32
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_current_rotation_velocity_metric_evaluator {
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_current_velocity_metric_evaluator {
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_distance_remaining_metric_evaluator {
                pub const M_FL_MAX_DISTANCE: usize = 0x50; // float32
                pub const M_FL_MIN_DISTANCE: usize = 0x54; // float32
                pub const M_FL_START_GOAL_FILTER_DISTANCE: usize = 0x58; // float32
                pub const M_FL_MAX_GOAL_OVERSHOOT_SCALE: usize = 0x5C; // float32
                pub const M_B_FILTER_FIXED_MIN_DISTANCE: usize = 0x60; // bool
                pub const M_B_FILTER_GOAL_DISTANCE: usize = 0x61; // bool
                pub const M_B_FILTER_GOAL_OVERSHOOT: usize = 0x62; // bool
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_foot_cycle_metric_evaluator {
                pub const M_FOOT_INDICES: usize = 0x50; // CUtlVector< int32 >
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_foot_position_metric_evaluator {
                pub const M_FOOT_INDICES: usize = 0x50; // CUtlVector< int32 >
                pub const M_B_IGNORE_SLOPE: usize = 0x68; // bool
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_future_facing_metric_evaluator {
                pub const M_FL_DISTANCE: usize = 0x50; // float32
                pub const M_FL_TIME: usize = 0x54; // float32
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_future_velocity_metric_evaluator {
                pub const M_FL_DISTANCE: usize = 0x50; // float32
                pub const M_FL_STOPPING_DISTANCE: usize = 0x54; // float32
                pub const M_FL_TARGET_SPEED: usize = 0x58; // float32
                pub const M_E_MODE: usize = 0x5C; // VelocityMetricMode
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_path_metric_evaluator {
                pub const M_PATH_TIME_SAMPLES: usize = 0x50; // CUtlVector< float32 >
                pub const M_FL_DISTANCE: usize = 0x68; // float32
                pub const M_B_EXTRAPOLATE_MOVEMENT: usize = 0x6C; // bool
                pub const M_FL_MIN_EXTRAPOLATION_SPEED: usize = 0x70; // float32
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_steps_remaining_metric_evaluator {
                pub const M_FOOT_INDICES: usize = 0x50; // CUtlVector< int32 >
                pub const M_FL_MIN_STEPS_REMAINING: usize = 0x68; // float32
            }
            // Parent: c_motion_metric_evaluator
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_time_remaining_metric_evaluator {
                pub const M_B_MATCH_BY_TIME_REMAINING: usize = 0x50; // bool
                pub const M_FL_MAX_TIME_REMAINING: usize = 0x54; // float32
                pub const M_B_FILTER_BY_TIME_REMAINING: usize = 0x58; // bool
                pub const M_FL_MIN_TIME_REMAINING: usize = 0x5C; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_anim_motor_updater_base {
                pub const M_NAME: usize = 0x10; // CUtlString
                pub const M_B_DEFAULT: usize = 0x18; // bool
            }
            // Parent: c_anim_motor_updater_base
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_path_anim_motor_updater_base {
                pub const M_B_LOCK_TO_PATH: usize = 0x20; // bool
            }
            // Parent: c_path_anim_motor_updater_base
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_damped_path_anim_motor_updater {
                pub const M_FL_ANTICIPATION_TIME: usize = 0x2C; // float32
                pub const M_FL_MIN_SPEED_SCALE: usize = 0x30; // float32
                pub const M_H_ANTICIPATION_POS_PARAM: usize = 0x34; // CAnimParamHandle
                pub const M_H_ANTICIPATION_HEADING_PARAM: usize = 0x36; // CAnimParamHandle
                pub const M_FL_SPRING_CONSTANT: usize = 0x38; // float32
                pub const M_FL_MIN_SPRING_TENSION: usize = 0x3C; // float32
                pub const M_FL_MAX_SPRING_TENSION: usize = 0x40; // float32
            }
            // Parent: c_path_anim_motor_updater_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_path_anim_motor_updater {
            }
            // Parent: c_anim_motor_updater_base
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_player_input_anim_motor_updater {
                pub const M_SAMPLE_TIMES: usize = 0x20; // CUtlVector< float32 >
                pub const M_FL_SPRING_CONSTANT: usize = 0x3C; // float32
                pub const M_FL_ANTICIPATION_DISTANCE: usize = 0x40; // float32
                pub const M_H_ANTICIPATION_POS_PARAM: usize = 0x44; // CAnimParamHandle
                pub const M_H_ANTICIPATION_HEADING_PARAM: usize = 0x46; // CAnimParamHandle
                pub const M_B_USE_ACCELERATION: usize = 0x48; // bool
            }
        }
        // Module: client.dll
        // Classes count: 256
        // Enums count: 8
        pub mod client {
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum CBaseCombatCharacterWaterWakeModeT {
                WaterWakeNone = 0,
                WaterWakeIdle = 1,
                WaterWakeWalking = 2,
                WaterWakeRunning = 3,
                WaterWakeWaterOverhead = 4
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum CompositeMaterialMatchFilterTypeT {
                MatchFilterMaterialAttributeExists = 0,
                MatchFilterMaterialShader = 1,
                MatchFilterMaterialNameSubstr = 2,
                MatchFilterMaterialAttributeEquals = 3,
                MatchFilterMaterialPropertyExists = 4,
                MatchFilterMaterialPropertyEquals = 5
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum CompositeMaterialVarSystemVarT {
                CompmatsysvarCompositetime = 0,
                CompmatsysvarEmptyResourceSpacer = 1
            }
            // Alignment: 4
            // Members count: 14
            #[repr(i32)]
            pub enum CompositeMaterialInputLooseVariableTypeT {
                LooseVariableTypeBoolean = 0,
                LooseVariableTypeInteger1 = 1,
                LooseVariableTypeInteger2 = 2,
                LooseVariableTypeInteger3 = 3,
                LooseVariableTypeInteger4 = 4,
                LooseVariableTypeFloat1 = 5,
                LooseVariableTypeFloat2 = 6,
                LooseVariableTypeFloat3 = 7,
                LooseVariableTypeFloat4 = 8,
                LooseVariableTypeColor4 = 9,
                LooseVariableTypeString = 10,
                LooseVariableTypeSystemvar = 11,
                LooseVariableTypeResourceMaterial = 12,
                LooseVariableTypeResourceTexture = 13
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum CompositeMaterialInputTextureTypeT {
                InputTextureTypeDefault = 0,
                InputTextureTypeNormalmap = 1,
                InputTextureTypeColor = 2,
                InputTextureTypeMasks = 3,
                InputTextureTypeRoughness = 4,
                InputTextureTypePearlescenceMask = 5,
                InputTextureTypeAo = 6
            }
            // Alignment: 4
            // Members count: 10
            #[repr(i32)]
            pub enum CompMatPropertyMutatorTypeT {
                CompMatPropertyMutatorInit = 0,
                CompMatPropertyMutatorCopyMatchingKeys = 1,
                CompMatPropertyMutatorCopyKeysWithSuffix = 2,
                CompMatPropertyMutatorCopyProperty = 3,
                CompMatPropertyMutatorSetValue = 4,
                CompMatPropertyMutatorGenerateTexture = 5,
                CompMatPropertyMutatorConditionalMutators = 6,
                CompMatPropertyMutatorPopInputQueue = 7,
                CompMatPropertyMutatorDrawText = 8,
                CompMatPropertyMutatorRandomRollInputVariables = 9
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum CompMatPropertyMutatorConditionTypeT {
                CompMatMutatorConditionInputContainerExists = 0,
                CompMatMutatorConditionInputContainerValueExists = 1,
                CompMatMutatorConditionInputContainerValueEquals = 2
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum CompositeMaterialInputContainerSourceTypeT {
                ContainerSourceTypeTargetMaterial = 0,
                ContainerSourceTypeMaterialFromTargetAttr = 1,
                ContainerSourceTypeSpecificMaterial = 2,
                ContainerSourceTypeLooseVariables = 3,
                ContainerSourceTypeVariableFromTargetAttr = 4,
                ContainerSourceTypeTargetInstanceMaterial = 5
            }
            // Parent: None
            // Fields count: 0
            pub mod c_entity_component {
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_nameStringableIndex (int32)
            pub mod c_entity_identity {
                pub const M_NAME_STRINGABLE_INDEX: usize = 0x14; // int32
                pub const M_NAME: usize = 0x18; // CUtlSymbolLarge
                pub const M_DESIGNER_NAME: usize = 0x20; // CUtlSymbolLarge
                pub const M_FLAGS: usize = 0x30; // uint32
                pub const M_WORLD_GROUP_ID: usize = 0x38; // WorldGroupId_t
                pub const M_F_DATA_OBJECT_TYPES: usize = 0x3C; // uint32
                pub const M_PATH_INDEX: usize = 0x40; // ChangeAccessorFieldPathIndex_t
                pub const M_P_PREV: usize = 0x58; // CEntityIdentity*
                pub const M_P_NEXT: usize = 0x60; // CEntityIdentity*
                pub const M_P_PREV_BY_CLASS: usize = 0x68; // CEntityIdentity*
                pub const M_P_NEXT_BY_CLASS: usize = 0x70; // CEntityIdentity*
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_pEntity (CEntityIdentity *)
            // NetworkVarNames: m_CScriptComponent (CScriptComponent::Storage_t)
            pub mod c_entity_instance {
                pub const M_ISZ_PRIVATE_V_SCRIPTS: usize = 0x8; // CUtlSymbolLarge
                pub const M_P_ENTITY: usize = 0x10; // CEntityIdentity*
                pub const M_C_SCRIPT_COMPONENT: usize = 0x28; // CScriptComponent*
                pub const M_B_VISIBLEIN_PVS: usize = 0x30; // bool
            }
            // Parent: c_entity_component
            // Fields count: 1
            pub mod c_script_component {
                pub const M_SCRIPT_CLASS_NAME: usize = 0x30; // CUtlSymbolLarge
            }
            // Parent: c_entity_component
            // Fields count: 2
            pub mod c_body_component {
                pub const M_P_SCENE_NODE: usize = 0x8; // CGameSceneNode*
                pub const M_P_CHAIN_ENTITY: usize = 0x20; // CNetworkVarChainer
            }
            // Parent: c_body_component
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_sceneNode (CGameSceneNode)
            pub mod c_body_component_point {
                pub const M_SCENE_NODE: usize = 0x50; // CGameSceneNode
                pub const M_P_CHAIN_ENTITY: usize = 0x1A0; // CNetworkVarChainer
            }
            // Parent: c_body_component
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_skeletonInstance (CSkeletonInstance)
            pub mod c_body_component_skeleton_instance {
                pub const M_SKELETON_INSTANCE: usize = 0x50; // CSkeletonInstance
                pub const M_P_CHAIN_ENTITY: usize = 0x430; // CNetworkVarChainer
            }
            // Parent: c_entity_component
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_bvDisabledHitGroups (uint32)
            pub mod c_hitbox_component {
                pub const M_BV_DISABLED_HIT_GROUPS: usize = 0x24; // uint32[1]
            }
            // Parent: c_entity_component
            // Fields count: 66
            //
            // Metadata:
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_SecondaryColor (Color)
            // NetworkVarNames: m_flBrightness (float)
            // NetworkVarNames: m_flBrightnessScale (float)
            // NetworkVarNames: m_flBrightnessMult (float)
            // NetworkVarNames: m_flRange (float)
            // NetworkVarNames: m_flFalloff (float)
            // NetworkVarNames: m_flAttenuation0 (float)
            // NetworkVarNames: m_flAttenuation1 (float)
            // NetworkVarNames: m_flAttenuation2 (float)
            // NetworkVarNames: m_flTheta (float)
            // NetworkVarNames: m_flPhi (float)
            // NetworkVarNames: m_hLightCookie (HRenderTextureStrong)
            // NetworkVarNames: m_nCascades (int)
            // NetworkVarNames: m_nCastShadows (int)
            // NetworkVarNames: m_nShadowWidth (int)
            // NetworkVarNames: m_nShadowHeight (int)
            // NetworkVarNames: m_bRenderDiffuse (bool)
            // NetworkVarNames: m_nRenderSpecular (int)
            // NetworkVarNames: m_bRenderTransmissive (bool)
            // NetworkVarNames: m_flOrthoLightWidth (float)
            // NetworkVarNames: m_flOrthoLightHeight (float)
            // NetworkVarNames: m_nStyle (int)
            // NetworkVarNames: m_Pattern (CUtlString)
            // NetworkVarNames: m_nCascadeRenderStaticObjects (int)
            // NetworkVarNames: m_flShadowCascadeCrossFade (float)
            // NetworkVarNames: m_flShadowCascadeDistanceFade (float)
            // NetworkVarNames: m_flShadowCascadeDistance0 (float)
            // NetworkVarNames: m_flShadowCascadeDistance1 (float)
            // NetworkVarNames: m_flShadowCascadeDistance2 (float)
            // NetworkVarNames: m_flShadowCascadeDistance3 (float)
            // NetworkVarNames: m_nShadowCascadeResolution0 (int)
            // NetworkVarNames: m_nShadowCascadeResolution1 (int)
            // NetworkVarNames: m_nShadowCascadeResolution2 (int)
            // NetworkVarNames: m_nShadowCascadeResolution3 (int)
            // NetworkVarNames: m_bUsesBakedShadowing (bool)
            // NetworkVarNames: m_nShadowPriority (int)
            // NetworkVarNames: m_nBakedShadowIndex (int)
            // NetworkVarNames: m_bRenderToCubemaps (bool)
            // NetworkVarNames: m_nDirectLight (int)
            // NetworkVarNames: m_nIndirectLight (int)
            // NetworkVarNames: m_flFadeMinDist (float)
            // NetworkVarNames: m_flFadeMaxDist (float)
            // NetworkVarNames: m_flShadowFadeMinDist (float)
            // NetworkVarNames: m_flShadowFadeMaxDist (float)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bFlicker (bool)
            // NetworkVarNames: m_bPrecomputedFieldsValid (bool)
            // NetworkVarNames: m_vPrecomputedBoundsMins (Vector)
            // NetworkVarNames: m_vPrecomputedBoundsMaxs (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent (Vector)
            // NetworkVarNames: m_flPrecomputedMaxRange (float)
            // NetworkVarNames: m_nFogLightingMode (int)
            // NetworkVarNames: m_flFogContributionStength (float)
            // NetworkVarNames: m_flNearClipPlane (float)
            // NetworkVarNames: m_SkyColor (Color)
            // NetworkVarNames: m_flSkyIntensity (float)
            // NetworkVarNames: m_SkyAmbientBounce (Color)
            // NetworkVarNames: m_bUseSecondaryColor (bool)
            // NetworkVarNames: m_bMixedShadows (bool)
            // NetworkVarNames: m_flLightStyleStartTime (GameTime_t)
            // NetworkVarNames: m_flCapsuleLength (float)
            // NetworkVarNames: m_flMinRoughness (float)
            pub mod c_light_component {
                pub const M_P_CHAIN_ENTITY: usize = 0x48; // CNetworkVarChainer
                pub const M_COLOR: usize = 0x85; // Color
                pub const M_SECONDARY_COLOR: usize = 0x89; // Color
                pub const M_FL_BRIGHTNESS: usize = 0x90; // float32
                pub const M_FL_BRIGHTNESS_SCALE: usize = 0x94; // float32
                pub const M_FL_BRIGHTNESS_MULT: usize = 0x98; // float32
                pub const M_FL_RANGE: usize = 0x9C; // float32
                pub const M_FL_FALLOFF: usize = 0xA0; // float32
                pub const M_FL_ATTENUATION0: usize = 0xA4; // float32
                pub const M_FL_ATTENUATION1: usize = 0xA8; // float32
                pub const M_FL_ATTENUATION2: usize = 0xAC; // float32
                pub const M_FL_THETA: usize = 0xB0; // float32
                pub const M_FL_PHI: usize = 0xB4; // float32
                pub const M_H_LIGHT_COOKIE: usize = 0xB8; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_N_CASCADES: usize = 0xC0; // int32
                pub const M_N_CAST_SHADOWS: usize = 0xC4; // int32
                pub const M_N_SHADOW_WIDTH: usize = 0xC8; // int32
                pub const M_N_SHADOW_HEIGHT: usize = 0xCC; // int32
                pub const M_B_RENDER_DIFFUSE: usize = 0xD0; // bool
                pub const M_N_RENDER_SPECULAR: usize = 0xD4; // int32
                pub const M_B_RENDER_TRANSMISSIVE: usize = 0xD8; // bool
                pub const M_FL_ORTHO_LIGHT_WIDTH: usize = 0xDC; // float32
                pub const M_FL_ORTHO_LIGHT_HEIGHT: usize = 0xE0; // float32
                pub const M_N_STYLE: usize = 0xE4; // int32
                pub const M_PATTERN: usize = 0xE8; // CUtlString
                pub const M_N_CASCADE_RENDER_STATIC_OBJECTS: usize = 0xF0; // int32
                pub const M_FL_SHADOW_CASCADE_CROSS_FADE: usize = 0xF4; // float32
                pub const M_FL_SHADOW_CASCADE_DISTANCE_FADE: usize = 0xF8; // float32
                pub const M_FL_SHADOW_CASCADE_DISTANCE0: usize = 0xFC; // float32
                pub const M_FL_SHADOW_CASCADE_DISTANCE1: usize = 0x100; // float32
                pub const M_FL_SHADOW_CASCADE_DISTANCE2: usize = 0x104; // float32
                pub const M_FL_SHADOW_CASCADE_DISTANCE3: usize = 0x108; // float32
                pub const M_N_SHADOW_CASCADE_RESOLUTION0: usize = 0x10C; // int32
                pub const M_N_SHADOW_CASCADE_RESOLUTION1: usize = 0x110; // int32
                pub const M_N_SHADOW_CASCADE_RESOLUTION2: usize = 0x114; // int32
                pub const M_N_SHADOW_CASCADE_RESOLUTION3: usize = 0x118; // int32
                pub const M_B_USES_BAKED_SHADOWING: usize = 0x11C; // bool
                pub const M_N_SHADOW_PRIORITY: usize = 0x120; // int32
                pub const M_N_BAKED_SHADOW_INDEX: usize = 0x124; // int32
                pub const M_B_RENDER_TO_CUBEMAPS: usize = 0x128; // bool
                pub const M_N_DIRECT_LIGHT: usize = 0x12C; // int32
                pub const M_N_INDIRECT_LIGHT: usize = 0x130; // int32
                pub const M_FL_FADE_MIN_DIST: usize = 0x134; // float32
                pub const M_FL_FADE_MAX_DIST: usize = 0x138; // float32
                pub const M_FL_SHADOW_FADE_MIN_DIST: usize = 0x13C; // float32
                pub const M_FL_SHADOW_FADE_MAX_DIST: usize = 0x140; // float32
                pub const M_B_ENABLED: usize = 0x144; // bool
                pub const M_B_FLICKER: usize = 0x145; // bool
                pub const M_B_PRECOMPUTED_FIELDS_VALID: usize = 0x146; // bool
                pub const M_V_PRECOMPUTED_BOUNDS_MINS: usize = 0x148; // 
                pub const M_V_PRECOMPUTED_BOUNDS_MAXS: usize = 0x154; // 
                pub const M_V_PRECOMPUTED_OBB_ORIGIN: usize = 0x160; // 
                pub const M_V_PRECOMPUTED_OBB_ANGLES: usize = 0x16C; // QAngle
                pub const M_V_PRECOMPUTED_OBB_EXTENT: usize = 0x178; // 
                pub const M_FL_PRECOMPUTED_MAX_RANGE: usize = 0x184; // float32
                pub const M_N_FOG_LIGHTING_MODE: usize = 0x188; // int32
                pub const M_FL_FOG_CONTRIBUTION_STENGTH: usize = 0x18C; // float32
                pub const M_FL_NEAR_CLIP_PLANE: usize = 0x190; // float32
                pub const M_SKY_COLOR: usize = 0x194; // Color
                pub const M_FL_SKY_INTENSITY: usize = 0x198; // float32
                pub const M_SKY_AMBIENT_BOUNCE: usize = 0x19C; // Color
                pub const M_B_USE_SECONDARY_COLOR: usize = 0x1A0; // bool
                pub const M_B_MIXED_SHADOWS: usize = 0x1A1; // bool
                pub const M_FL_LIGHT_STYLE_START_TIME: usize = 0x1A4; // GameTime_t
                pub const M_FL_CAPSULE_LENGTH: usize = 0x1A8; // float32
                pub const M_FL_MIN_ROUGHNESS: usize = 0x1AC; // float32
            }
            // Parent: c_entity_component
            // Fields count: 5
            pub mod c_render_component {
                pub const M_P_CHAIN_ENTITY: usize = 0x10; // CNetworkVarChainer
                pub const M_B_IS_RENDERING_WITH_VIEW_MODELS: usize = 0x50; // bool
                pub const M_N_SPLITSCREEN_FLAGS: usize = 0x54; // uint32
                pub const M_B_ENABLE_RENDERING: usize = 0x60; // bool
                pub const M_B_INTERPOLATION_READY_TO_DRAW: usize = 0xB0; // bool
            }
            // Parent: None
            // Fields count: 1
            pub mod c_buoyancy_helper {
                pub const M_FL_FLUID_DENSITY: usize = 0x18; // float32
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod c_base_player_controller_api {
            }
            // Parent: None
            // Fields count: 2
            pub mod c_command_context {
                pub const NEEDSPROCESSING: usize = 0x0; // bool
                pub const COMMAND_NUMBER: usize = 0x90; // int32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: nType (FixAngleSet_t)
            // NetworkVarNames: qAngle (QAngle)
            // NetworkVarNames: nIndex (uint32)
            pub mod view_angle_server_change_t {
                pub const N_TYPE: usize = 0x30; // FixAngleSet_t
                pub const Q_ANGLE: usize = 0x34; // QAngle
                pub const N_INDEX: usize = 0x40; // uint32
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod c_dynamic_prop_api {
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_autoaim_services {
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: localSound (Vector)
            // NetworkVarNames: soundscapeIndex (int32)
            // NetworkVarNames: localBits (uint8)
            // NetworkVarNames: soundscapeEntityListIndex (int)
            // NetworkVarNames: soundEventHash (uint32)
            pub mod audioparams_t {
                pub const LOCAL_SOUND: usize = 0x8; // Vector[8]
                pub const SOUNDSCAPE_INDEX: usize = 0x68; // int32
                pub const LOCAL_BITS: usize = 0x6C; // uint8
                pub const SOUNDSCAPE_ENTITY_LIST_INDEX: usize = 0x70; // int32
                pub const SOUND_EVENT_HASH: usize = 0x74; // uint32
            }
            // Parent: c_player_pawn_component
            // Fields count: 20
            //
            // Metadata:
            // NetworkVarNames: m_vecCsViewPunchAngle (QAngle)
            // NetworkVarNames: m_nCsViewPunchAngleTick (GameTick_t)
            // NetworkVarNames: m_flCsViewPunchAngleTickRatio (float32)
            // NetworkVarNames: m_PlayerFog (fogplayerparams_t)
            // NetworkVarNames: m_hColorCorrectionCtrl (CHandle< CColorCorrection>)
            // NetworkVarNames: m_hViewEntity (CHandle< CBaseEntity>)
            // NetworkVarNames: m_hTonemapController (CHandle< CTonemapController2>)
            // NetworkVarNames: m_audio (audioparams_t)
            // NetworkVarNames: m_PostProcessingVolumes (CHandle<C_PostProcessingVolume>)
            pub mod c_player_camera_services {
                pub const M_VEC_CS_VIEW_PUNCH_ANGLE: usize = 0x40; // QAngle
                pub const M_N_CS_VIEW_PUNCH_ANGLE_TICK: usize = 0x4C; // GameTick_t
                pub const M_FL_CS_VIEW_PUNCH_ANGLE_TICK_RATIO: usize = 0x50; // float32
                pub const M_PLAYER_FOG: usize = 0x58; // C_fogplayerparams_t
                pub const M_H_COLOR_CORRECTION_CTRL: usize = 0x98; // CHandle< C_ColorCorrection >
                pub const M_H_VIEW_ENTITY: usize = 0x9C; // CHandle< C_BaseEntity >
                pub const M_H_TONEMAP_CONTROLLER: usize = 0xA0; // CHandle< C_TonemapController2 >
                pub const M_AUDIO: usize = 0xA8; // audioparams_t
                pub const M_POST_PROCESSING_VOLUMES: usize = 0x120; // C_NetworkUtlVectorBase< CHandle< C_PostProcessingVolume > >
                pub const M_FL_OLD_PLAYER_Z: usize = 0x138; // float32
                pub const M_FL_OLD_PLAYER_VIEW_OFFSET_Z: usize = 0x13C; // float32
                pub const M_CURRENT_FOG: usize = 0x140; // fogparams_t
                pub const M_H_OLD_FOG_CONTROLLER: usize = 0x1A8; // CHandle< C_FogController >
                pub const M_B_OVERRIDE_FOG_COLOR: usize = 0x1AC; // bool[5]
                pub const M_OVERRIDE_FOG_COLOR: usize = 0x1B1; // Color[5]
                pub const M_B_OVERRIDE_FOG_START_END: usize = 0x1C5; // bool[5]
                pub const M_F_OVERRIDE_FOG_START: usize = 0x1CC; // float32[5]
                pub const M_F_OVERRIDE_FOG_END: usize = 0x1E0; // float32[5]
                pub const M_H_ACTIVE_POST_PROCESSING_VOLUME: usize = 0x1F4; // CHandle< C_PostProcessingVolume >
                pub const M_ANG_DEMO_VIEW_ANGLES: usize = 0x1F8; // QAngle
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_flashlight_services {
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_item_services {
            }
            // Parent: c_player_pawn_component
            // Fields count: 15
            //
            // Metadata:
            // NetworkVarNames: m_nToggleButtonDownMask (ButtonBitMask_t)
            // NetworkVarNames: m_flMaxspeed (float32)
            // NetworkVarNames: m_arrForceSubtickMoveWhen (float32)
            pub mod c_player_movement_services {
                pub const M_N_IMPULSE: usize = 0x40; // int32
                pub const M_N_BUTTONS: usize = 0x48; // CInButtonState
                pub const M_N_QUEUED_BUTTON_DOWN_MASK: usize = 0x68; // uint64
                pub const M_N_QUEUED_BUTTON_CHANGE_MASK: usize = 0x70; // uint64
                pub const M_N_BUTTON_DOUBLE_PRESSED: usize = 0x78; // uint64
                pub const M_P_BUTTON_PRESSED_CMD_NUMBER: usize = 0x80; // uint32[64]
                pub const M_N_LAST_COMMAND_NUMBER_PROCESSED: usize = 0x180; // uint32
                pub const M_N_TOGGLE_BUTTON_DOWN_MASK: usize = 0x188; // uint64
                pub const M_FL_MAXSPEED: usize = 0x198; // float32
                pub const M_ARR_FORCE_SUBTICK_MOVE_WHEN: usize = 0x19C; // float32[4]
                pub const M_FL_FORWARD_MOVE: usize = 0x1AC; // float32
                pub const M_FL_LEFT_MOVE: usize = 0x1B0; // float32
                pub const M_FL_UP_MOVE: usize = 0x1B4; // float32
                pub const M_VEC_LAST_MOVEMENT_IMPULSES: usize = 0x1B8; // 
                pub const M_VEC_OLD_VIEW_ANGLES: usize = 0x1C4; // QAngle
            }
            // Parent: c_player_movement_services
            // Fields count: 12
            //
            // Metadata:
            // NetworkVarNames: m_flFallVelocity (float32)
            // NetworkVarNames: m_bInCrouch (bool)
            // NetworkVarNames: m_nCrouchState (uint32)
            // NetworkVarNames: m_flCrouchTransitionStartTime (GameTime_t)
            // NetworkVarNames: m_bDucked (bool)
            // NetworkVarNames: m_bDucking (bool)
            // NetworkVarNames: m_bInDuckJump (bool)
            pub mod c_player_movement_services_humanoid {
                pub const M_FL_STEP_SOUND_TIME: usize = 0x1D8; // float32
                pub const M_FL_FALL_VELOCITY: usize = 0x1DC; // float32
                pub const M_B_IN_CROUCH: usize = 0x1E0; // bool
                pub const M_N_CROUCH_STATE: usize = 0x1E4; // uint32
                pub const M_FL_CROUCH_TRANSITION_START_TIME: usize = 0x1E8; // GameTime_t
                pub const M_B_DUCKED: usize = 0x1EC; // bool
                pub const M_B_DUCKING: usize = 0x1ED; // bool
                pub const M_B_IN_DUCK_JUMP: usize = 0x1EE; // bool
                pub const M_GROUND_NORMAL: usize = 0x1F0; // 
                pub const M_FL_SURFACE_FRICTION: usize = 0x1FC; // float32
                pub const M_SURFACE_PROPS: usize = 0x200; // CUtlStringToken
                pub const M_N_STEPSIDE: usize = 0x210; // int32
            }
            // Parent: c_player_pawn_component
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: m_iObserverMode (uint8)
            // NetworkVarNames: m_hObserverTarget (CHandle< CBaseEntity>)
            pub mod c_player_observer_services {
                pub const M_I_OBSERVER_MODE: usize = 0x40; // uint8
                pub const M_H_OBSERVER_TARGET: usize = 0x44; // CHandle< C_BaseEntity >
                pub const M_I_OBSERVER_LAST_MODE: usize = 0x48; // ObserverMode_t
                pub const M_B_FORCED_OBSERVER_MODE: usize = 0x4C; // bool
                pub const M_FL_OBSERVER_CHASE_DISTANCE: usize = 0x50; // float32
                pub const M_FL_OBSERVER_CHASE_DISTANCE_CALC_TIME: usize = 0x54; // GameTime_t
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_use_services {
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_water_services {
            }
            // Parent: c_player_pawn_component
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_hMyWeapons (CHandle< C_BasePlayerWeapon >)
            // NetworkVarNames: m_hActiveWeapon (CHandle< CBasePlayerWeapon>)
            // NetworkVarNames: m_hLastWeapon (CHandle< CBasePlayerWeapon>)
            // NetworkVarNames: m_iAmmo (uint16)
            pub mod c_player_weapon_services {
                pub const M_H_MY_WEAPONS: usize = 0x40; // C_NetworkUtlVectorBase< CHandle< C_BasePlayerWeapon > >
                pub const M_H_ACTIVE_WEAPON: usize = 0x58; // CHandle< C_BasePlayerWeapon >
                pub const M_H_LAST_WEAPON: usize = 0x5C; // CHandle< C_BasePlayerWeapon >
                pub const M_I_AMMO: usize = 0x60; // uint16[32]
            }
            // Parent: c_body_component_skeleton_instance
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_animationController (CBaseAnimGraphController)
            pub mod c_body_component_base_anim_graph {
                pub const M_ANIMATION_CONTROLLER: usize = 0x460; // CBaseAnimGraphController
                pub const M_P_CHAIN_ENTITY: usize = 0x1C28; // CNetworkVarChainer
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_ID (CUtlStringToken)
            // NetworkVarNames: m_Values (Vector4D)
            pub mod entity_render_attribute_t {
                pub const M_ID: usize = 0x30; // CUtlStringToken
                pub const M_VALUES: usize = 0x34; // Vector4D
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_Handle (ModelConfigHandle_t)
            // NetworkVarNames: m_Name (string_t)
            // NetworkVarNames: m_AssociatedEntities (CHandle< C_BaseModelEntity >)
            // NetworkVarNames: m_AssociatedEntityNames (string_t)
            pub mod active_model_config_t {
                pub const M_HANDLE: usize = 0x28; // ModelConfigHandle_t
                pub const M_NAME: usize = 0x30; // CUtlSymbolLarge
                pub const M_ASSOCIATED_ENTITIES: usize = 0x38; // C_NetworkUtlVectorBase< CHandle< C_BaseModelEntity > >
                pub const M_ASSOCIATED_ENTITY_NAMES: usize = 0x50; // C_NetworkUtlVectorBase< CUtlSymbolLarge >
            }
            // Parent: c_body_component_skeleton_instance
            // Fields count: 1
            pub mod c_body_component_base_model_entity {
                pub const M_P_CHAIN_ENTITY: usize = 0x460; // CNetworkVarChainer
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hOwner (CEntityHandle)
            // NetworkVarNames: m_name (CUtlStringToken)
            pub mod c_game_scene_node_handle {
                pub const M_H_OWNER: usize = 0x8; // CEntityHandle
                pub const M_NAME: usize = 0xC; // CUtlStringToken
            }
            // Parent: None
            // Fields count: 32
            //
            // Metadata:
            // NetworkVarNames: m_hParent (CGameSceneNodeHandle)
            // NetworkVarNames: m_vecOrigin (CNetworkOriginCellCoordQuantizedVector)
            // NetworkVarNames: m_angRotation (QAngle)
            // NetworkVarNames: m_flScale (float)
            // NetworkVarNames: m_name (CUtlStringToken)
            // NetworkVarNames: m_hierarchyAttachName (CUtlStringToken)
            pub mod c_game_scene_node {
                pub const M_NODE_TO_WORLD: usize = 0x10; // CTransform
                pub const M_P_OWNER: usize = 0x30; // CEntityInstance*
                pub const M_P_PARENT: usize = 0x38; // CGameSceneNode*
                pub const M_P_CHILD: usize = 0x40; // CGameSceneNode*
                pub const M_P_NEXT_SIBLING: usize = 0x48; // CGameSceneNode*
                pub const M_H_PARENT: usize = 0x70; // CGameSceneNodeHandle
                pub const M_VEC_ORIGIN: usize = 0x80; // CNetworkOriginCellCoordQuantizedVector
                pub const M_ANG_ROTATION: usize = 0xB8; // QAngle
                pub const M_FL_SCALE: usize = 0xC4; // float32
                pub const M_VEC_ABS_ORIGIN: usize = 0xC8; // 
                pub const M_ANG_ABS_ROTATION: usize = 0xD4; // QAngle
                pub const M_FL_ABS_SCALE: usize = 0xE0; // float32
                pub const M_N_PARENT_ATTACHMENT_OR_BONE: usize = 0xE4; // int16
                pub const M_B_DEBUG_ABS_ORIGIN_CHANGES: usize = 0xE6; // bool
                pub const M_B_DORMANT: usize = 0xE7; // bool
                pub const M_B_FORCE_PARENT_TO_BE_NETWORKED: usize = 0xE8; // bool
                pub const M_B_DIRTY_HIERARCHY: usize = 0x0; // bitfield:1
                pub const M_B_DIRTY_BONE_MERGE_INFO: usize = 0x0; // bitfield:1
                pub const M_B_NETWORKED_POSITION_CHANGED: usize = 0x0; // bitfield:1
                pub const M_B_NETWORKED_ANGLES_CHANGED: usize = 0x0; // bitfield:1
                pub const M_B_NETWORKED_SCALE_CHANGED: usize = 0x0; // bitfield:1
                pub const M_B_WILL_BE_CALLING_POST_DATA_UPDATE: usize = 0x0; // bitfield:1
                pub const M_B_BONE_MERGE_FLEX: usize = 0x0; // bitfield:1
                pub const M_N_LATCH_ABS_ORIGIN: usize = 0x0; // bitfield:2
                pub const M_B_DIRTY_BONE_MERGE_BONE_TO_ROOT: usize = 0x0; // bitfield:1
                pub const M_N_HIERARCHICAL_DEPTH: usize = 0xEB; // uint8
                pub const M_N_HIERARCHY_TYPE: usize = 0xEC; // uint8
                pub const M_N_DO_NOT_SET_ANIM_TIME_IN_INVALIDATE_PHYSICS_COUNT: usize = 0xED; // uint8
                pub const M_NAME: usize = 0xF0; // CUtlStringToken
                pub const M_HIERARCHY_ATTACH_NAME: usize = 0x130; // CUtlStringToken
                pub const M_FL_Z_OFFSET: usize = 0x134; // float32
                pub const M_V_RENDER_ORIGIN: usize = 0x138; // 
            }
            // Parent: None
            // Fields count: 6
            pub mod sequence_history_t {
                pub const M_H_SEQUENCE: usize = 0x0; // HSequence
                pub const M_FL_SEQ_START_TIME: usize = 0x4; // GameTime_t
                pub const M_FL_SEQ_FIXED_CYCLE: usize = 0x8; // float32
                pub const M_N_SEQ_LOOP_MODE: usize = 0xC; // AnimLoopMode_t
                pub const M_FL_PLAYBACK_RATE: usize = 0x10; // float32
                pub const M_FL_CYCLES_PER_SECOND: usize = 0x14; // float32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_hSequence (HSequence)
            // NetworkVarNames: m_flPrevCycle (float32)
            // NetworkVarNames: m_flCycle (float32)
            pub mod c_networked_sequence_operation {
                pub const M_H_SEQUENCE: usize = 0x8; // HSequence
                pub const M_FL_PREV_CYCLE: usize = 0xC; // float32
                pub const M_FL_CYCLE: usize = 0x10; // float32
                pub const M_FL_WEIGHT: usize = 0x14; // CNetworkedQuantizedFloat
                pub const M_B_SEQUENCE_CHANGE_NETWORKED: usize = 0x1C; // bool
                pub const M_B_DISCONTINUITY: usize = 0x1D; // bool
                pub const M_FL_PREV_CYCLE_FROM_DISCONTINUITY: usize = 0x20; // float32
                pub const M_FL_PREV_CYCLE_FOR_ANIM_EVENT_DETECTION: usize = 0x24; // float32
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_hModel (HModelStrong)
            // NetworkVarNames: m_bClientClothCreationSuppressed (bool)
            // NetworkVarNames: m_MeshGroupMask (MeshGroupMask_t)
            // NetworkVarNames: m_nIdealMotionType (int8)
            pub mod c_model_state {
                pub const M_H_MODEL: usize = 0xA0; // CStrongHandle< InfoForResourceTypeCModel >
                pub const M_MODEL_NAME: usize = 0xA8; // CUtlSymbolLarge
                pub const M_B_CLIENT_CLOTH_CREATION_SUPPRESSED: usize = 0xE8; // bool
                pub const M_MESH_GROUP_MASK: usize = 0x180; // uint64
                pub const M_N_IDEAL_MOTION_TYPE: usize = 0x202; // int8
                pub const M_N_FORCE_LOD: usize = 0x203; // int8
                pub const M_N_CLOTH_UPDATE_FLAGS: usize = 0x204; // int8
            }
            // Parent: c_game_scene_node
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_modelState (CModelState)
            // NetworkVarNames: m_bIsAnimationEnabled (bool)
            // NetworkVarNames: m_bUseParentRenderBounds (bool)
            // NetworkVarNames: m_materialGroup (CUtlStringToken)
            // NetworkVarNames: m_nHitboxSet (uint8)
            pub mod c_skeleton_instance {
                pub const M_MODEL_STATE: usize = 0x160; // CModelState
                pub const M_B_IS_ANIMATION_ENABLED: usize = 0x370; // bool
                pub const M_B_USE_PARENT_RENDER_BOUNDS: usize = 0x371; // bool
                pub const M_B_DISABLE_SOLID_COLLISIONS_FOR_HIERARCHY: usize = 0x372; // bool
                pub const M_B_DIRTY_MOTION_TYPE: usize = 0x0; // bitfield:1
                pub const M_B_IS_GENERATING_LATCHED_PARENT_SPACE_STATE: usize = 0x0; // bitfield:1
                pub const M_MATERIAL_GROUP: usize = 0x374; // CUtlStringToken
                pub const M_N_HITBOX_SET: usize = 0x378; // uint8
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_timestamp (GameTime_t)
            // NetworkVarNames: m_nWorldGroupId (WorldGroupId_t)
            pub mod interval_timer {
                pub const M_TIMESTAMP: usize = 0x8; // GameTime_t
                pub const M_N_WORLD_GROUP_ID: usize = 0xC; // WorldGroupId_t
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_duration (float32)
            // NetworkVarNames: m_timestamp (GameTime_t)
            // NetworkVarNames: m_timescale (float32)
            // NetworkVarNames: m_nWorldGroupId (WorldGroupId_t)
            pub mod countdown_timer {
                pub const M_DURATION: usize = 0x8; // float32
                pub const M_TIMESTAMP: usize = 0xC; // GameTime_t
                pub const M_TIMESCALE: usize = 0x10; // float32
                pub const M_N_WORLD_GROUP_ID: usize = 0x14; // WorldGroupId_t
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_duration (float32)
            // NetworkVarNames: m_timestamp (float32)
            // NetworkVarNames: m_timescale (float32)
            pub mod engine_countdown_timer {
                pub const M_DURATION: usize = 0x8; // float32
                pub const M_TIMESTAMP: usize = 0xC; // float32
                pub const M_TIMESCALE: usize = 0x10; // float32
            }
            // Parent: interval_timer
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_flValues (float)
            // NetworkVarNames: m_nValueCounts (int)
            // NetworkVarNames: m_nBucketCount (int)
            // NetworkVarNames: m_flInterval (float)
            // NetworkVarNames: m_flFinalValue (float)
            // NetworkVarNames: m_nCompressionType (TimelineCompression_t)
            // NetworkVarNames: m_bStopped (bool)
            pub mod c_timeline {
                pub const M_FL_VALUES: usize = 0x10; // float32[64]
                pub const M_N_VALUE_COUNTS: usize = 0x110; // int32[64]
                pub const M_N_BUCKET_COUNT: usize = 0x210; // int32
                pub const M_FL_INTERVAL: usize = 0x214; // float32
                pub const M_FL_FINAL_VALUE: usize = 0x218; // float32
                pub const M_N_COMPRESSION_TYPE: usize = 0x21C; // TimelineCompression_t
                pub const M_B_STOPPED: usize = 0x220; // bool
            }
            // Parent: None
            // Fields count: 24
            //
            // Metadata:
            // NetworkVarNames: m_PredNetBoolVariables (uint32)
            // NetworkVarNames: m_PredNetByteVariables (byte)
            // NetworkVarNames: m_PredNetUInt16Variables (uint16)
            // NetworkVarNames: m_PredNetIntVariables (int32)
            // NetworkVarNames: m_PredNetUInt32Variables (uint32)
            // NetworkVarNames: m_PredNetUInt64Variables (uint64)
            // NetworkVarNames: m_PredNetFloatVariables (float)
            // NetworkVarNames: m_PredNetVectorVariables (Vector)
            // NetworkVarNames: m_PredNetQuaternionVariables (Quaternion)
            // NetworkVarNames: m_PredNetGlobalSymbolVariables (CGlobalSymbol)
            // NetworkVarNames: m_OwnerOnlyPredNetBoolVariables (uint32)
            // NetworkVarNames: m_OwnerOnlyPredNetByteVariables (byte)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt16Variables (uint16)
            // NetworkVarNames: m_OwnerOnlyPredNetIntVariables (int32)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt32Variables (uint32)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt64Variables (uint64)
            // NetworkVarNames: m_OwnerOnlyPredNetFloatVariables (float)
            // NetworkVarNames: m_OwnerOnlyPredNetVectorVariables (Vector)
            // NetworkVarNames: m_OwnerOnlyPredNetQuaternionVariables (Quaternion)
            // NetworkVarNames: m_OwnerOnlyPredNetGlobalSymbolVariables (CGlobalSymbol)
            // NetworkVarNames: m_nBoolVariablesCount (int)
            // NetworkVarNames: m_nOwnerOnlyBoolVariablesCount (int)
            // NetworkVarNames: m_nRandomSeedOffset (int)
            // NetworkVarNames: m_flLastTeleportTime (float)
            pub mod c_anim_graph_networked_variables {
                pub const M_PRED_NET_BOOL_VARIABLES: usize = 0x8; // C_NetworkUtlVectorBase< uint32 >
                pub const M_PRED_NET_BYTE_VARIABLES: usize = 0x20; // C_NetworkUtlVectorBase< uint8 >
                pub const M_PRED_NET_U_INT16_VARIABLES: usize = 0x38; // C_NetworkUtlVectorBase< uint16 >
                pub const M_PRED_NET_INT_VARIABLES: usize = 0x50; // C_NetworkUtlVectorBase< int32 >
                pub const M_PRED_NET_U_INT32_VARIABLES: usize = 0x68; // C_NetworkUtlVectorBase< uint32 >
                pub const M_PRED_NET_U_INT64_VARIABLES: usize = 0x80; // C_NetworkUtlVectorBase< uint64 >
                pub const M_PRED_NET_FLOAT_VARIABLES: usize = 0x98; // C_NetworkUtlVectorBase< float32 >
                pub const M_PRED_NET_VECTOR_VARIABLES: usize = 0xB0; // C_NetworkUtlVectorBase< Vector >
                pub const M_PRED_NET_QUATERNION_VARIABLES: usize = 0xC8; // C_NetworkUtlVectorBase< Quaternion >
                pub const M_PRED_NET_GLOBAL_SYMBOL_VARIABLES: usize = 0xE0; // C_NetworkUtlVectorBase< CGlobalSymbol >
                pub const M_OWNER_ONLY_PRED_NET_BOOL_VARIABLES: usize = 0xF8; // C_NetworkUtlVectorBase< uint32 >
                pub const M_OWNER_ONLY_PRED_NET_BYTE_VARIABLES: usize = 0x110; // C_NetworkUtlVectorBase< uint8 >
                pub const M_OWNER_ONLY_PRED_NET_U_INT16_VARIABLES: usize = 0x128; // C_NetworkUtlVectorBase< uint16 >
                pub const M_OWNER_ONLY_PRED_NET_INT_VARIABLES: usize = 0x140; // C_NetworkUtlVectorBase< int32 >
                pub const M_OWNER_ONLY_PRED_NET_U_INT32_VARIABLES: usize = 0x158; // C_NetworkUtlVectorBase< uint32 >
                pub const M_OWNER_ONLY_PRED_NET_U_INT64_VARIABLES: usize = 0x170; // C_NetworkUtlVectorBase< uint64 >
                pub const M_OWNER_ONLY_PRED_NET_FLOAT_VARIABLES: usize = 0x188; // C_NetworkUtlVectorBase< float32 >
                pub const M_OWNER_ONLY_PRED_NET_VECTOR_VARIABLES: usize = 0x1A0; // C_NetworkUtlVectorBase< Vector >
                pub const M_OWNER_ONLY_PRED_NET_QUATERNION_VARIABLES: usize = 0x1B8; // C_NetworkUtlVectorBase< Quaternion >
                pub const M_OWNER_ONLY_PRED_NET_GLOBAL_SYMBOL_VARIABLES: usize = 0x1D0; // C_NetworkUtlVectorBase< CGlobalSymbol >
                pub const M_N_BOOL_VARIABLES_COUNT: usize = 0x1E8; // int32
                pub const M_N_OWNER_ONLY_BOOL_VARIABLES_COUNT: usize = 0x1EC; // int32
                pub const M_N_RANDOM_SEED_OFFSET: usize = 0x1F0; // int32
                pub const M_FL_LAST_TELEPORT_TIME: usize = 0x1F4; // float32
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod c_base_entity_api {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod c_take_damage_info_api {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            pub mod c_client_point_entity_api {
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod c_pulse_graph_instance_client_entity {
                pub const M_P_PARENT: usize = 0xD8; // CClientScriptEntity*
            }
            // Parent: c_entity_instance
            // Fields count: 80
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // NetworkVarNames: m_CBodyComponent (CBodyComponent::Storage_t)
            // NetworkVarNames: m_iMaxHealth (int32)
            // NetworkVarNames: m_iHealth (int32)
            // NetworkVarNames: m_lifeState (uint8)
            // NetworkVarNames: m_bTakesDamage (bool)
            // NetworkVarNames: m_nTakeDamageFlags (TakeDamageFlags_t)
            // NetworkVarNames: m_bIsPlatform (bool)
            // NetworkVarNames: m_ubInterpolationFrame (uint8)
            // NetworkVarNames: m_nSubclassID (EntitySubclassID_t)
            // NetworkVarNames: m_flAnimTime (float32)
            // NetworkVarNames: m_flSimulationTime (float32)
            // NetworkVarNames: m_flCreateTime (GameTime_t)
            // NetworkVarNames: m_bClientSideRagdoll (bool)
            // NetworkVarNames: m_iTeamNum (uint8)
            // NetworkVarNames: m_spawnflags (uint32)
            // NetworkVarNames: m_nNextThinkTick (GameTick_t)
            // NetworkVarNames: m_fFlags (uint32)
            // NetworkVarNames: m_vecBaseVelocity (Vector)
            // NetworkVarNames: m_hEffectEntity (CHandle< CBaseEntity>)
            // NetworkVarNames: m_hOwnerEntity (CHandle< CBaseEntity>)
            // NetworkVarNames: m_MoveCollide (MoveCollide_t)
            // NetworkVarNames: m_MoveType (MoveType_t)
            // NetworkVarNames: m_flWaterLevel (float32)
            // NetworkVarNames: m_fEffects (uint32)
            // NetworkVarNames: m_hGroundEntity (CHandle< CBaseEntity>)
            // NetworkVarNames: m_flFriction (float32)
            // NetworkVarNames: m_flElasticity (float32)
            // NetworkVarNames: m_flGravityScale (float32)
            // NetworkVarNames: m_flTimeScale (float32)
            // NetworkVarNames: m_bAnimatedEveryTick (bool)
            // NetworkVarNames: m_flNavIgnoreUntilTime (GameTime_t)
            pub mod c_base_entity {
                pub const M_C_BODY_COMPONENT: usize = 0x38; // CBodyComponent*
                pub const M_NETWORK_TRANSMIT_COMPONENT: usize = 0x40; // CNetworkTransmitComponent
                pub const M_N_LAST_THINK_TICK: usize = 0x310; // GameTick_t
                pub const M_P_GAME_SCENE_NODE: usize = 0x318; // CGameSceneNode*
                pub const M_P_RENDER_COMPONENT: usize = 0x320; // CRenderComponent*
                pub const M_P_COLLISION: usize = 0x328; // CCollisionProperty*
                pub const M_I_MAX_HEALTH: usize = 0x330; // int32
                pub const M_I_HEALTH: usize = 0x334; // int32
                pub const M_LIFE_STATE: usize = 0x338; // uint8
                pub const M_B_TAKES_DAMAGE: usize = 0x339; // bool
                pub const M_N_TAKE_DAMAGE_FLAGS: usize = 0x33C; // TakeDamageFlags_t
                pub const M_B_IS_PLATFORM: usize = 0x340; // bool
                pub const M_UB_INTERPOLATION_FRAME: usize = 0x341; // uint8
                pub const M_H_SCENE_OBJECT_CONTROLLER: usize = 0x344; // CHandle< C_BaseEntity >
                pub const M_N_NO_INTERPOLATION_TICK: usize = 0x348; // int32
                pub const M_N_VISIBILITY_NO_INTERPOLATION_TICK: usize = 0x34C; // int32
                pub const M_FL_PROXY_RANDOM_VALUE: usize = 0x350; // float32
                pub const M_I_E_FLAGS: usize = 0x354; // int32
                pub const M_N_WATER_TYPE: usize = 0x358; // uint8
                pub const M_B_INTERPOLATE_EVEN_WITH_NO_MODEL: usize = 0x359; // bool
                pub const M_B_PREDICTION_ELIGIBLE: usize = 0x35A; // bool
                pub const M_B_APPLY_LAYER_MATCH_ID_TO_MODEL: usize = 0x35B; // bool
                pub const M_TOK_LAYER_MATCH_ID: usize = 0x35C; // CUtlStringToken
                pub const M_N_SUBCLASS_ID: usize = 0x360; // CUtlStringToken
                pub const M_N_SIMULATION_TICK: usize = 0x370; // int32
                pub const M_I_CURRENT_THINK_CONTEXT: usize = 0x374; // int32
                pub const M_A_THINK_FUNCTIONS: usize = 0x378; // CUtlVector< thinkfunc_t >
                pub const M_N_DISABLE_CONTEXT_THINK_START_TICK: usize = 0x390; // GameTick_t
                pub const M_FL_ANIM_TIME: usize = 0x394; // float32
                pub const M_FL_SIMULATION_TIME: usize = 0x398; // float32
                pub const M_N_SCENE_OBJECT_OVERRIDE_FLAGS: usize = 0x39C; // uint8
                pub const M_B_HAS_SUCCESSFULLY_INTERPOLATED: usize = 0x39D; // bool
                pub const M_B_HAS_ADDED_VARS_TO_INTERPOLATION: usize = 0x39E; // bool
                pub const M_B_RENDER_EVEN_WHEN_NOT_SUCCESSFULLY_INTERPOLATED: usize = 0x39F; // bool
                pub const M_N_INTERPOLATION_LATCH_DIRTY_FLAGS: usize = 0x3A0; // int32[2]
                pub const M_LIST_ENTRY: usize = 0x3A8; // uint16[11]
                pub const M_FL_CREATE_TIME: usize = 0x3C0; // GameTime_t
                pub const M_FL_SPEED: usize = 0x3C4; // float32
                pub const M_ENT_CLIENT_FLAGS: usize = 0x3C8; // uint16
                pub const M_B_CLIENT_SIDE_RAGDOLL: usize = 0x3CA; // bool
                pub const M_I_TEAM_NUM: usize = 0x3CB; // uint8
                pub const M_SPAWNFLAGS: usize = 0x3CC; // uint32
                pub const M_N_NEXT_THINK_TICK: usize = 0x3D0; // GameTick_t
                pub const M_F_FLAGS: usize = 0x3D4; // uint32
                pub const M_VEC_ABS_VELOCITY: usize = 0x3D8; // 
                pub const M_VEC_VELOCITY: usize = 0x3E8; // CNetworkVelocityVector
                pub const M_VEC_BASE_VELOCITY: usize = 0x418; // 
                pub const M_H_EFFECT_ENTITY: usize = 0x424; // CHandle< C_BaseEntity >
                pub const M_H_OWNER_ENTITY: usize = 0x428; // CHandle< C_BaseEntity >
                pub const M_MOVE_COLLIDE: usize = 0x42C; // MoveCollide_t
                pub const M_MOVE_TYPE: usize = 0x42D; // MoveType_t
                pub const M_N_ACTUAL_MOVE_TYPE: usize = 0x42E; // MoveType_t
                pub const M_FL_WATER_LEVEL: usize = 0x430; // float32
                pub const M_F_EFFECTS: usize = 0x434; // uint32
                pub const M_H_GROUND_ENTITY: usize = 0x438; // CHandle< C_BaseEntity >
                pub const M_FL_FRICTION: usize = 0x43C; // float32
                pub const M_FL_ELASTICITY: usize = 0x440; // float32
                pub const M_FL_GRAVITY_SCALE: usize = 0x444; // float32
                pub const M_FL_TIME_SCALE: usize = 0x448; // float32
                pub const M_B_ANIMATED_EVERY_TICK: usize = 0x44C; // bool
                pub const M_FL_NAV_IGNORE_UNTIL_TIME: usize = 0x450; // GameTime_t
                pub const M_H_THINK: usize = 0x454; // uint16
                pub const M_F_B_BOX_VIS_FLAGS: usize = 0x460; // uint8
                pub const M_B_PREDICTABLE: usize = 0x461; // bool
                pub const M_B_RENDER_WITH_VIEW_MODELS: usize = 0x462; // bool
                pub const M_N_SPLIT_USER_PLAYER_PREDICTION_SLOT: usize = 0x464; // CSplitScreenSlot
                pub const M_N_FIRST_PREDICTABLE_COMMAND: usize = 0x468; // int32
                pub const M_N_LAST_PREDICTABLE_COMMAND: usize = 0x46C; // int32
                pub const M_H_OLD_MOVE_PARENT: usize = 0x470; // CHandle< C_BaseEntity >
                pub const M_PARTICLES: usize = 0x478; // CParticleProperty
                pub const M_VEC_PREDICTED_SCRIPT_FLOATS: usize = 0x4A0; // CUtlVector< float32 >
                pub const M_VEC_PREDICTED_SCRIPT_FLOAT_I_DS: usize = 0x4B8; // CUtlVector< int32 >
                pub const M_N_NEXT_SCRIPT_VAR_RECORD_ID: usize = 0x4E8; // int32
                pub const M_VEC_ANG_VELOCITY: usize = 0x4F8; // QAngle
                pub const M_DATA_CHANGE_EVENT_REF: usize = 0x504; // int32
                pub const M_DEPENDENCIES: usize = 0x508; // CUtlVector< CEntityHandle >
                pub const M_N_CREATION_TICK: usize = 0x520; // int32
                pub const M_B_ANIM_TIME_CHANGED: usize = 0x539; // bool
                pub const M_B_SIMULATION_TIME_CHANGED: usize = 0x53A; // bool
                pub const M_S_UNIQUE_HAMMER_ID: usize = 0x548; // CUtlString
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_logical_entity {
            }
            // Parent: None
            // Fields count: 5
            pub mod c_base_flex_emphasized_phoneme {
                pub const M_S_CLASS_NAME: usize = 0x0; // CUtlString
                pub const M_FL_AMOUNT: usize = 0x18; // float32
                pub const M_B_REQUIRED: usize = 0x1C; // bool
                pub const M_B_BASECHECKED: usize = 0x1D; // bool
                pub const M_B_VALID: usize = 0x1E; // bool
            }
            // Parent: c_base_entity
            // Fields count: 18
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // NetworkVarNames: m_MinFalloff (float32)
            // NetworkVarNames: m_MaxFalloff (float32)
            // NetworkVarNames: m_flFadeInDuration (float32)
            // NetworkVarNames: m_flFadeOutDuration (float32)
            // NetworkVarNames: m_flMaxWeight (float32)
            // NetworkVarNames: m_flCurWeight (float32)
            // NetworkVarNames: m_netlookupFilename (char)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bMaster (bool)
            // NetworkVarNames: m_bClientSide (bool)
            // NetworkVarNames: m_bExclusive (bool)
            pub mod c_color_correction {
                pub const M_VEC_ORIGIN: usize = 0x550; // 
                pub const M_MIN_FALLOFF: usize = 0x55C; // float32
                pub const M_MAX_FALLOFF: usize = 0x560; // float32
                pub const M_FL_FADE_IN_DURATION: usize = 0x564; // float32
                pub const M_FL_FADE_OUT_DURATION: usize = 0x568; // float32
                pub const M_FL_MAX_WEIGHT: usize = 0x56C; // float32
                pub const M_FL_CUR_WEIGHT: usize = 0x570; // float32
                pub const M_NETLOOKUP_FILENAME: usize = 0x574; // char[512]
                pub const M_B_ENABLED: usize = 0x774; // bool
                pub const M_B_MASTER: usize = 0x775; // bool
                pub const M_B_CLIENT_SIDE: usize = 0x776; // bool
                pub const M_B_EXCLUSIVE: usize = 0x777; // bool
                pub const M_B_ENABLED_ON_CLIENT: usize = 0x778; // bool[1]
                pub const M_FL_CUR_WEIGHT_ON_CLIENT: usize = 0x77C; // float32[1]
                pub const M_B_FADING_IN: usize = 0x780; // bool[1]
                pub const M_FL_FADE_START_WEIGHT: usize = 0x784; // float32[1]
                pub const M_FL_FADE_START_TIME: usize = 0x788; // float32[1]
                pub const M_FL_FADE_DURATION: usize = 0x78C; // float32[1]
            }
            // Parent: c_base_entity
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_EnvWindShared (CEnvWindShared)
            pub mod c_env_wind_clientside {
                pub const M_ENV_WIND_SHARED: usize = 0x550; // C_EnvWindShared
            }
            // Parent: c_base_entity
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_hEntAttached (CHandle< C_BaseEntity>)
            // NetworkVarNames: m_bCheapEffect (bool)
            pub mod c_entity_flame {
                pub const M_H_ENT_ATTACHED: usize = 0x550; // CHandle< C_BaseEntity >
                pub const M_H_OLD_ATTACHED: usize = 0x578; // CHandle< C_BaseEntity >
                pub const M_B_CHEAP_EFFECT: usize = 0x57C; // bool
            }
            // Parent: None
            // Fields count: 30
            //
            // Metadata:
            // NetworkVarNames: m_hTargetEntity (CHandle< C_BaseEntity>)
            // NetworkVarNames: m_bState (bool)
            // NetworkVarNames: m_bAlwaysUpdate (bool)
            // NetworkVarNames: m_flLightFOV (float32)
            // NetworkVarNames: m_bEnableShadows (bool)
            // NetworkVarNames: m_bSimpleProjection (bool)
            // NetworkVarNames: m_bLightOnlyTarget (bool)
            // NetworkVarNames: m_bLightWorld (bool)
            // NetworkVarNames: m_bCameraSpace (bool)
            // NetworkVarNames: m_flBrightnessScale (float32)
            // NetworkVarNames: m_LightColor (Color)
            // NetworkVarNames: m_flIntensity (float32)
            // NetworkVarNames: m_flLinearAttenuation (float32)
            // NetworkVarNames: m_flQuadraticAttenuation (float32)
            // NetworkVarNames: m_bVolumetric (bool)
            // NetworkVarNames: m_flVolumetricIntensity (float32)
            // NetworkVarNames: m_flNoiseStrength (float32)
            // NetworkVarNames: m_flFlashlightTime (float32)
            // NetworkVarNames: m_nNumPlanes (uint32)
            // NetworkVarNames: m_flPlaneOffset (float32)
            // NetworkVarNames: m_flColorTransitionTime (float32)
            // NetworkVarNames: m_flAmbient (float32)
            // NetworkVarNames: m_SpotlightTextureName (char)
            // NetworkVarNames: m_nSpotlightTextureFrame (int32)
            // NetworkVarNames: m_nShadowQuality (uint32)
            // NetworkVarNames: m_flNearZ (float32)
            // NetworkVarNames: m_flFarZ (float32)
            // NetworkVarNames: m_flProjectionSize (float32)
            // NetworkVarNames: m_flRotation (float32)
            // NetworkVarNames: m_bFlipHorizontal (bool)
            pub mod c_projected_texture_base {
                pub const M_H_TARGET_ENTITY: usize = 0xC; // CHandle< C_BaseEntity >
                pub const M_B_STATE: usize = 0x10; // bool
                pub const M_B_ALWAYS_UPDATE: usize = 0x11; // bool
                pub const M_FL_LIGHT_FOV: usize = 0x14; // float32
                pub const M_B_ENABLE_SHADOWS: usize = 0x18; // bool
                pub const M_B_SIMPLE_PROJECTION: usize = 0x19; // bool
                pub const M_B_LIGHT_ONLY_TARGET: usize = 0x1A; // bool
                pub const M_B_LIGHT_WORLD: usize = 0x1B; // bool
                pub const M_B_CAMERA_SPACE: usize = 0x1C; // bool
                pub const M_FL_BRIGHTNESS_SCALE: usize = 0x20; // float32
                pub const M_LIGHT_COLOR: usize = 0x24; // Color
                pub const M_FL_INTENSITY: usize = 0x28; // float32
                pub const M_FL_LINEAR_ATTENUATION: usize = 0x2C; // float32
                pub const M_FL_QUADRATIC_ATTENUATION: usize = 0x30; // float32
                pub const M_B_VOLUMETRIC: usize = 0x34; // bool
                pub const M_FL_VOLUMETRIC_INTENSITY: usize = 0x38; // float32
                pub const M_FL_NOISE_STRENGTH: usize = 0x3C; // float32
                pub const M_FL_FLASHLIGHT_TIME: usize = 0x40; // float32
                pub const M_N_NUM_PLANES: usize = 0x44; // uint32
                pub const M_FL_PLANE_OFFSET: usize = 0x48; // float32
                pub const M_FL_COLOR_TRANSITION_TIME: usize = 0x4C; // float32
                pub const M_FL_AMBIENT: usize = 0x50; // float32
                pub const M_SPOTLIGHT_TEXTURE_NAME: usize = 0x54; // char[512]
                pub const M_N_SPOTLIGHT_TEXTURE_FRAME: usize = 0x254; // int32
                pub const M_N_SHADOW_QUALITY: usize = 0x258; // uint32
                pub const M_FL_NEAR_Z: usize = 0x25C; // float32
                pub const M_FL_FAR_Z: usize = 0x260; // float32
                pub const M_FL_PROJECTION_SIZE: usize = 0x264; // float32
                pub const M_FL_ROTATION: usize = 0x268; // float32
                pub const M_B_FLIP_HORIZONTAL: usize = 0x26C; // bool
            }
            // Parent: c_base_entity
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_flScale (float32)
            // NetworkVarNames: m_flStartScale (float32)
            // NetworkVarNames: m_flScaleTime (float)
            // NetworkVarNames: m_nFlags (uint32)
            pub mod c_base_fire {
                pub const M_FL_SCALE: usize = 0x550; // float32
                pub const M_FL_START_SCALE: usize = 0x554; // float32
                pub const M_FL_SCALE_TIME: usize = 0x558; // float32
                pub const M_N_FLAGS: usize = 0x55C; // uint32
            }
            // Parent: c_base_fire
            // Fields count: 13
            //
            // Metadata:
            // MNetworkOverride
            // MNetworkOverride
            // NetworkVarNames: m_nFlameModelIndex (int32)
            // NetworkVarNames: m_nFlameFromAboveModelIndex (int32)
            pub mod c_fire_smoke {
                pub const M_N_FLAME_MODEL_INDEX: usize = 0x560; // int32
                pub const M_N_FLAME_FROM_ABOVE_MODEL_INDEX: usize = 0x564; // int32
                pub const M_FL_SCALE_REGISTER: usize = 0x568; // float32
                pub const M_FL_SCALE_START: usize = 0x56C; // float32
                pub const M_FL_SCALE_END: usize = 0x570; // float32
                pub const M_FL_SCALE_TIME_START: usize = 0x574; // GameTime_t
                pub const M_FL_SCALE_TIME_END: usize = 0x578; // GameTime_t
                pub const M_FL_CHILD_FLAME_SPREAD: usize = 0x57C; // float32
                pub const M_FL_CLIP_PERC: usize = 0x590; // float32
                pub const M_B_CLIP_TESTED: usize = 0x594; // bool
                pub const M_B_FADING_OUT: usize = 0x595; // bool
                pub const M_T_PARTICLE_SPAWN: usize = 0x598; // TimedEvent
                pub const M_P_FIRE_OVERLAY: usize = 0x5A0; // CFireOverlay*
            }
            // Parent: None
            // Fields count: 1
            pub mod c_rope_keyframe_c_physics_delegate {
                pub const M_P_KEYFRAME: usize = 0x8; // C_RopeKeyframe*
            }
            // Parent: None
            // Fields count: 1
            pub mod c_scene_entity_queued_events_t {
                pub const STARTTIME: usize = 0x0; // float32
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_tint_controller {
            }
            // Parent: None
            // Fields count: 13
            pub mod c_flashlight_effect {
                pub const M_B_IS_ON: usize = 0x10; // bool
                pub const M_B_MUZZLE_FLASH_ENABLED: usize = 0x20; // bool
                pub const M_FL_MUZZLE_FLASH_BRIGHTNESS: usize = 0x24; // float32
                pub const M_QUAT_MUZZLE_FLASH_ORIENTATION: usize = 0x30; // Quaternion
                pub const M_VEC_MUZZLE_FLASH_ORIGIN: usize = 0x40; // 
                pub const M_FL_FOV: usize = 0x4C; // float32
                pub const M_FL_FAR_Z: usize = 0x50; // float32
                pub const M_FL_LINEAR_ATTEN: usize = 0x54; // float32
                pub const M_B_CASTS_SHADOWS: usize = 0x58; // bool
                pub const M_FL_CURRENT_PULL_BACK_DIST: usize = 0x5C; // float32
                pub const M_FLASHLIGHT_TEXTURE: usize = 0x60; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_MUZZLE_FLASH_TEXTURE: usize = 0x68; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_TEXTURE_NAME: usize = 0x70; // char[64]
            }
            // Parent: None
            // Fields count: 5
            pub mod c_interpolated_value {
                pub const M_FL_START_TIME: usize = 0x0; // float32
                pub const M_FL_END_TIME: usize = 0x4; // float32
                pub const M_FL_START_VALUE: usize = 0x8; // float32
                pub const M_FL_END_VALUE: usize = 0xC; // float32
                pub const M_N_INTERP_TYPE: usize = 0x10; // int32
            }
            // Parent: None
            // Fields count: 4
            pub mod c_glow_sprite {
                pub const M_V_COLOR: usize = 0x0; // 
                pub const M_FL_HORZ_SIZE: usize = 0xC; // float32
                pub const M_FL_VERT_SIZE: usize = 0x10; // float32
                pub const M_H_MATERIAL: usize = 0x18; // CStrongHandle< InfoForResourceTypeIMaterial2 >
            }
            // Parent: None
            // Fields count: 15
            pub mod c_glow_overlay {
                pub const M_V_POS: usize = 0x8; // 
                pub const M_B_DIRECTIONAL: usize = 0x14; // bool
                pub const M_V_DIRECTION: usize = 0x18; // 
                pub const M_B_IN_SKY: usize = 0x24; // bool
                pub const M_SKY_OBSTRUCTION_SCALE: usize = 0x28; // float32
                pub const M_SPRITES: usize = 0x30; // CGlowSprite[4]
                pub const M_N_SPRITES: usize = 0xB0; // int32
                pub const M_FL_PROXY_RADIUS: usize = 0xB4; // float32
                pub const M_FL_HDR_COLOR_SCALE: usize = 0xB8; // float32
                pub const M_FL_GLOW_OBSTRUCTION_SCALE: usize = 0xBC; // float32
                pub const M_B_CACHE_GLOW_OBSTRUCTION: usize = 0xC0; // bool
                pub const M_B_CACHE_SKY_OBSTRUCTION: usize = 0xC1; // bool
                pub const M_B_ACTIVATED: usize = 0xC2; // int16
                pub const M_LIST_INDEX: usize = 0xC4; // uint16
                pub const M_QUERY_HANDLE: usize = 0xC8; // int32
            }
            // Parent: None
            // Fields count: 0
            pub mod i_client_alpha_property {
            }
            // Parent: c_base_entity
            // Fields count: 2
            pub mod c_skybox_reference {
                pub const M_WORLD_GROUP_ID: usize = 0x550; // WorldGroupId_t
                pub const M_H_SKY_CAMERA: usize = 0x554; // CHandle< C_SkyCamera >
            }
            // Parent: c_base_entity
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_skyboxData (sky3dparams_t)
            // NetworkVarNames: m_skyboxSlotToken (CUtlStringToken)
            pub mod c_sky_camera {
                pub const M_SKYBOX_DATA: usize = 0x550; // sky3dparams_t
                pub const M_SKYBOX_SLOT_TOKEN: usize = 0x5E0; // CUtlStringToken
                pub const M_B_USE_ANGLES: usize = 0x5E4; // bool
                pub const M_P_NEXT: usize = 0x5E8; // C_SkyCamera*
            }
            // Parent: None
            // Fields count: 2
            pub mod timed_event {
                pub const M_TIME_BETWEEN_EVENTS: usize = 0x0; // float32
                pub const M_F_NEXT_EVENT: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_nInteractsAs (uint64)
            // NetworkVarNames: m_nInteractsWith (uint64)
            // NetworkVarNames: m_nInteractsExclude (uint64)
            // NetworkVarNames: m_nEntityId (uint32)
            // NetworkVarNames: m_nOwnerId (uint32)
            // NetworkVarNames: m_nHierarchyId (uint16)
            // NetworkVarNames: m_nCollisionGroup (uint8)
            // NetworkVarNames: m_nCollisionFunctionMask (uint8)
            pub mod v_physics_collision_attribute_t {
                pub const M_N_INTERACTS_AS: usize = 0x8; // uint64
                pub const M_N_INTERACTS_WITH: usize = 0x10; // uint64
                pub const M_N_INTERACTS_EXCLUDE: usize = 0x18; // uint64
                pub const M_N_ENTITY_ID: usize = 0x20; // uint32
                pub const M_N_OWNER_ID: usize = 0x24; // uint32
                pub const M_N_HIERARCHY_ID: usize = 0x28; // uint16
                pub const M_N_COLLISION_GROUP: usize = 0x2A; // uint8
                pub const M_N_COLLISION_FUNCTION_MASK: usize = 0x2B; // uint8
            }
            // Parent: None
            // Fields count: 17
            //
            // Metadata:
            // NetworkVarNames: m_collisionAttribute (VPhysicsCollisionAttribute_t)
            // NetworkVarNames: m_vecMins (Vector)
            // NetworkVarNames: m_vecMaxs (Vector)
            // NetworkVarNames: m_usSolidFlags (uint8)
            // NetworkVarNames: m_nSolidType (SolidType_t)
            // NetworkVarNames: m_triggerBloat (uint8)
            // NetworkVarNames: m_nSurroundType (SurroundingBoundsType_t)
            // NetworkVarNames: m_CollisionGroup (uint8)
            // NetworkVarNames: m_nEnablePhysics (uint8)
            // NetworkVarNames: m_vecSpecifiedSurroundingMins (Vector)
            // NetworkVarNames: m_vecSpecifiedSurroundingMaxs (Vector)
            // NetworkVarNames: m_vCapsuleCenter1 (Vector)
            // NetworkVarNames: m_vCapsuleCenter2 (Vector)
            // NetworkVarNames: m_flCapsuleRadius (float)
            pub mod c_collision_property {
                pub const M_COLLISION_ATTRIBUTE: usize = 0x10; // VPhysicsCollisionAttribute_t
                pub const M_VEC_MINS: usize = 0x40; // 
                pub const M_VEC_MAXS: usize = 0x4C; // 
                pub const M_US_SOLID_FLAGS: usize = 0x5A; // uint8
                pub const M_N_SOLID_TYPE: usize = 0x5B; // SolidType_t
                pub const M_TRIGGER_BLOAT: usize = 0x5C; // uint8
                pub const M_N_SURROUND_TYPE: usize = 0x5D; // SurroundingBoundsType_t
                pub const M_COLLISION_GROUP: usize = 0x5E; // uint8
                pub const M_N_ENABLE_PHYSICS: usize = 0x5F; // uint8
                pub const M_FL_BOUNDING_RADIUS: usize = 0x60; // float32
                pub const M_VEC_SPECIFIED_SURROUNDING_MINS: usize = 0x64; // 
                pub const M_VEC_SPECIFIED_SURROUNDING_MAXS: usize = 0x70; // 
                pub const M_VEC_SURROUNDING_MAXS: usize = 0x7C; // 
                pub const M_VEC_SURROUNDING_MINS: usize = 0x88; // 
                pub const M_V_CAPSULE_CENTER1: usize = 0x94; // 
                pub const M_V_CAPSULE_CENTER2: usize = 0xA0; // 
                pub const M_FL_CAPSULE_RADIUS: usize = 0xAC; // float32
            }
            // Parent: None
            // Fields count: 12
            pub mod c_decal_info {
                pub const M_FL_ANIMATION_SCALE: usize = 0x0; // float32
                pub const M_FL_ANIMATION_LIFE_SPAN: usize = 0x4; // float32
                pub const M_FL_PLACE_TIME: usize = 0x8; // float32
                pub const M_FL_FADE_START_TIME: usize = 0xC; // float32
                pub const M_FL_FADE_DURATION: usize = 0x10; // float32
                pub const M_N_VB_SLOT: usize = 0x14; // int32
                pub const M_N_BONE_INDEX: usize = 0x18; // int32
                pub const M_V_POSITION: usize = 0x28; // 
                pub const M_FL_BOUNDING_RADIUS_SQR: usize = 0x34; // float32
                pub const M_P_NEXT: usize = 0x40; // CDecalInfo*
                pub const M_P_PREV: usize = 0x48; // CDecalInfo*
                pub const M_N_DECAL_MATERIAL_INDEX: usize = 0xA8; // int32
            }
            // Parent: None
            // Fields count: 21
            //
            // Metadata:
            // NetworkVarNames: m_vOrigin (Vector)
            // NetworkVarNames: m_vStart (Vector)
            // NetworkVarNames: m_vNormal (Vector)
            // NetworkVarNames: m_vAngles (QAngle)
            // NetworkVarNames: m_hEntity (CEntityHandle)
            // NetworkVarNames: m_hOtherEntity (CEntityHandle)
            // NetworkVarNames: m_flScale (float32)
            // NetworkVarNames: m_flMagnitude (float32)
            // NetworkVarNames: m_flRadius (float32)
            // NetworkVarNames: m_nSurfaceProp (CUtlStringToken)
            // NetworkVarNames: m_nEffectIndex (HParticleSystemDefinition)
            // NetworkVarNames: m_nDamageType (uint32)
            // NetworkVarNames: m_nPenetrate (uint8)
            // NetworkVarNames: m_nMaterial (uint16)
            // NetworkVarNames: m_nHitBox (uint16)
            // NetworkVarNames: m_nColor (uint8)
            // NetworkVarNames: m_fFlags (uint8)
            // NetworkVarNames: m_nAttachmentIndex (AttachmentHandle_t)
            // NetworkVarNames: m_nAttachmentName (CUtlStringToken)
            // NetworkVarNames: m_iEffectName (uint16)
            // NetworkVarNames: m_nExplosionType (uint8)
            pub mod c_effect_data {
                pub const M_V_ORIGIN: usize = 0x8; // 
                pub const M_V_START: usize = 0x14; // 
                pub const M_V_NORMAL: usize = 0x20; // 
                pub const M_V_ANGLES: usize = 0x2C; // QAngle
                pub const M_H_ENTITY: usize = 0x38; // CEntityHandle
                pub const M_H_OTHER_ENTITY: usize = 0x3C; // CEntityHandle
                pub const M_FL_SCALE: usize = 0x40; // float32
                pub const M_FL_MAGNITUDE: usize = 0x44; // float32
                pub const M_FL_RADIUS: usize = 0x48; // float32
                pub const M_N_SURFACE_PROP: usize = 0x4C; // CUtlStringToken
                pub const M_N_EFFECT_INDEX: usize = 0x50; // CWeakHandle< InfoForResourceTypeIParticleSystemDefinition >
                pub const M_N_DAMAGE_TYPE: usize = 0x58; // uint32
                pub const M_N_PENETRATE: usize = 0x5C; // uint8
                pub const M_N_MATERIAL: usize = 0x5E; // uint16
                pub const M_N_HIT_BOX: usize = 0x60; // uint16
                pub const M_N_COLOR: usize = 0x62; // uint8
                pub const M_F_FLAGS: usize = 0x63; // uint8
                pub const M_N_ATTACHMENT_INDEX: usize = 0x64; // AttachmentHandle_t
                pub const M_N_ATTACHMENT_NAME: usize = 0x68; // CUtlStringToken
                pub const M_I_EFFECT_NAME: usize = 0x6C; // uint16
                pub const M_N_EXPLOSION_TYPE: usize = 0x6E; // uint8
            }
            // Parent: c_base_entity
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_flFadeStartDist (float32)
            // NetworkVarNames: m_flFadeEndDist (float32)
            pub mod c_env_detail_controller {
                pub const M_FL_FADE_START_DIST: usize = 0x550; // float32
                pub const M_FL_FADE_END_DIST: usize = 0x554; // float32
            }
            // Parent: None
            // Fields count: 29
            //
            // Metadata:
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_iWindSeed (uint32)
            // NetworkVarNames: m_iMinWind (uint16)
            // NetworkVarNames: m_iMaxWind (uint16)
            // NetworkVarNames: m_windRadius (int32)
            // NetworkVarNames: m_iMinGust (uint16)
            // NetworkVarNames: m_iMaxGust (uint16)
            // NetworkVarNames: m_flMinGustDelay (float32)
            // NetworkVarNames: m_flMaxGustDelay (float32)
            // NetworkVarNames: m_flGustDuration (float32)
            // NetworkVarNames: m_iGustDirChange (uint16)
            // NetworkVarNames: m_location (Vector)
            // NetworkVarNames: m_iInitialWindDir (uint16)
            // NetworkVarNames: m_flInitialWindSpeed (float32)
            pub mod c_env_wind_shared {
                pub const M_FL_START_TIME: usize = 0x8; // GameTime_t
                pub const M_I_WIND_SEED: usize = 0xC; // uint32
                pub const M_I_MIN_WIND: usize = 0x10; // uint16
                pub const M_I_MAX_WIND: usize = 0x12; // uint16
                pub const M_WIND_RADIUS: usize = 0x14; // int32
                pub const M_I_MIN_GUST: usize = 0x18; // uint16
                pub const M_I_MAX_GUST: usize = 0x1A; // uint16
                pub const M_FL_MIN_GUST_DELAY: usize = 0x1C; // float32
                pub const M_FL_MAX_GUST_DELAY: usize = 0x20; // float32
                pub const M_FL_GUST_DURATION: usize = 0x24; // float32
                pub const M_I_GUST_DIR_CHANGE: usize = 0x28; // uint16
                pub const M_LOCATION: usize = 0x2C; // 
                pub const M_ISZ_GUST_SOUND: usize = 0x38; // int32
                pub const M_I_WIND_DIR: usize = 0x3C; // int32
                pub const M_FL_WIND_SPEED: usize = 0x40; // float32
                pub const M_CURRENT_WIND_VECTOR: usize = 0x44; // 
                pub const M_CURRENT_SWAY_VECTOR: usize = 0x50; // 
                pub const M_PREV_SWAY_VECTOR: usize = 0x5C; // 
                pub const M_I_INITIAL_WIND_DIR: usize = 0x68; // uint16
                pub const M_FL_INITIAL_WIND_SPEED: usize = 0x6C; // float32
                pub const M_FL_VARIATION_TIME: usize = 0x70; // GameTime_t
                pub const M_FL_SWAY_TIME: usize = 0x74; // GameTime_t
                pub const M_FL_SIM_TIME: usize = 0x78; // GameTime_t
                pub const M_FL_SWITCH_TIME: usize = 0x7C; // GameTime_t
                pub const M_FL_AVE_WIND_SPEED: usize = 0x80; // float32
                pub const M_B_GUSTING: usize = 0x84; // bool
                pub const M_FL_WIND_ANGLE_VARIATION: usize = 0x88; // float32
                pub const M_FL_WIND_SPEED_VARIATION: usize = 0x8C; // float32
                pub const M_I_ENT_INDEX: usize = 0x90; // CEntityIndex
            }
            // Parent: None
            // Fields count: 2
            pub mod c_env_wind_shared_wind_ave_event_t {
                pub const M_FL_START_WIND_SPEED: usize = 0x0; // float32
                pub const M_FL_AVE_WIND_SPEED: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 2
            pub mod c_env_wind_shared_wind_variation_event_t {
                pub const M_FL_WIND_ANGLE_VARIATION: usize = 0x0; // float32
                pub const M_FL_WIND_SPEED_VARIATION: usize = 0x4; // float32
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_info_ladder_dismount {
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // NetworkVarNames: m_nModelID (int32)
            // NetworkVarNames: m_hMaterial (HMaterialStrong)
            // NetworkVarNames: m_solid (ShardSolid_t)
            // NetworkVarNames: m_ShatterPanelMode (ShatterPanelMode)
            // NetworkVarNames: m_vecPanelSize (Vector2D)
            // NetworkVarNames: m_vecStressPositionA (Vector2D)
            // NetworkVarNames: m_vecStressPositionB (Vector2D)
            // NetworkVarNames: m_vecPanelVertices (Vector2D)
            // NetworkVarNames: m_flGlassHalfThickness (float)
            // NetworkVarNames: m_bHasParent (bool)
            // NetworkVarNames: m_bParentFrozen (bool)
            // NetworkVarNames: m_SurfacePropStringToken (CUtlStringToken)
            pub mod shard_model_desc_t {
                pub const M_N_MODEL_ID: usize = 0x8; // int32
                pub const M_H_MATERIAL: usize = 0x10; // CStrongHandle< InfoForResourceTypeIMaterial2 >
                pub const M_SOLID: usize = 0x18; // ShardSolid_t
                pub const M_SHATTER_PANEL_MODE: usize = 0x19; // ShatterPanelMode
                pub const M_VEC_PANEL_SIZE: usize = 0x1C; // Vector2D
                pub const M_VEC_STRESS_POSITION_A: usize = 0x24; // Vector2D
                pub const M_VEC_STRESS_POSITION_B: usize = 0x2C; // Vector2D
                pub const M_VEC_PANEL_VERTICES: usize = 0x38; // C_NetworkUtlVectorBase< Vector2D >
                pub const M_FL_GLASS_HALF_THICKNESS: usize = 0x50; // float32
                pub const M_B_HAS_PARENT: usize = 0x54; // bool
                pub const M_B_PARENT_FROZEN: usize = 0x55; // bool
                pub const M_SURFACE_PROP_STRING_TOKEN: usize = 0x58; // CUtlStringToken
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_game_rules_proxy {
            }
            // Parent: None
            // Fields count: 0
            pub mod c_game_rules {
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_iGlowType (int32)
            // NetworkVarNames: m_iGlowTeam (int32)
            // NetworkVarNames: m_nGlowRange (int32)
            // NetworkVarNames: m_nGlowRangeMin (int32)
            // NetworkVarNames: m_glowColorOverride (Color)
            // NetworkVarNames: m_bFlashing (bool)
            // NetworkVarNames: m_flGlowTime (float)
            // NetworkVarNames: m_flGlowStartTime (float)
            // NetworkVarNames: m_bEligibleForScreenHighlight (bool)
            pub mod c_glow_property {
                pub const M_F_GLOW_COLOR: usize = 0x8; // 
                pub const M_I_GLOW_TYPE: usize = 0x30; // int32
                pub const M_I_GLOW_TEAM: usize = 0x34; // int32
                pub const M_N_GLOW_RANGE: usize = 0x38; // int32
                pub const M_N_GLOW_RANGE_MIN: usize = 0x3C; // int32
                pub const M_GLOW_COLOR_OVERRIDE: usize = 0x40; // Color
                pub const M_B_FLASHING: usize = 0x44; // bool
                pub const M_FL_GLOW_TIME: usize = 0x48; // float32
                pub const M_FL_GLOW_START_TIME: usize = 0x4C; // float32
                pub const M_B_ELIGIBLE_FOR_SCREEN_HIGHLIGHT: usize = 0x50; // bool
                pub const M_B_GLOWING: usize = 0x51; // bool
            }
            // Parent: c_game_rules
            // Fields count: 0
            pub mod c_multiplay_rules {
            }
            // Parent: None
            // Fields count: 25
            //
            // Metadata:
            // NetworkVarNames: dirPrimary (Vector)
            // NetworkVarNames: colorPrimary (Color)
            // NetworkVarNames: colorSecondary (Color)
            // NetworkVarNames: colorPrimaryLerpTo (Color)
            // NetworkVarNames: colorSecondaryLerpTo (Color)
            // NetworkVarNames: start (float32)
            // NetworkVarNames: end (float32)
            // NetworkVarNames: farz (float32)
            // NetworkVarNames: maxdensity (float32)
            // NetworkVarNames: exponent (float32)
            // NetworkVarNames: HDRColorScale (float32)
            // NetworkVarNames: skyboxFogFactor (float32)
            // NetworkVarNames: skyboxFogFactorLerpTo (float32)
            // NetworkVarNames: startLerpTo (float32)
            // NetworkVarNames: endLerpTo (float32)
            // NetworkVarNames: maxdensityLerpTo (float32)
            // NetworkVarNames: lerptime (GameTime_t)
            // NetworkVarNames: duration (float32)
            // NetworkVarNames: blendtobackground (float32)
            // NetworkVarNames: scattering (float32)
            // NetworkVarNames: locallightscale (float32)
            // NetworkVarNames: enable (bool)
            // NetworkVarNames: blend (bool)
            // NetworkVarNames: m_bNoReflectionFog (bool)
            pub mod fogparams_t {
                pub const DIR_PRIMARY: usize = 0x8; // 
                pub const COLOR_PRIMARY: usize = 0x14; // Color
                pub const COLOR_SECONDARY: usize = 0x18; // Color
                pub const COLOR_PRIMARY_LERP_TO: usize = 0x1C; // Color
                pub const COLOR_SECONDARY_LERP_TO: usize = 0x20; // Color
                pub const START: usize = 0x24; // float32
                pub const END: usize = 0x28; // float32
                pub const FARZ: usize = 0x2C; // float32
                pub const MAXDENSITY: usize = 0x30; // float32
                pub const EXPONENT: usize = 0x34; // float32
                pub const HDR_COLOR_SCALE: usize = 0x38; // float32
                pub const SKYBOX_FOG_FACTOR: usize = 0x3C; // float32
                pub const SKYBOX_FOG_FACTOR_LERP_TO: usize = 0x40; // float32
                pub const START_LERP_TO: usize = 0x44; // float32
                pub const END_LERP_TO: usize = 0x48; // float32
                pub const MAXDENSITY_LERP_TO: usize = 0x4C; // float32
                pub const LERPTIME: usize = 0x50; // GameTime_t
                pub const DURATION: usize = 0x54; // float32
                pub const BLENDTOBACKGROUND: usize = 0x58; // float32
                pub const SCATTERING: usize = 0x5C; // float32
                pub const LOCALLIGHTSCALE: usize = 0x60; // float32
                pub const ENABLE: usize = 0x64; // bool
                pub const BLEND: usize = 0x65; // bool
                pub const M_B_NO_REFLECTION_FOG: usize = 0x66; // bool
                pub const M_B_PADDING: usize = 0x67; // bool
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_hCtrl (CHandle< CFogController>)
            pub mod c_fogplayerparams_t {
                pub const M_H_CTRL: usize = 0x8; // CHandle< C_FogController >
                pub const M_FL_TRANSITION_TIME: usize = 0xC; // float32
                pub const M_OLD_COLOR: usize = 0x10; // Color
                pub const M_FL_OLD_START: usize = 0x14; // float32
                pub const M_FL_OLD_END: usize = 0x18; // float32
                pub const M_FL_OLD_MAX_DENSITY: usize = 0x1C; // float32
                pub const M_FL_OLD_HDR_COLOR_SCALE: usize = 0x20; // float32
                pub const M_FL_OLD_FAR_Z: usize = 0x24; // float32
                pub const M_NEW_COLOR: usize = 0x28; // Color
                pub const M_FL_NEW_START: usize = 0x2C; // float32
                pub const M_FL_NEW_END: usize = 0x30; // float32
                pub const M_FL_NEW_MAX_DENSITY: usize = 0x34; // float32
                pub const M_FL_NEW_HDR_COLOR_SCALE: usize = 0x38; // float32
                pub const M_FL_NEW_FAR_Z: usize = 0x3C; // float32
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: scale (int16)
            // NetworkVarNames: origin (Vector)
            // NetworkVarNames: bClip3DSkyBoxNearToWorldFar (bool)
            // NetworkVarNames: flClip3DSkyBoxNearToWorldFarOffset (float32)
            // NetworkVarNames: fog (fogparams_t)
            // NetworkVarNames: m_nWorldGroupID (WorldGroupId_t)
            pub mod sky3dparams_t {
                pub const SCALE: usize = 0x8; // int16
                pub const ORIGIN: usize = 0xC; // 
                pub const B_CLIP3_D_SKY_BOX_NEAR_TO_WORLD_FAR: usize = 0x18; // bool
                pub const FL_CLIP3_D_SKY_BOX_NEAR_TO_WORLD_FAR_OFFSET: usize = 0x1C; // float32
                pub const FOG: usize = 0x20; // fogparams_t
                pub const M_N_WORLD_GROUP_ID: usize = 0x88; // WorldGroupId_t
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_Transforms (CTransform)
            // NetworkVarNames: m_hOwner (EHANDLE)
            pub mod physics_ragdoll_pose_t {
                pub const M_P_CHAIN_ENTITY: usize = 0x8; // CNetworkVarChainer
                pub const M_TRANSFORMS: usize = 0x30; // C_NetworkUtlVectorBase< CTransform >
                pub const M_H_OWNER: usize = 0x48; // CHandle< C_BaseEntity >
                pub const M_B_DIRTY: usize = 0x68; // bool
            }
            // Parent: c_game_rules
            // Fields count: 0
            pub mod c_singleplay_rules {
            }
            // Parent: c_base_entity
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_iszStackName (string_t)
            // NetworkVarNames: m_iszOperatorName (string_t)
            // NetworkVarNames: m_iszOpvarName (string_t)
            // NetworkVarNames: m_iOpvarIndex (int)
            // NetworkVarNames: m_bUseAutoCompare (bool)
            pub mod c_sound_opvar_set_point_base {
                pub const M_ISZ_STACK_NAME: usize = 0x550; // CUtlSymbolLarge
                pub const M_ISZ_OPERATOR_NAME: usize = 0x558; // CUtlSymbolLarge
                pub const M_ISZ_OPVAR_NAME: usize = 0x560; // CUtlSymbolLarge
                pub const M_I_OPVAR_INDEX: usize = 0x568; // int32
                pub const M_B_USE_AUTO_COMPARE: usize = 0x56C; // bool
            }
            // Parent: c_sound_opvar_set_point_base
            // Fields count: 0
            pub mod c_sound_opvar_set_point_entity {
            }
            // Parent: c_sound_opvar_set_point_entity
            // Fields count: 0
            pub mod c_sound_opvar_set_aabb_entity {
            }
            // Parent: c_sound_opvar_set_aabb_entity
            // Fields count: 0
            pub mod c_sound_opvar_set_obb_entity {
            }
            // Parent: c_sound_opvar_set_point_entity
            // Fields count: 0
            pub mod c_sound_opvar_set_path_corner_entity {
            }
            // Parent: c_sound_opvar_set_point_entity
            // Fields count: 0
            pub mod c_sound_opvar_set_auto_room_entity {
            }
            // Parent: c_sound_opvar_set_point_base
            // Fields count: 0
            pub mod c_sound_opvar_set_obb_wind_entity {
            }
            // Parent: c_multiplay_rules
            // Fields count: 0
            pub mod c_teamplay_rules {
            }
            // Parent: c_base_entity
            // Fields count: 30
            //
            // Metadata:
            // NetworkVarNames: m_bTimerPaused (bool)
            // NetworkVarNames: m_flTimeRemaining (float)
            // NetworkVarNames: m_flTimerEndTime (GameTime_t)
            // NetworkVarNames: m_bIsDisabled (bool)
            // NetworkVarNames: m_bShowInHUD (bool)
            // NetworkVarNames: m_nTimerLength (int)
            // NetworkVarNames: m_nTimerInitialLength (int)
            // NetworkVarNames: m_nTimerMaxLength (int)
            // NetworkVarNames: m_bAutoCountdown (bool)
            // NetworkVarNames: m_nSetupTimeLength (int)
            // NetworkVarNames: m_nState (int)
            // NetworkVarNames: m_bStartPaused (bool)
            // NetworkVarNames: m_bInCaptureWatchState (bool)
            // NetworkVarNames: m_flTotalTime (float)
            // NetworkVarNames: m_bStopWatchTimer (bool)
            pub mod c_team_round_timer {
                pub const M_B_TIMER_PAUSED: usize = 0x550; // bool
                pub const M_FL_TIME_REMAINING: usize = 0x554; // float32
                pub const M_FL_TIMER_END_TIME: usize = 0x558; // GameTime_t
                pub const M_B_IS_DISABLED: usize = 0x55C; // bool
                pub const M_B_SHOW_IN_HUD: usize = 0x55D; // bool
                pub const M_N_TIMER_LENGTH: usize = 0x560; // int32
                pub const M_N_TIMER_INITIAL_LENGTH: usize = 0x564; // int32
                pub const M_N_TIMER_MAX_LENGTH: usize = 0x568; // int32
                pub const M_B_AUTO_COUNTDOWN: usize = 0x56C; // bool
                pub const M_N_SETUP_TIME_LENGTH: usize = 0x570; // int32
                pub const M_N_STATE: usize = 0x574; // int32
                pub const M_B_START_PAUSED: usize = 0x578; // bool
                pub const M_B_IN_CAPTURE_WATCH_STATE: usize = 0x579; // bool
                pub const M_FL_TOTAL_TIME: usize = 0x57C; // float32
                pub const M_B_STOP_WATCH_TIMER: usize = 0x580; // bool
                pub const M_B_FIRE_FINISHED: usize = 0x581; // bool
                pub const M_B_FIRE5_MIN_REMAIN: usize = 0x582; // bool
                pub const M_B_FIRE4_MIN_REMAIN: usize = 0x583; // bool
                pub const M_B_FIRE3_MIN_REMAIN: usize = 0x584; // bool
                pub const M_B_FIRE2_MIN_REMAIN: usize = 0x585; // bool
                pub const M_B_FIRE1_MIN_REMAIN: usize = 0x586; // bool
                pub const M_B_FIRE30_SEC_REMAIN: usize = 0x587; // bool
                pub const M_B_FIRE10_SEC_REMAIN: usize = 0x588; // bool
                pub const M_B_FIRE5_SEC_REMAIN: usize = 0x589; // bool
                pub const M_B_FIRE4_SEC_REMAIN: usize = 0x58A; // bool
                pub const M_B_FIRE3_SEC_REMAIN: usize = 0x58B; // bool
                pub const M_B_FIRE2_SEC_REMAIN: usize = 0x58C; // bool
                pub const M_B_FIRE1_SEC_REMAIN: usize = 0x58D; // bool
                pub const M_N_OLD_TIMER_LENGTH: usize = 0x590; // int32
                pub const M_N_OLD_TIMER_STATE: usize = 0x594; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_Attributes (CEconItemAttribute)
            pub mod c_attribute_list {
                pub const M_ATTRIBUTES: usize = 0x8; // C_UtlVectorEmbeddedNetworkVar< CEconItemAttribute >
                pub const M_P_MANAGER: usize = 0x58; // CAttributeManager*
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_iAttributeDefinitionIndex (attrib_definition_index_t)
            // NetworkVarNames: m_flValue (float)
            // NetworkVarNames: m_flInitialValue (float)
            // NetworkVarNames: m_nRefundableCurrency (int)
            // NetworkVarNames: m_bSetBonus (bool)
            pub mod c_econ_item_attribute {
                pub const M_I_ATTRIBUTE_DEFINITION_INDEX: usize = 0x30; // uint16
                pub const M_FL_VALUE: usize = 0x34; // float32
                pub const M_FL_INITIAL_VALUE: usize = 0x38; // float32
                pub const M_N_REFUNDABLE_CURRENCY: usize = 0x3C; // int32
                pub const M_B_SET_BONUS: usize = 0x40; // bool
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: m_iReapplyProvisionParity (int)
            // NetworkVarNames: m_hOuter (EHANDLE)
            // NetworkVarNames: m_ProviderType (attributeprovidertypes_t)
            pub mod c_attribute_manager {
                pub const M_PROVIDERS: usize = 0x8; // CUtlVector< CHandle< C_BaseEntity > >
                pub const M_I_REAPPLY_PROVISION_PARITY: usize = 0x20; // int32
                pub const M_H_OUTER: usize = 0x24; // CHandle< C_BaseEntity >
                pub const M_B_PREVENT_LOOPBACK: usize = 0x28; // bool
                pub const M_PROVIDER_TYPE: usize = 0x2C; // attributeprovidertypes_t
                pub const M_CACHED_RESULTS: usize = 0x30; // CUtlVector< CAttributeManager::cached_attribute_float_t >
            }
            // Parent: None
            // Fields count: 3
            pub mod c_attribute_manager_cached_attribute_float_t {
                pub const FL_IN: usize = 0x0; // float32
                pub const I_ATTRIB_HOOK: usize = 0x8; // CUtlSymbolLarge
                pub const FL_OUT: usize = 0x10; // float32
            }
            // Parent: c_attribute_manager
            // Fields count: 3
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_Item (CEconItemView)
            pub mod c_attribute_container {
                pub const M_ITEM: usize = 0x50; // C_EconItemView
                pub const M_I_EXTERNAL_ITEM_PROVIDER_REGISTERED_TOKEN: usize = 0x498; // int32
                pub const M_ULL_REGISTERED_AS_ITEM_ID: usize = 0x4A0; // uint64
            }
            // Parent: None
            // Fields count: 1
            pub mod c_econ_entity_attached_model_data_t {
                pub const M_I_MODEL_DISPLAY_FLAGS: usize = 0x0; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_bSpotted (bool)
            // NetworkVarNames: m_bSpottedByMask (uint32)
            pub mod entity_spotted_state_t {
                pub const M_B_SPOTTED: usize = 0x8; // bool
                pub const M_B_SPOTTED_BY_MASK: usize = 0xC; // uint32[2]
            }
            // Parent: c_game_rules_proxy
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_pGameRules (CCSGameRules*)
            pub mod c_cs_game_rules_proxy {
                pub const M_P_GAME_RULES: usize = 0x550; // C_CSGameRules*
            }
            // Parent: c_teamplay_rules
            // Fields count: 117
            //
            // Metadata:
            // NetworkVarNames: m_bFreezePeriod (bool)
            // NetworkVarNames: m_bWarmupPeriod (bool)
            // NetworkVarNames: m_fWarmupPeriodEnd (GameTime_t)
            // NetworkVarNames: m_fWarmupPeriodStart (GameTime_t)
            // NetworkVarNames: m_nTotalPausedTicks (int)
            // NetworkVarNames: m_nPauseStartTick (int)
            // NetworkVarNames: m_bServerPaused (bool)
            // NetworkVarNames: m_bGamePaused (bool)
            // NetworkVarNames: m_bTerroristTimeOutActive (bool)
            // NetworkVarNames: m_bCTTimeOutActive (bool)
            // NetworkVarNames: m_flTerroristTimeOutRemaining (float)
            // NetworkVarNames: m_flCTTimeOutRemaining (float)
            // NetworkVarNames: m_nTerroristTimeOuts (int)
            // NetworkVarNames: m_nCTTimeOuts (int)
            // NetworkVarNames: m_bTechnicalTimeOut (bool)
            // NetworkVarNames: m_bMatchWaitingForResume (bool)
            // NetworkVarNames: m_iRoundTime (int)
            // NetworkVarNames: m_fMatchStartTime (float)
            // NetworkVarNames: m_fRoundStartTime (GameTime_t)
            // NetworkVarNames: m_flRestartRoundTime (GameTime_t)
            // NetworkVarNames: m_bGameRestart (bool)
            // NetworkVarNames: m_flGameStartTime (float)
            // NetworkVarNames: m_timeUntilNextPhaseStarts (float)
            // NetworkVarNames: m_gamePhase (int)
            // NetworkVarNames: m_totalRoundsPlayed (int)
            // NetworkVarNames: m_nRoundsPlayedThisPhase (int)
            // NetworkVarNames: m_nOvertimePlaying (int)
            // NetworkVarNames: m_iHostagesRemaining (int)
            // NetworkVarNames: m_bAnyHostageReached (bool)
            // NetworkVarNames: m_bMapHasBombTarget (bool)
            // NetworkVarNames: m_bMapHasRescueZone (bool)
            // NetworkVarNames: m_bMapHasBuyZone (bool)
            // NetworkVarNames: m_bIsQueuedMatchmaking (bool)
            // NetworkVarNames: m_nQueuedMatchmakingMode (int)
            // NetworkVarNames: m_bIsValveDS (bool)
            // NetworkVarNames: m_bLogoMap (bool)
            // NetworkVarNames: m_bPlayAllStepSoundsOnServer (bool)
            // NetworkVarNames: m_iSpectatorSlotCount (int)
            // NetworkVarNames: m_MatchDevice (int)
            // NetworkVarNames: m_bHasMatchStarted (bool)
            // NetworkVarNames: m_nNextMapInMapgroup (int)
            // NetworkVarNames: m_szTournamentEventName (char)
            // NetworkVarNames: m_szTournamentEventStage (char)
            // NetworkVarNames: m_szMatchStatTxt (char)
            // NetworkVarNames: m_szTournamentPredictionsTxt (char)
            // NetworkVarNames: m_nTournamentPredictionsPct (int)
            // NetworkVarNames: m_flCMMItemDropRevealStartTime (GameTime_t)
            // NetworkVarNames: m_flCMMItemDropRevealEndTime (GameTime_t)
            // NetworkVarNames: m_bIsDroppingItems (bool)
            // NetworkVarNames: m_bIsQuestEligible (bool)
            // NetworkVarNames: m_bIsHltvActive (bool)
            // NetworkVarNames: m_nGuardianModeWaveNumber (int)
            // NetworkVarNames: m_nGuardianModeSpecialKillsRemaining (int)
            // NetworkVarNames: m_nGuardianModeSpecialWeaponNeeded (int)
            // NetworkVarNames: m_numGlobalGiftsGiven (uint32)
            // NetworkVarNames: m_numGlobalGifters (uint32)
            // NetworkVarNames: m_numGlobalGiftsPeriodSeconds (uint32)
            // NetworkVarNames: m_arrFeaturedGiftersAccounts (uint32)
            // NetworkVarNames: m_arrFeaturedGiftersGifts (uint32)
            // NetworkVarNames: m_arrProhibitedItemIndices (uint16)
            // NetworkVarNames: m_arrTournamentActiveCasterAccounts (uint32)
            // NetworkVarNames: m_numBestOfMaps (int)
            // NetworkVarNames: m_nHalloweenMaskListSeed (int)
            // NetworkVarNames: m_bBombDropped (bool)
            // NetworkVarNames: m_bBombPlanted (bool)
            // NetworkVarNames: m_iRoundWinStatus (int)
            // NetworkVarNames: m_eRoundWinReason (int)
            // NetworkVarNames: m_bTCantBuy (bool)
            // NetworkVarNames: m_bCTCantBuy (bool)
            // NetworkVarNames: m_flGuardianBuyUntilTime (GameTime_t)
            // NetworkVarNames: m_iMatchStats_RoundResults (int)
            // NetworkVarNames: m_iMatchStats_PlayersAlive_CT (int)
            // NetworkVarNames: m_iMatchStats_PlayersAlive_T (int)
            // NetworkVarNames: m_TeamRespawnWaveTimes (float)
            // NetworkVarNames: m_flNextRespawnWave (GameTime_t)
            // NetworkVarNames: m_nServerQuestID (int)
            // NetworkVarNames: m_vMinimapMins (Vector)
            // NetworkVarNames: m_vMinimapMaxs (Vector)
            // NetworkVarNames: m_MinimapVerticalSectionHeights (float)
            // NetworkVarNames: m_nEndMatchMapGroupVoteTypes (int)
            // NetworkVarNames: m_nEndMatchMapGroupVoteOptions (int)
            // NetworkVarNames: m_nEndMatchMapVoteWinner (int)
            // NetworkVarNames: m_iNumConsecutiveCTLoses (int)
            // NetworkVarNames: m_iNumConsecutiveTerroristLoses (int)
            // NetworkVarNames: m_nMatchAbortedEarlyReason (int)
            // NetworkVarNames: m_pGameModeRules (CCSGameModeRules *)
            // NetworkVarNames: m_RetakeRules (CRetakeGameRules)
            // NetworkVarNames: m_nMatchEndCount (uint8)
            // NetworkVarNames: m_nTTeamIntroVariant (int)
            // NetworkVarNames: m_nCTTeamIntroVariant (int)
            // NetworkVarNames: m_bTeamIntroPeriod (bool)
            // NetworkVarNames: m_iRoundEndWinnerTeam (int)
            // NetworkVarNames: m_eRoundEndReason (int)
            // NetworkVarNames: m_bRoundEndShowTimerDefend (bool)
            // NetworkVarNames: m_iRoundEndTimerTime (int)
            // NetworkVarNames: m_sRoundEndFunFactToken (CUtlString)
            // NetworkVarNames: m_iRoundEndFunFactPlayerSlot (CPlayerSlot)
            // NetworkVarNames: m_iRoundEndFunFactData1 (int)
            // NetworkVarNames: m_iRoundEndFunFactData2 (int)
            // NetworkVarNames: m_iRoundEndFunFactData3 (int)
            // NetworkVarNames: m_sRoundEndMessage (CUtlString)
            // NetworkVarNames: m_iRoundEndPlayerCount (int)
            // NetworkVarNames: m_bRoundEndNoMusic (bool)
            // NetworkVarNames: m_iRoundEndLegacy (int)
            // NetworkVarNames: m_nRoundEndCount (uint8)
            // NetworkVarNames: m_iRoundStartRoundNumber (int)
            // NetworkVarNames: m_nRoundStartCount (uint8)
            pub mod c_cs_game_rules {
                pub const M_P_CHAIN_ENTITY: usize = 0x8; // CNetworkVarChainer
                pub const M_B_FREEZE_PERIOD: usize = 0x30; // bool
                pub const M_B_WARMUP_PERIOD: usize = 0x31; // bool
                pub const M_F_WARMUP_PERIOD_END: usize = 0x34; // GameTime_t
                pub const M_F_WARMUP_PERIOD_START: usize = 0x38; // GameTime_t
                pub const M_N_TOTAL_PAUSED_TICKS: usize = 0x3C; // int32
                pub const M_N_PAUSE_START_TICK: usize = 0x40; // int32
                pub const M_B_SERVER_PAUSED: usize = 0x44; // bool
                pub const M_B_GAME_PAUSED: usize = 0x45; // bool
                pub const M_B_TERRORIST_TIME_OUT_ACTIVE: usize = 0x46; // bool
                pub const M_B_CT_TIME_OUT_ACTIVE: usize = 0x47; // bool
                pub const M_FL_TERRORIST_TIME_OUT_REMAINING: usize = 0x48; // float32
                pub const M_FL_CT_TIME_OUT_REMAINING: usize = 0x4C; // float32
                pub const M_N_TERRORIST_TIME_OUTS: usize = 0x50; // int32
                pub const M_N_CT_TIME_OUTS: usize = 0x54; // int32
                pub const M_B_TECHNICAL_TIME_OUT: usize = 0x58; // bool
                pub const M_B_MATCH_WAITING_FOR_RESUME: usize = 0x59; // bool
                pub const M_I_ROUND_TIME: usize = 0x5C; // int32
                pub const M_F_MATCH_START_TIME: usize = 0x60; // float32
                pub const M_F_ROUND_START_TIME: usize = 0x64; // GameTime_t
                pub const M_FL_RESTART_ROUND_TIME: usize = 0x68; // GameTime_t
                pub const M_B_GAME_RESTART: usize = 0x6C; // bool
                pub const M_FL_GAME_START_TIME: usize = 0x70; // float32
                pub const M_TIME_UNTIL_NEXT_PHASE_STARTS: usize = 0x74; // float32
                pub const M_GAME_PHASE: usize = 0x78; // int32
                pub const M_TOTAL_ROUNDS_PLAYED: usize = 0x7C; // int32
                pub const M_N_ROUNDS_PLAYED_THIS_PHASE: usize = 0x80; // int32
                pub const M_N_OVERTIME_PLAYING: usize = 0x84; // int32
                pub const M_I_HOSTAGES_REMAINING: usize = 0x88; // int32
                pub const M_B_ANY_HOSTAGE_REACHED: usize = 0x8C; // bool
                pub const M_B_MAP_HAS_BOMB_TARGET: usize = 0x8D; // bool
                pub const M_B_MAP_HAS_RESCUE_ZONE: usize = 0x8E; // bool
                pub const M_B_MAP_HAS_BUY_ZONE: usize = 0x8F; // bool
                pub const M_B_IS_QUEUED_MATCHMAKING: usize = 0x90; // bool
                pub const M_N_QUEUED_MATCHMAKING_MODE: usize = 0x94; // int32
                pub const M_B_IS_VALVE_DS: usize = 0x98; // bool
                pub const M_B_LOGO_MAP: usize = 0x99; // bool
                pub const M_B_PLAY_ALL_STEP_SOUNDS_ON_SERVER: usize = 0x9A; // bool
                pub const M_I_SPECTATOR_SLOT_COUNT: usize = 0x9C; // int32
                pub const M_MATCH_DEVICE: usize = 0xA0; // int32
                pub const M_B_HAS_MATCH_STARTED: usize = 0xA4; // bool
                pub const M_N_NEXT_MAP_IN_MAPGROUP: usize = 0xA8; // int32
                pub const M_SZ_TOURNAMENT_EVENT_NAME: usize = 0xAC; // char[512]
                pub const M_SZ_TOURNAMENT_EVENT_STAGE: usize = 0x2AC; // char[512]
                pub const M_SZ_MATCH_STAT_TXT: usize = 0x4AC; // char[512]
                pub const M_SZ_TOURNAMENT_PREDICTIONS_TXT: usize = 0x6AC; // char[512]
                pub const M_N_TOURNAMENT_PREDICTIONS_PCT: usize = 0x8AC; // int32
                pub const M_FL_CMM_ITEM_DROP_REVEAL_START_TIME: usize = 0x8B0; // GameTime_t
                pub const M_FL_CMM_ITEM_DROP_REVEAL_END_TIME: usize = 0x8B4; // GameTime_t
                pub const M_B_IS_DROPPING_ITEMS: usize = 0x8B8; // bool
                pub const M_B_IS_QUEST_ELIGIBLE: usize = 0x8B9; // bool
                pub const M_B_IS_HLTV_ACTIVE: usize = 0x8BA; // bool
                pub const M_N_GUARDIAN_MODE_WAVE_NUMBER: usize = 0x8BC; // int32
                pub const M_N_GUARDIAN_MODE_SPECIAL_KILLS_REMAINING: usize = 0x8C0; // int32
                pub const M_N_GUARDIAN_MODE_SPECIAL_WEAPON_NEEDED: usize = 0x8C4; // int32
                pub const M_N_GUARDIAN_GRENADES_TO_GIVE_BOTS: usize = 0x8C8; // int32
                pub const M_N_NUM_HEAVIES_TO_SPAWN: usize = 0x8CC; // int32
                pub const M_NUM_GLOBAL_GIFTS_GIVEN: usize = 0x8D0; // uint32
                pub const M_NUM_GLOBAL_GIFTERS: usize = 0x8D4; // uint32
                pub const M_NUM_GLOBAL_GIFTS_PERIOD_SECONDS: usize = 0x8D8; // uint32
                pub const M_ARR_FEATURED_GIFTERS_ACCOUNTS: usize = 0x8DC; // uint32[4]
                pub const M_ARR_FEATURED_GIFTERS_GIFTS: usize = 0x8EC; // uint32[4]
                pub const M_ARR_PROHIBITED_ITEM_INDICES: usize = 0x8FC; // uint16[100]
                pub const M_ARR_TOURNAMENT_ACTIVE_CASTER_ACCOUNTS: usize = 0x9C4; // uint32[4]
                pub const M_NUM_BEST_OF_MAPS: usize = 0x9D4; // int32
                pub const M_N_HALLOWEEN_MASK_LIST_SEED: usize = 0x9D8; // int32
                pub const M_B_BOMB_DROPPED: usize = 0x9DC; // bool
                pub const M_B_BOMB_PLANTED: usize = 0x9DD; // bool
                pub const M_I_ROUND_WIN_STATUS: usize = 0x9E0; // int32
                pub const M_E_ROUND_WIN_REASON: usize = 0x9E4; // int32
                pub const M_B_T_CANT_BUY: usize = 0x9E8; // bool
                pub const M_B_CT_CANT_BUY: usize = 0x9E9; // bool
                pub const M_FL_GUARDIAN_BUY_UNTIL_TIME: usize = 0x9EC; // GameTime_t
                pub const M_I_MATCH_STATS_ROUND_RESULTS: usize = 0x9F0; // int32[30]
                pub const M_I_MATCH_STATS_PLAYERS_ALIVE_CT: usize = 0xA68; // int32[30]
                pub const M_I_MATCH_STATS_PLAYERS_ALIVE_T: usize = 0xAE0; // int32[30]
                pub const M_TEAM_RESPAWN_WAVE_TIMES: usize = 0xB58; // float32[32]
                pub const M_FL_NEXT_RESPAWN_WAVE: usize = 0xBD8; // GameTime_t[32]
                pub const M_N_SERVER_QUEST_ID: usize = 0xC58; // int32
                pub const M_V_MINIMAP_MINS: usize = 0xC5C; // 
                pub const M_V_MINIMAP_MAXS: usize = 0xC68; // 
                pub const M_MINIMAP_VERTICAL_SECTION_HEIGHTS: usize = 0xC74; // float32[8]
                pub const M_B_DONT_INCREMENT_COOP_WAVE: usize = 0xC94; // bool
                pub const M_B_SPAWNED_TERROR_HUNT_HEAVY: usize = 0xC95; // bool
                pub const M_N_END_MATCH_MAP_GROUP_VOTE_TYPES: usize = 0xC98; // int32[10]
                pub const M_N_END_MATCH_MAP_GROUP_VOTE_OPTIONS: usize = 0xCC0; // int32[10]
                pub const M_N_END_MATCH_MAP_VOTE_WINNER: usize = 0xCE8; // int32
                pub const M_I_NUM_CONSECUTIVE_CT_LOSES: usize = 0xCEC; // int32
                pub const M_I_NUM_CONSECUTIVE_TERRORIST_LOSES: usize = 0xCF0; // int32
                pub const M_B_MARK_CLIENT_STOP_RECORD_AT_ROUND_END: usize = 0xD10; // bool
                pub const M_N_MATCH_ABORTED_EARLY_REASON: usize = 0xD68; // int32
                pub const M_B_HAS_TRIGGERED_ROUND_START_MUSIC: usize = 0xD6C; // bool
                pub const M_B_HAS_TRIGGERED_COOP_SPAWN_RESET: usize = 0xD6D; // bool
                pub const M_B_SWITCHING_TEAMS_AT_ROUND_RESET: usize = 0xD6E; // bool
                pub const M_P_GAME_MODE_RULES: usize = 0xD88; // CCSGameModeRules*
                pub const M_RETAKE_RULES: usize = 0xD90; // C_RetakeGameRules
                pub const M_N_MATCH_END_COUNT: usize = 0xEA8; // uint8
                pub const M_N_T_TEAM_INTRO_VARIANT: usize = 0xEAC; // int32
                pub const M_N_CT_TEAM_INTRO_VARIANT: usize = 0xEB0; // int32
                pub const M_B_TEAM_INTRO_PERIOD: usize = 0xEB4; // bool
                pub const M_I_ROUND_END_WINNER_TEAM: usize = 0xEB8; // int32
                pub const M_E_ROUND_END_REASON: usize = 0xEBC; // int32
                pub const M_B_ROUND_END_SHOW_TIMER_DEFEND: usize = 0xEC0; // bool
                pub const M_I_ROUND_END_TIMER_TIME: usize = 0xEC4; // int32
                pub const M_S_ROUND_END_FUN_FACT_TOKEN: usize = 0xEC8; // CUtlString
                pub const M_I_ROUND_END_FUN_FACT_PLAYER_SLOT: usize = 0xED0; // CPlayerSlot
                pub const M_I_ROUND_END_FUN_FACT_DATA1: usize = 0xED4; // int32
                pub const M_I_ROUND_END_FUN_FACT_DATA2: usize = 0xED8; // int32
                pub const M_I_ROUND_END_FUN_FACT_DATA3: usize = 0xEDC; // int32
                pub const M_S_ROUND_END_MESSAGE: usize = 0xEE0; // CUtlString
                pub const M_I_ROUND_END_PLAYER_COUNT: usize = 0xEE8; // int32
                pub const M_B_ROUND_END_NO_MUSIC: usize = 0xEEC; // bool
                pub const M_I_ROUND_END_LEGACY: usize = 0xEF0; // int32
                pub const M_N_ROUND_END_COUNT: usize = 0xEF4; // uint8
                pub const M_I_ROUND_START_ROUND_NUMBER: usize = 0xEF8; // int32
                pub const M_N_ROUND_START_COUNT: usize = 0xEFC; // uint8
                pub const M_FL_LAST_PERF_SAMPLE_TIME: usize = 0x4F08; // float64
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod ccs_take_damage_info_api {
            }
            // Parent: None
            // Fields count: 1
            pub mod ccs_game_mode_rules {
                pub const M_P_CHAIN_ENTITY: usize = 0x8; // CNetworkVarChainer
            }
            // Parent: ccs_game_mode_rules
            // Fields count: 0
            pub mod ccs_game_mode_rules_noop {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod ccs_game_mode_script {
            }
            // Parent: ccs_game_mode_rules
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_WeaponSequence (CUtlString)
            pub mod ccs_game_mode_rules_arms_race {
                pub const M_WEAPON_SEQUENCE: usize = 0x38; // C_NetworkUtlVectorBase< CUtlString >
            }
            // Parent: ccs_game_mode_script
            // Fields count: 1
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod ccs_arms_race_script {
                pub const M_P_OUTER: usize = 0xD8; // CCSGameModeRules_ArmsRace*
            }
            // Parent: ccs_game_mode_rules
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_flDMBonusStartTime (GameTime_t)
            // NetworkVarNames: m_flDMBonusTimeLength (float)
            // NetworkVarNames: m_nDMBonusWeaponLoadoutSlot (int16)
            pub mod ccs_game_mode_rules_deathmatch {
                pub const M_FL_DM_BONUS_START_TIME: usize = 0x38; // GameTime_t
                pub const M_FL_DM_BONUS_TIME_LENGTH: usize = 0x3C; // float32
                pub const M_N_DM_BONUS_WEAPON_LOADOUT_SLOT: usize = 0x40; // int16
            }
            // Parent: ccs_game_mode_script
            // Fields count: 1
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod ccs_deathmatch_script {
                pub const M_P_OUTER: usize = 0xD8; // CCSGameModeRules_Deathmatch*
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_nMatchSeed (int)
            // NetworkVarNames: m_bBlockersPresent (bool)
            // NetworkVarNames: m_bRoundInProgress (bool)
            // NetworkVarNames: m_iFirstSecondHalfRound (int)
            // NetworkVarNames: m_iBombSite (int)
            pub mod c_retake_game_rules {
                pub const M_N_MATCH_SEED: usize = 0xF8; // int32
                pub const M_B_BLOCKERS_PRESENT: usize = 0xFC; // bool
                pub const M_B_ROUND_IN_PROGRESS: usize = 0xFD; // bool
                pub const M_I_FIRST_SECOND_HALF_ROUND: usize = 0x100; // int32
                pub const M_I_BOMB_SITE: usize = 0x104; // int32
            }
            // Parent: None
            // Fields count: 13
            //
            // Metadata:
            // NetworkVarNames: m_iKills (int)
            // NetworkVarNames: m_iDeaths (int)
            // NetworkVarNames: m_iAssists (int)
            // NetworkVarNames: m_iDamage (int)
            // NetworkVarNames: m_iEquipmentValue (int)
            // NetworkVarNames: m_iMoneySaved (int)
            // NetworkVarNames: m_iKillReward (int)
            // NetworkVarNames: m_iLiveTime (int)
            // NetworkVarNames: m_iHeadShotKills (int)
            // NetworkVarNames: m_iObjective (int)
            // NetworkVarNames: m_iCashEarned (int)
            // NetworkVarNames: m_iUtilityDamage (int)
            // NetworkVarNames: m_iEnemiesFlashed (int)
            pub mod cs_per_round_stats_t {
                pub const M_I_KILLS: usize = 0x30; // int32
                pub const M_I_DEATHS: usize = 0x34; // int32
                pub const M_I_ASSISTS: usize = 0x38; // int32
                pub const M_I_DAMAGE: usize = 0x3C; // int32
                pub const M_I_EQUIPMENT_VALUE: usize = 0x40; // int32
                pub const M_I_MONEY_SAVED: usize = 0x44; // int32
                pub const M_I_KILL_REWARD: usize = 0x48; // int32
                pub const M_I_LIVE_TIME: usize = 0x4C; // int32
                pub const M_I_HEAD_SHOT_KILLS: usize = 0x50; // int32
                pub const M_I_OBJECTIVE: usize = 0x54; // int32
                pub const M_I_CASH_EARNED: usize = 0x58; // int32
                pub const M_I_UTILITY_DAMAGE: usize = 0x5C; // int32
                pub const M_I_ENEMIES_FLASHED: usize = 0x60; // int32
            }
            // Parent: cs_per_round_stats_t
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_iEnemy5Ks (int)
            // NetworkVarNames: m_iEnemy4Ks (int)
            // NetworkVarNames: m_iEnemy3Ks (int)
            // NetworkVarNames: m_iEnemyKnifeKills (int)
            // NetworkVarNames: m_iEnemyTaserKills (int)
            pub mod cs_match_stats_t {
                pub const M_I_ENEMY5_KS: usize = 0x68; // int32
                pub const M_I_ENEMY4_KS: usize = 0x6C; // int32
                pub const M_I_ENEMY3_KS: usize = 0x70; // int32
                pub const M_I_ENEMY_KNIFE_KILLS: usize = 0x74; // int32
                pub const M_I_ENEMY_TASER_KILLS: usize = 0x78; // int32
            }
            // Parent: c_base_entity
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_nVariant (int)
            // NetworkVarNames: m_nRandom (int)
            // NetworkVarNames: m_nOrdinal (int)
            // NetworkVarNames: m_sWeaponName (CUtlString)
            // NetworkVarNames: m_xuid (XUID)
            // NetworkVarNames: m_agentItem (CEconItemView)
            // NetworkVarNames: m_glovesItem (CEconItemView)
            // NetworkVarNames: m_weaponItem (CEconItemView)
            pub mod c_csgo_team_preview_character_position {
                pub const M_N_VARIANT: usize = 0x550; // int32
                pub const M_N_RANDOM: usize = 0x554; // int32
                pub const M_N_ORDINAL: usize = 0x558; // int32
                pub const M_S_WEAPON_NAME: usize = 0x560; // CUtlString
                pub const M_XUID: usize = 0x568; // uint64
                pub const M_AGENT_ITEM: usize = 0x570; // C_EconItemView
                pub const M_GLOVES_ITEM: usize = 0x9B8; // C_EconItemView
                pub const M_WEAPON_ITEM: usize = 0xE00; // C_EconItemView
            }
            // Parent: c_csgo_team_preview_character_position
            // Fields count: 0
            pub mod c_csgo_team_select_character_position {
            }
            // Parent: c_csgo_team_select_character_position
            // Fields count: 0
            pub mod c_csgo_team_select_terrorist_position {
            }
            // Parent: c_csgo_team_select_character_position
            // Fields count: 0
            pub mod c_csgo_team_select_counter_terrorist_position {
            }
            // Parent: c_csgo_team_preview_character_position
            // Fields count: 0
            pub mod c_csgo_team_intro_character_position {
            }
            // Parent: c_csgo_team_intro_character_position
            // Fields count: 0
            pub mod c_csgo_team_intro_terrorist_position {
            }
            // Parent: c_csgo_team_intro_character_position
            // Fields count: 0
            pub mod c_csgo_team_intro_counter_terrorist_position {
            }
            // Parent: c_csgo_team_intro_character_position
            // Fields count: 0
            pub mod ccsgo_wingman_intro_character_position {
            }
            // Parent: ccsgo_wingman_intro_character_position
            // Fields count: 0
            pub mod ccsgo_wingman_intro_terrorist_position {
            }
            // Parent: ccsgo_wingman_intro_character_position
            // Fields count: 0
            pub mod ccsgo_wingman_intro_counter_terrorist_position {
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_cs_minimap_boundary {
            }
            // Parent: c_base_entity
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_hPlayer (CHandle< CCSPlayerPawn>)
            // NetworkVarNames: m_hPingedEntity (CHandle< CBaseEntity>)
            // NetworkVarNames: m_iType (int)
            // NetworkVarNames: m_bUrgent (bool)
            // NetworkVarNames: m_szPlaceName (char)
            pub mod c_player_ping {
                pub const M_H_PLAYER: usize = 0x580; // CHandle< C_CSPlayerPawn >
                pub const M_H_PINGED_ENTITY: usize = 0x584; // CHandle< C_BaseEntity >
                pub const M_I_TYPE: usize = 0x588; // int32
                pub const M_B_URGENT: usize = 0x58C; // bool
                pub const M_SZ_PLACE_NAME: usize = 0x58D; // char[18]
            }
            // Parent: c_player_pawn_component
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_hPlayerPing (CHandle< CBaseEntity>)
            pub mod ccs_player_ping_services {
                pub const M_H_PLAYER_PING: usize = 0x40; // CHandle< C_BaseEntity >
            }
            // Parent: c_base_entity
            // Fields count: 10
            //
            // Metadata:
            // NetworkVarNames: m_bHostageAlive (bool)
            // NetworkVarNames: m_isHostageFollowingSomeone (bool)
            // NetworkVarNames: m_iHostageEntityIDs (CEntityIndex)
            // NetworkVarNames: m_bombsiteCenterA (Vector)
            // NetworkVarNames: m_bombsiteCenterB (Vector)
            // NetworkVarNames: m_hostageRescueX (int)
            // NetworkVarNames: m_hostageRescueY (int)
            // NetworkVarNames: m_hostageRescueZ (int)
            // NetworkVarNames: m_bEndMatchNextMapAllVoted (bool)
            pub mod c_cs_player_resource {
                pub const M_B_HOSTAGE_ALIVE: usize = 0x550; // bool[12]
                pub const M_IS_HOSTAGE_FOLLOWING_SOMEONE: usize = 0x55C; // bool[12]
                pub const M_I_HOSTAGE_ENTITY_I_DS: usize = 0x568; // CEntityIndex[12]
                pub const M_BOMBSITE_CENTER_A: usize = 0x598; // 
                pub const M_BOMBSITE_CENTER_B: usize = 0x5A4; // 
                pub const M_HOSTAGE_RESCUE_X: usize = 0x5B0; // int32[4]
                pub const M_HOSTAGE_RESCUE_Y: usize = 0x5C0; // int32[4]
                pub const M_HOSTAGE_RESCUE_Z: usize = 0x5D0; // int32[4]
                pub const M_B_END_MATCH_NEXT_MAP_ALL_VOTED: usize = 0x5E0; // bool
                pub const M_FOUND_GOAL_POSITIONS: usize = 0x5E1; // bool
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod ccs_player_controller_api {
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_view_model_services {
            }
            // Parent: c_player_camera_services
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: m_iFOV (uint32)
            // NetworkVarNames: m_iFOVStart (uint32)
            // NetworkVarNames: m_flFOVTime (GameTime_t)
            // NetworkVarNames: m_flFOVRate (float32)
            // NetworkVarNames: m_hZoomOwner (CHandle< CBaseEntity>)
            pub mod ccs_player_base_camera_services {
                pub const M_I_FOV: usize = 0x210; // uint32
                pub const M_I_FOV_START: usize = 0x214; // uint32
                pub const M_FL_FOV_TIME: usize = 0x218; // GameTime_t
                pub const M_FL_FOV_RATE: usize = 0x21C; // float32
                pub const M_H_ZOOM_OWNER: usize = 0x220; // CHandle< C_BaseEntity >
                pub const M_FL_LAST_SHOT_FOV: usize = 0x224; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_nItemDefIndex (uint16)
            // NetworkVarNames: m_nCount (uint16)
            pub mod weapon_purchase_count_t {
                pub const M_N_ITEM_DEF_INDEX: usize = 0x30; // uint16
                pub const M_N_COUNT: usize = 0x32; // uint16
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_weaponPurchases (WeaponPurchaseCount_t)
            pub mod weapon_purchase_tracker_t {
                pub const M_WEAPON_PURCHASES: usize = 0x8; // C_UtlVectorEmbeddedNetworkVar< WeaponPurchaseCount_t >
            }
            // Parent: c_player_pawn_component
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_bIsRescuing (bool)
            // NetworkVarNames: m_weaponPurchasesThisMatch (WeaponPurchaseTracker_t)
            // NetworkVarNames: m_weaponPurchasesThisRound (WeaponPurchaseTracker_t)
            pub mod ccs_player_action_tracking_services {
                pub const M_H_LAST_WEAPON_BEFORE_C4_AUTO_SWITCH: usize = 0x40; // CHandle< C_BasePlayerWeapon >
                pub const M_B_IS_RESCUING: usize = 0x44; // bool
                pub const M_WEAPON_PURCHASES_THIS_MATCH: usize = 0x48; // WeaponPurchaseTracker_t
                pub const M_WEAPON_PURCHASES_THIS_ROUND: usize = 0xA0; // WeaponPurchaseTracker_t
            }
            // Parent: c_player_pawn_component
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_totalHitsOnServer (int32)
            pub mod ccs_player_bullet_services {
                pub const M_TOTAL_HITS_ON_SERVER: usize = 0x40; // int32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_unDefIdx (item_definition_index_t)
            // NetworkVarNames: m_nCost (int)
            // NetworkVarNames: m_nPrevArmor (int)
            // NetworkVarNames: m_bPrevHelmet (bool)
            // NetworkVarNames: m_hItem (CEntityHandle)
            pub mod sellback_purchase_entry_t {
                pub const M_UN_DEF_IDX: usize = 0x30; // uint16
                pub const M_N_COST: usize = 0x34; // int32
                pub const M_N_PREV_ARMOR: usize = 0x38; // int32
                pub const M_B_PREV_HELMET: usize = 0x3C; // bool
                pub const M_H_ITEM: usize = 0x40; // CEntityHandle
            }
            // Parent: c_player_pawn_component
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_vecSellbackPurchaseEntries (SellbackPurchaseEntry_t)
            pub mod ccs_player_buy_services {
                pub const M_VEC_SELLBACK_PURCHASE_ENTRIES: usize = 0x40; // C_UtlVectorEmbeddedNetworkVar< SellbackPurchaseEntry_t >
            }
            // Parent: ccs_player_base_camera_services
            // Fields count: 1
            pub mod ccs_player_camera_services {
                pub const M_FL_DEATH_CAM_TILT: usize = 0x228; // float32
            }
            // Parent: c_player_pawn_component
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hCarriedHostage (CHandle< CBaseEntity>)
            // NetworkVarNames: m_hCarriedHostageProp (CHandle< CBaseEntity>)
            pub mod ccs_player_hostage_services {
                pub const M_H_CARRIED_HOSTAGE: usize = 0x40; // CHandle< C_BaseEntity >
                pub const M_H_CARRIED_HOSTAGE_PROP: usize = 0x44; // CHandle< C_BaseEntity >
            }
            // Parent: c_player_item_services
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_bHasDefuser (bool)
            // NetworkVarNames: m_bHasHelmet (bool)
            // NetworkVarNames: m_bHasHeavyArmor (bool)
            pub mod ccs_player_item_services {
                pub const M_B_HAS_DEFUSER: usize = 0x40; // bool
                pub const M_B_HAS_HELMET: usize = 0x41; // bool
                pub const M_B_HAS_HEAVY_ARMOR: usize = 0x42; // bool
            }
            // Parent: c_player_movement_services_humanoid
            // Fields count: 37
            //
            // Metadata:
            // NetworkVarNames: m_flMaxFallVelocity (float)
            // NetworkVarNames: m_vecLadderNormal (Vector)
            // NetworkVarNames: m_nLadderSurfacePropIndex (int)
            // NetworkVarNames: m_flDuckAmount (float)
            // NetworkVarNames: m_flDuckSpeed (float)
            // NetworkVarNames: m_bDuckOverride (bool)
            // NetworkVarNames: m_bDesiresDuck (bool)
            // NetworkVarNames: m_nDuckTimeMsecs (uint32)
            // NetworkVarNames: m_nDuckJumpTimeMsecs (uint32)
            // NetworkVarNames: m_nJumpTimeMsecs (uint32)
            // NetworkVarNames: m_flLastDuckTime (float)
            // NetworkVarNames: m_nGameCodeHasMovedPlayerAfterCommand (int)
            // NetworkVarNames: m_bOldJumpPressed (bool)
            // NetworkVarNames: m_flJumpUntil (float)
            // NetworkVarNames: m_flJumpVel (float)
            // NetworkVarNames: m_fStashGrenadeParameterWhen (GameTime_t)
            // NetworkVarNames: m_nButtonDownMaskPrev (ButtonBitMask_t)
            // NetworkVarNames: m_flOffsetTickCompleteTime (float)
            // NetworkVarNames: m_flOffsetTickStashedSpeed (float)
            // NetworkVarNames: m_flStamina (float)
            pub mod ccs_player_movement_services {
                pub const M_FL_MAX_FALL_VELOCITY: usize = 0x218; // float32
                pub const M_VEC_LADDER_NORMAL: usize = 0x21C; // 
                pub const M_N_LADDER_SURFACE_PROP_INDEX: usize = 0x228; // int32
                pub const M_FL_DUCK_AMOUNT: usize = 0x22C; // float32
                pub const M_FL_DUCK_SPEED: usize = 0x230; // float32
                pub const M_B_DUCK_OVERRIDE: usize = 0x234; // bool
                pub const M_B_DESIRES_DUCK: usize = 0x235; // bool
                pub const M_FL_DUCK_OFFSET: usize = 0x238; // float32
                pub const M_N_DUCK_TIME_MSECS: usize = 0x23C; // uint32
                pub const M_N_DUCK_JUMP_TIME_MSECS: usize = 0x240; // uint32
                pub const M_N_JUMP_TIME_MSECS: usize = 0x244; // uint32
                pub const M_FL_LAST_DUCK_TIME: usize = 0x248; // float32
                pub const M_VEC_LAST_POSITION_AT_FULL_CROUCH_SPEED: usize = 0x258; // Vector2D
                pub const M_DUCK_UNTIL_ON_GROUND: usize = 0x260; // bool
                pub const M_B_HAS_WALK_MOVED_SINCE_LAST_JUMP: usize = 0x261; // bool
                pub const M_B_IN_STUCK_TEST: usize = 0x262; // bool
                pub const M_FL_STUCK_CHECK_TIME: usize = 0x270; // float32[64][2]
                pub const M_N_TRACE_COUNT: usize = 0x470; // int32
                pub const M_STUCK_LAST: usize = 0x474; // int32
                pub const M_B_SPEED_CROPPED: usize = 0x478; // bool
                pub const M_N_OLD_WATER_LEVEL: usize = 0x47C; // int32
                pub const M_FL_WATER_ENTRY_TIME: usize = 0x480; // float32
                pub const M_VEC_FORWARD: usize = 0x484; // 
                pub const M_VEC_LEFT: usize = 0x490; // 
                pub const M_VEC_UP: usize = 0x49C; // 
                pub const M_N_GAME_CODE_HAS_MOVED_PLAYER_AFTER_COMMAND: usize = 0x4A8; // int32
                pub const M_B_OLD_JUMP_PRESSED: usize = 0x4AC; // bool
                pub const M_FL_JUMP_PRESSED_TIME: usize = 0x4B0; // float32
                pub const M_FL_JUMP_UNTIL: usize = 0x4B4; // float32
                pub const M_FL_JUMP_VEL: usize = 0x4B8; // float32
                pub const M_F_STASH_GRENADE_PARAMETER_WHEN: usize = 0x4BC; // GameTime_t
                pub const M_N_BUTTON_DOWN_MASK_PREV: usize = 0x4C0; // uint64
                pub const M_FL_OFFSET_TICK_COMPLETE_TIME: usize = 0x4C8; // float32
                pub const M_FL_OFFSET_TICK_STASHED_SPEED: usize = 0x4CC; // float32
                pub const M_FL_STAMINA: usize = 0x4D0; // float32
                pub const M_FL_HEIGHT_AT_JUMP_START: usize = 0x4D4; // float32
                pub const M_FL_MAX_JUMP_HEIGHT_THIS_JUMP: usize = 0x4D8; // float32
            }
            // Parent: c_player_use_services
            // Fields count: 0
            pub mod ccs_player_use_services {
            }
            // Parent: c_player_view_model_services
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_hViewModel (CHandle< CBaseViewModel >)
            pub mod ccs_player_view_model_services {
                pub const M_H_VIEW_MODEL: usize = 0x40; // CHandle< C_BaseViewModel >[3]
            }
            // Parent: c_player_water_services
            // Fields count: 3
            pub mod ccs_player_water_services {
                pub const M_FL_WATER_JUMP_TIME: usize = 0x40; // float32
                pub const M_VEC_WATER_JUMP_VEL: usize = 0x44; // 
                pub const M_FL_SWIM_SOUND_TIME: usize = 0x50; // float32
            }
            // Parent: c_player_weapon_services
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_flNextAttack (GameTime_t)
            // NetworkVarNames: m_bIsLookingAtWeapon (bool)
            // NetworkVarNames: m_bIsHoldingLookAtWeapon (bool)
            pub mod ccs_player_weapon_services {
                pub const M_FL_NEXT_ATTACK: usize = 0xB8; // GameTime_t
                pub const M_B_IS_LOOKING_AT_WEAPON: usize = 0xBC; // bool
                pub const M_B_IS_HOLDING_LOOK_AT_WEAPON: usize = 0xBD; // bool
                pub const M_N_OLD_SHOOT_POSITION_HISTORY_COUNT: usize = 0xC0; // uint32
                pub const M_N_OLD_INPUT_HISTORY_COUNT: usize = 0x458; // uint32
            }
            // Parent: c_player_observer_services
            // Fields count: 8
            pub mod ccs_observer_observer_services {
                pub const M_H_LAST_OBSERVER_TARGET: usize = 0x58; // CEntityHandle
                pub const M_VEC_OBSERVER_INTERPOLATE_OFFSET: usize = 0x5C; // 
                pub const M_VEC_OBSERVER_INTERP_START_POS: usize = 0x68; // 
                pub const M_FL_OBS_INTERP_PATH_LENGTH: usize = 0x74; // float32
                pub const M_Q_OBS_INTERP_ORIENTATION_START: usize = 0x80; // Quaternion
                pub const M_Q_OBS_INTERP_ORIENTATION_TRAVEL_DIR: usize = 0x90; // Quaternion
                pub const M_OBS_INTERP_STATE: usize = 0xA0; // ObserverInterpState_t
                pub const M_B_OBSERVER_INTERPOLATION_NEEDS_DEFERRED_SETUP: usize = 0xA4; // bool
            }
            // Parent: ccs_player_base_camera_services
            // Fields count: 0
            pub mod ccs_observer_camera_services {
            }
            // Parent: c_player_movement_services
            // Fields count: 0
            pub mod ccs_observer_movement_services {
            }
            // Parent: c_player_use_services
            // Fields count: 0
            pub mod ccs_observer_use_services {
            }
            // Parent: c_player_view_model_services
            // Fields count: 0
            pub mod ccs_observer_view_model_services {
            }
            // Parent: c_player_controller_component
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_perRoundStats (CSPerRoundStats_t)
            // NetworkVarNames: m_matchStats (CSMatchStats_t)
            // NetworkVarNames: m_iNumRoundKills (int)
            // NetworkVarNames: m_iNumRoundKillsHeadshots (int)
            // NetworkVarNames: m_unTotalRoundDamageDealt (uint32)
            pub mod ccs_player_controller_action_tracking_services {
                pub const M_PER_ROUND_STATS: usize = 0x40; // C_UtlVectorEmbeddedNetworkVar< CSPerRoundStats_t >
                pub const M_MATCH_STATS: usize = 0x90; // CSMatchStats_t
                pub const M_I_NUM_ROUND_KILLS: usize = 0x110; // int32
                pub const M_I_NUM_ROUND_KILLS_HEADSHOTS: usize = 0x114; // int32
                pub const M_UN_TOTAL_ROUND_DAMAGE_DEALT: usize = 0x118; // uint32
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_PlayerDamager (CHandle< CCSPlayerPawnBase >)
            // NetworkVarNames: m_PlayerRecipient (CHandle< CCSPlayerPawnBase >)
            // NetworkVarNames: m_hPlayerControllerDamager (CHandle< CCSPlayerController >)
            // NetworkVarNames: m_hPlayerControllerRecipient (CHandle< CCSPlayerController >)
            // NetworkVarNames: m_szPlayerDamagerName (CUtlString)
            // NetworkVarNames: m_szPlayerRecipientName (CUtlString)
            // NetworkVarNames: m_DamagerXuid (uint64)
            // NetworkVarNames: m_RecipientXuid (uint64)
            // NetworkVarNames: m_iDamage (int)
            // NetworkVarNames: m_iActualHealthRemoved (int)
            // NetworkVarNames: m_iNumHits (int)
            // NetworkVarNames: m_iLastBulletUpdate (int)
            // NetworkVarNames: m_bIsOtherEnemy (bool)
            // NetworkVarNames: m_killType (EKillTypes_t)
            pub mod c_damage_record {
                pub const M_PLAYER_DAMAGER: usize = 0x28; // CHandle< C_CSPlayerPawnBase >
                pub const M_PLAYER_RECIPIENT: usize = 0x2C; // CHandle< C_CSPlayerPawnBase >
                pub const M_H_PLAYER_CONTROLLER_DAMAGER: usize = 0x30; // CHandle< CCSPlayerController >
                pub const M_H_PLAYER_CONTROLLER_RECIPIENT: usize = 0x34; // CHandle< CCSPlayerController >
                pub const M_SZ_PLAYER_DAMAGER_NAME: usize = 0x38; // CUtlString
                pub const M_SZ_PLAYER_RECIPIENT_NAME: usize = 0x40; // CUtlString
                pub const M_DAMAGER_XUID: usize = 0x48; // uint64
                pub const M_RECIPIENT_XUID: usize = 0x50; // uint64
                pub const M_I_DAMAGE: usize = 0x58; // int32
                pub const M_I_ACTUAL_HEALTH_REMOVED: usize = 0x5C; // int32
                pub const M_I_NUM_HITS: usize = 0x60; // int32
                pub const M_I_LAST_BULLET_UPDATE: usize = 0x64; // int32
                pub const M_B_IS_OTHER_ENEMY: usize = 0x68; // bool
                pub const M_KILL_TYPE: usize = 0x69; // EKillTypes_t
            }
            // Parent: c_player_controller_component
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_nSendUpdate (int)
            // NetworkVarNames: m_DamageList (CDamageRecord)
            pub mod ccs_player_controller_damage_services {
                pub const M_N_SEND_UPDATE: usize = 0x40; // int32
                pub const M_DAMAGE_LIST: usize = 0x48; // C_UtlVectorEmbeddedNetworkVar< CDamageRecord >
            }
            // Parent: c_player_controller_component
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_iAccount (int)
            // NetworkVarNames: m_iStartAccount (int)
            // NetworkVarNames: m_iTotalCashSpent (int)
            // NetworkVarNames: m_iCashSpentThisRound (int)
            pub mod ccs_player_controller_in_game_money_services {
                pub const M_I_ACCOUNT: usize = 0x40; // int32
                pub const M_I_START_ACCOUNT: usize = 0x44; // int32
                pub const M_I_TOTAL_CASH_SPENT: usize = 0x48; // int32
                pub const M_I_CASH_SPENT_THIS_ROUND: usize = 0x4C; // int32
                pub const M_N_PREVIOUS_ACCOUNT: usize = 0x50; // int32
            }
            // Parent: None
            // Fields count: 3
            pub mod server_authoritative_weapon_slot_t {
                pub const UN_CLASS: usize = 0x28; // uint16
                pub const UN_SLOT: usize = 0x2A; // uint16
                pub const UN_ITEM_DEF_IDX: usize = 0x2C; // uint16
            }
            // Parent: c_player_controller_component
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_unMusicID (item_definition_index_t)
            // NetworkVarNames: m_rank (MedalRank_t)
            // NetworkVarNames: m_nPersonaDataPublicLevel (int)
            // NetworkVarNames: m_nPersonaDataPublicCommendsLeader (int)
            // NetworkVarNames: m_nPersonaDataPublicCommendsTeacher (int)
            // NetworkVarNames: m_nPersonaDataPublicCommendsFriendly (int)
            // NetworkVarNames: m_nPersonaDataXpTrailLevel (int)
            // NetworkVarNames: m_vecServerAuthoritativeWeaponSlots (ServerAuthoritativeWeaponSlot_t)
            pub mod ccs_player_controller_inventory_services {
                pub const M_UN_MUSIC_ID: usize = 0x40; // uint16
                pub const M_RANK: usize = 0x44; // MedalRank_t[6]
                pub const M_N_PERSONA_DATA_PUBLIC_LEVEL: usize = 0x5C; // int32
                pub const M_N_PERSONA_DATA_PUBLIC_COMMENDS_LEADER: usize = 0x60; // int32
                pub const M_N_PERSONA_DATA_PUBLIC_COMMENDS_TEACHER: usize = 0x64; // int32
                pub const M_N_PERSONA_DATA_PUBLIC_COMMENDS_FRIENDLY: usize = 0x68; // int32
                pub const M_N_PERSONA_DATA_XP_TRAIL_LEVEL: usize = 0x6C; // int32
                pub const M_VEC_SERVER_AUTHORITATIVE_WEAPON_SLOTS: usize = 0x70; // C_UtlVectorEmbeddedNetworkVar< ServerAuthoritativeWeaponSlot_t >
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod ccs_weapon_base_v_data_api {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod ccs_weapon_base_api {
            }
            // Parent: None
            // Fields count: 13
            pub mod c_iron_sight_controller {
                pub const M_B_IRON_SIGHT_AVAILABLE: usize = 0x10; // bool
                pub const M_FL_IRON_SIGHT_AMOUNT: usize = 0x14; // float32
                pub const M_FL_IRON_SIGHT_AMOUNT_GAINED: usize = 0x18; // float32
                pub const M_FL_IRON_SIGHT_AMOUNT_BIASED: usize = 0x1C; // float32
                pub const M_FL_IRON_SIGHT_AMOUNT_INTERPOLATED: usize = 0x20; // float32
                pub const M_FL_IRON_SIGHT_AMOUNT_GAINED_INTERPOLATED: usize = 0x24; // float32
                pub const M_FL_IRON_SIGHT_AMOUNT_BIASED_INTERPOLATED: usize = 0x28; // float32
                pub const M_FL_INTERPOLATION_LAST_UPDATED: usize = 0x2C; // float32
                pub const M_ANG_DELTA_AVERAGE: usize = 0x30; // QAngle[8]
                pub const M_ANG_VIEW_LAST: usize = 0x90; // QAngle
                pub const M_VEC_DOT_COORDS: usize = 0x9C; // Vector2D
                pub const M_FL_DOT_BLUR: usize = 0xA4; // float32
                pub const M_FL_SPEED_RATIO: usize = 0xA8; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyElementNameFn
            pub mod composite_material_match_filter_t {
                pub const M_N_COMPOSITE_MATERIAL_MATCH_FILTER_TYPE: usize = 0x0; // CompositeMaterialMatchFilterType_t
                pub const M_STR_MATCH_FILTER: usize = 0x8; // CUtlString
                pub const M_STR_MATCH_VALUE: usize = 0x10; // CUtlString
                pub const M_B_PASS_WHEN_TRUE: usize = 0x18; // bool
            }
            // Parent: None
            // Fields count: 34
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyElementNameFn
            pub mod composite_material_input_loose_variable_t {
                pub const M_STR_NAME: usize = 0x0; // CUtlString
                pub const M_B_EXPOSE_EXTERNALLY: usize = 0x8; // bool
                pub const M_STR_EXPOSED_FRIENDLY_NAME: usize = 0x10; // CUtlString
                pub const M_STR_EXPOSED_FRIENDLY_GROUP_NAME: usize = 0x18; // CUtlString
                pub const M_B_EXPOSED_VARIABLE_IS_FIXED_RANGE: usize = 0x20; // bool
                pub const M_STR_EXPOSED_VISIBLE_WHEN_TRUE: usize = 0x28; // CUtlString
                pub const M_STR_EXPOSED_HIDDEN_WHEN_TRUE: usize = 0x30; // CUtlString
                pub const M_N_VARIABLE_TYPE: usize = 0x38; // CompositeMaterialInputLooseVariableType_t
                pub const M_B_VALUE_BOOLEAN: usize = 0x3C; // bool
                pub const M_N_VALUE_INT_X: usize = 0x40; // int32
                pub const M_N_VALUE_INT_Y: usize = 0x44; // int32
                pub const M_N_VALUE_INT_Z: usize = 0x48; // int32
                pub const M_N_VALUE_INT_W: usize = 0x4C; // int32
                pub const M_B_HAS_FLOAT_BOUNDS: usize = 0x50; // bool
                pub const M_FL_VALUE_FLOAT_X: usize = 0x54; // float32
                pub const M_FL_VALUE_FLOAT_X_MIN: usize = 0x58; // float32
                pub const M_FL_VALUE_FLOAT_X_MAX: usize = 0x5C; // float32
                pub const M_FL_VALUE_FLOAT_Y: usize = 0x60; // float32
                pub const M_FL_VALUE_FLOAT_Y_MIN: usize = 0x64; // float32
                pub const M_FL_VALUE_FLOAT_Y_MAX: usize = 0x68; // float32
                pub const M_FL_VALUE_FLOAT_Z: usize = 0x6C; // float32
                pub const M_FL_VALUE_FLOAT_Z_MIN: usize = 0x70; // float32
                pub const M_FL_VALUE_FLOAT_Z_MAX: usize = 0x74; // float32
                pub const M_FL_VALUE_FLOAT_W: usize = 0x78; // float32
                pub const M_FL_VALUE_FLOAT_W_MIN: usize = 0x7C; // float32
                pub const M_FL_VALUE_FLOAT_W_MAX: usize = 0x80; // float32
                pub const M_C_VALUE_COLOR4: usize = 0x84; // Color
                pub const M_N_VALUE_SYSTEM_VAR: usize = 0x88; // CompositeMaterialVarSystemVar_t
                pub const M_STR_RESOURCE_MATERIAL: usize = 0x90; // CResourceName
                pub const M_STR_TEXTURE_CONTENT_ASSET_PATH: usize = 0x170; // CUtlString
                pub const M_STR_TEXTURE_RUNTIME_RESOURCE_PATH: usize = 0x178; // CResourceName
                pub const M_STR_TEXTURE_COMPILATION_VTEX_TEMPLATE: usize = 0x258; // CUtlString
                pub const M_N_TEXTURE_TYPE: usize = 0x260; // CompositeMaterialInputTextureType_t
                pub const M_STR_STRING: usize = 0x268; // CUtlString
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyElementNameFn
            pub mod comp_mat_mutator_condition_t {
                pub const M_N_MUTATOR_CONDITION: usize = 0x0; // CompMatPropertyMutatorConditionType_t
                pub const M_STR_MUTATOR_CONDITION_CONTAINER_NAME: usize = 0x8; // CUtlString
                pub const M_STR_MUTATOR_CONDITION_CONTAINER_VAR_NAME: usize = 0x10; // CUtlString
                pub const M_STR_MUTATOR_CONDITION_CONTAINER_VAR_VALUE: usize = 0x18; // CUtlString
                pub const M_B_PASS_WHEN_TRUE: usize = 0x20; // bool
            }
            // Parent: None
            // Fields count: 28
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyElementNameFn
            pub mod comp_mat_property_mutator_t {
                pub const M_B_ENABLED: usize = 0x0; // bool
                pub const M_N_MUTATOR_COMMAND_TYPE: usize = 0x4; // CompMatPropertyMutatorType_t
                pub const M_STR_INIT_WITH_CONTAINER: usize = 0x8; // CUtlString
                pub const M_STR_COPY_PROPERTY_INPUT_CONTAINER_SRC: usize = 0x10; // CUtlString
                pub const M_STR_COPY_PROPERTY_INPUT_CONTAINER_PROPERTY: usize = 0x18; // CUtlString
                pub const M_STR_COPY_PROPERTY_TARGET_PROPERTY: usize = 0x20; // CUtlString
                pub const M_STR_RANDOM_ROLL_INPUT_VARS_SEED_INPUT_VAR: usize = 0x28; // CUtlString
                pub const M_VEC_RANDOM_ROLL_INPUT_VARS_INPUT_VARS_TO_ROLL: usize = 0x30; // CUtlVector< CUtlString >
                pub const M_STR_COPY_MATCHING_KEYS_INPUT_CONTAINER_SRC: usize = 0x48; // CUtlString
                pub const M_STR_COPY_KEYS_WITH_SUFFIX_INPUT_CONTAINER_SRC: usize = 0x50; // CUtlString
                pub const M_STR_COPY_KEYS_WITH_SUFFIX_FIND_SUFFIX: usize = 0x58; // CUtlString
                pub const M_STR_COPY_KEYS_WITH_SUFFIX_REPLACE_SUFFIX: usize = 0x60; // CUtlString
                pub const M_N_SET_VALUE_VALUE: usize = 0x68; // CompositeMaterialInputLooseVariable_t
                pub const M_STR_GENERATE_TEXTURE_TARGET_PARAM: usize = 0x2D8; // CUtlString
                pub const M_STR_GENERATE_TEXTURE_INITIAL_CONTAINER: usize = 0x2E0; // CUtlString
                pub const M_N_RESOLUTION: usize = 0x2E8; // int32
                pub const M_B_IS_SCRATCH_TARGET: usize = 0x2EC; // bool
                pub const M_B_SPLAT_DEBUG_INFO: usize = 0x2ED; // bool
                pub const M_B_CAPTURE_IN_RENDER_DOC: usize = 0x2EE; // bool
                pub const M_VEC_TEX_GEN_INSTRUCTIONS: usize = 0x2F0; // CUtlVector< CompMatPropertyMutator_t >
                pub const M_VEC_CONDITIONAL_MUTATORS: usize = 0x308; // CUtlVector< CompMatPropertyMutator_t >
                pub const M_STR_POP_INPUT_QUEUE_CONTAINER: usize = 0x320; // CUtlString
                pub const M_STR_DRAW_TEXT_INPUT_CONTAINER_SRC: usize = 0x328; // CUtlString
                pub const M_STR_DRAW_TEXT_INPUT_CONTAINER_PROPERTY: usize = 0x330; // CUtlString
                pub const M_VEC_DRAW_TEXT_POSITION: usize = 0x338; // Vector2D
                pub const M_COL_DRAW_TEXT_COLOR: usize = 0x340; // Color
                pub const M_STR_DRAW_TEXT_FONT: usize = 0x348; // CUtlString
                pub const M_VEC_CONDITIONS: usize = 0x350; // CUtlVector< CompMatMutatorCondition_t >
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyElementNameFn
            pub mod composite_material_input_container_t {
                pub const M_B_ENABLED: usize = 0x0; // bool
                pub const M_N_COMPOSITE_MATERIAL_INPUT_CONTAINER_SOURCE_TYPE: usize = 0x4; // CompositeMaterialInputContainerSourceType_t
                pub const M_STR_SPECIFIC_CONTAINER_MATERIAL: usize = 0x8; // CResourceName
                pub const M_STR_ATTR_NAME: usize = 0xE8; // CUtlString
                pub const M_STR_ALIAS: usize = 0xF0; // CUtlString
                pub const M_VEC_LOOSE_VARIABLES: usize = 0xF8; // CUtlVector< CompositeMaterialInputLooseVariable_t >
                pub const M_STR_ATTR_NAME_FOR_VAR: usize = 0x110; // CUtlString
                pub const M_B_EXPOSE_EXTERNALLY: usize = 0x118; // bool
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyElementNameFn
            pub mod composite_material_assembly_procedure_t {
                pub const M_VEC_COMP_MAT_INCLUDES: usize = 0x0; // CUtlVector< CResourceName >
                pub const M_VEC_MATCH_FILTERS: usize = 0x18; // CUtlVector< CompositeMaterialMatchFilter_t >
                pub const M_VEC_COMPOSITE_INPUT_CONTAINERS: usize = 0x30; // CUtlVector< CompositeMaterialInputContainer_t >
                pub const M_VEC_PROPERTY_MUTATORS: usize = 0x48; // CUtlVector< CompMatPropertyMutator_t >
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyElementNameFn
            pub mod generated_texture_handle_t {
                pub const M_STR_BITMAP_NAME: usize = 0x0; // CUtlString
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MPropertyElementNameFn
            pub mod composite_material_t {
                pub const M_TARGET_K_VS: usize = 0x8; // KeyValues3
                pub const M_PRE_GENERATION_K_VS: usize = 0x18; // KeyValues3
                pub const M_FINAL_K_VS: usize = 0x28; // KeyValues3
                pub const M_VEC_GENERATED_TEXTURES: usize = 0x40; // CUtlVector< GeneratedTextureHandle_t >
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod composite_material_editor_point_t {
                pub const M_MODEL_NAME: usize = 0x0; // CResourceName
                pub const M_N_SEQUENCE_INDEX: usize = 0xE0; // int32
                pub const M_FL_CYCLE: usize = 0xE4; // float32
                pub const M_KV_MODEL_STATE_CHOICES: usize = 0xE8; // KeyValues3
                pub const M_B_ENABLE_CHILD_MODEL: usize = 0xF8; // bool
                pub const M_CHILD_MODEL_NAME: usize = 0x100; // CResourceName
                pub const M_VEC_COMPOSITE_MATERIAL_ASSEMBLY_PROCEDURES: usize = 0x1E0; // CUtlVector< CompositeMaterialAssemblyProcedure_t >
                pub const M_VEC_COMPOSITE_MATERIALS: usize = 0x1F8; // CUtlVector< CompositeMaterial_t >
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_composite_material_editor_doc {
                pub const M_N_VERSION: usize = 0x8; // int32
                pub const M_POINTS: usize = 0x10; // CUtlVector< CompositeMaterialEditorPoint_t >
                pub const M_K_VTHUMBNAIL: usize = 0x28; // KeyValues3
            }
            // Parent: None
            // Fields count: 43
            pub mod c_global_light_base {
                pub const M_B_SPOT_LIGHT: usize = 0x10; // bool
                pub const M_SPOT_LIGHT_ORIGIN: usize = 0x14; // 
                pub const M_SPOT_LIGHT_ANGLES: usize = 0x20; // QAngle
                pub const M_SHADOW_DIRECTION: usize = 0x2C; // 
                pub const M_AMBIENT_DIRECTION: usize = 0x38; // 
                pub const M_SPECULAR_DIRECTION: usize = 0x44; // 
                pub const M_INSPECTOR_SPECULAR_DIRECTION: usize = 0x50; // 
                pub const M_FL_SPECULAR_POWER: usize = 0x5C; // float32
                pub const M_FL_SPECULAR_INDEPENDENCE: usize = 0x60; // float32
                pub const M_SPECULAR_COLOR: usize = 0x64; // Color
                pub const M_B_START_DISABLED: usize = 0x68; // bool
                pub const M_B_ENABLED: usize = 0x69; // bool
                pub const M_LIGHT_COLOR: usize = 0x6A; // Color
                pub const M_AMBIENT_COLOR1: usize = 0x6E; // Color
                pub const M_AMBIENT_COLOR2: usize = 0x72; // Color
                pub const M_AMBIENT_COLOR3: usize = 0x76; // Color
                pub const M_FL_SUN_DISTANCE: usize = 0x7C; // float32
                pub const M_FL_FOV: usize = 0x80; // float32
                pub const M_FL_NEAR_Z: usize = 0x84; // float32
                pub const M_FL_FAR_Z: usize = 0x88; // float32
                pub const M_B_ENABLE_SHADOWS: usize = 0x8C; // bool
                pub const M_B_OLD_ENABLE_SHADOWS: usize = 0x8D; // bool
                pub const M_B_BACKGROUND_CLEAR_NOT_REQUIRED: usize = 0x8E; // bool
                pub const M_FL_CLOUD_SCALE: usize = 0x90; // float32
                pub const M_FL_CLOUD1_SPEED: usize = 0x94; // float32
                pub const M_FL_CLOUD1_DIRECTION: usize = 0x98; // float32
                pub const M_FL_CLOUD2_SPEED: usize = 0x9C; // float32
                pub const M_FL_CLOUD2_DIRECTION: usize = 0xA0; // float32
                pub const M_FL_AMBIENT_SCALE1: usize = 0xB0; // float32
                pub const M_FL_AMBIENT_SCALE2: usize = 0xB4; // float32
                pub const M_FL_GROUND_SCALE: usize = 0xB8; // float32
                pub const M_FL_LIGHT_SCALE: usize = 0xBC; // float32
                pub const M_FL_FO_W_DARKNESS: usize = 0xC0; // float32
                pub const M_B_ENABLE_SEPARATE_SKYBOX_FOG: usize = 0xC4; // bool
                pub const M_V_FOW_COLOR: usize = 0xC8; // 
                pub const M_VIEW_ORIGIN: usize = 0xD4; // 
                pub const M_VIEW_ANGLES: usize = 0xE0; // QAngle
                pub const M_FL_VIEW_FO_V: usize = 0xEC; // float32
                pub const M_WORLD_POINTS: usize = 0xF0; // Vector[8]
                pub const M_V_FOG_OFFSET_LAYER0: usize = 0x4A8; // Vector2D
                pub const M_V_FOG_OFFSET_LAYER1: usize = 0x4B0; // Vector2D
                pub const M_H_ENV_WIND: usize = 0x4B8; // CHandle< C_BaseEntity >
                pub const M_H_ENV_SKY: usize = 0x4BC; // CHandle< C_BaseEntity >
            }
            // Parent: c_base_entity
            // Fields count: 1
            pub mod c_global_light {
                pub const M_WIND_CLOTH_FORCE_HANDLE: usize = 0xA10; // uint16
            }
            // Parent: c_anim_graph_controller_base
            // Fields count: 4
            pub mod c_csgo_preview_model_graph_controller {
                pub const M_PSZ_CHARACTER_MODE: usize = 0x18; // CAnimGraphParamOptionalRef< char* >
                pub const M_PSZ_WEAPON_STATE: usize = 0x38; // CAnimGraphParamOptionalRef< char* >
                pub const M_PSZ_WEAPON_TYPE: usize = 0x58; // CAnimGraphParamOptionalRef< char* >
                pub const M_PSZ_END_OF_MATCH_CELEBRATION: usize = 0x78; // CAnimGraphParamOptionalRef< char* >
            }
            // Parent: c_anim_graph_controller_base
            // Fields count: 8
            pub mod c_csgo_preview_player_graph_controller {
                pub const M_PSZ_CHARACTER_MODE: usize = 0x18; // CAnimGraphParamOptionalRef< char* >
                pub const M_PSZ_TEAM_PREVIEW_VARIANT: usize = 0x38; // CAnimGraphParamOptionalRef< char* >
                pub const M_PSZ_TEAM_PREVIEW_POSITION: usize = 0x58; // CAnimGraphParamOptionalRef< char* >
                pub const M_PSZ_END_OF_MATCH_CELEBRATION: usize = 0x78; // CAnimGraphParamOptionalRef< char* >
                pub const M_N_TEAM_PREVIEW_RANDOM: usize = 0x98; // CAnimGraphParamOptionalRef< int32 >
                pub const M_PSZ_WEAPON_STATE: usize = 0xB0; // CAnimGraphParamOptionalRef< char* >
                pub const M_PSZ_WEAPON_TYPE: usize = 0xD0; // CAnimGraphParamOptionalRef< char* >
                pub const M_B_CT: usize = 0xF0; // CAnimGraphParamOptionalRef< bool >
            }
            // Parent: c_base_entity
            // Fields count: 10
            pub mod c_csgo_map_preview_camera_path_node {
                pub const M_SZ_PARENT_PATH_UNIQUE_ID: usize = 0x550; // CUtlSymbolLarge
                pub const M_N_PATH_INDEX: usize = 0x558; // int32
                pub const M_V_IN_TANGENT_LOCAL: usize = 0x55C; // 
                pub const M_V_OUT_TANGENT_LOCAL: usize = 0x568; // 
                pub const M_FL_FOV: usize = 0x574; // float32
                pub const M_FL_SPEED: usize = 0x578; // float32
                pub const M_FL_EASE_IN: usize = 0x57C; // float32
                pub const M_FL_EASE_OUT: usize = 0x580; // float32
                pub const M_V_IN_TANGENT_WORLD: usize = 0x584; // 
                pub const M_V_OUT_TANGENT_WORLD: usize = 0x590; // 
            }
            // Parent: c_base_entity
            // Fields count: 8
            pub mod c_csgo_map_preview_camera_path {
                pub const M_FL_Z_FAR: usize = 0x550; // float32
                pub const M_FL_Z_NEAR: usize = 0x554; // float32
                pub const M_B_LOOP: usize = 0x558; // bool
                pub const M_B_VERTICAL_FOV: usize = 0x559; // bool
                pub const M_B_CONSTANT_SPEED: usize = 0x55A; // bool
                pub const M_FL_DURATION: usize = 0x55C; // float32
                pub const M_FL_PATH_LENGTH: usize = 0x5A0; // float32
                pub const M_FL_PATH_DURATION: usize = 0x5A4; // float32
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod ccs_player_glow_services {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod c_cs_observer_pawn_api {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod c_cs_player_pawn_api {
            }
            // Parent: c_base_entity
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_iActiveIssueIndex (int)
            // NetworkVarNames: m_iOnlyTeamToVote (int)
            // NetworkVarNames: m_nVoteOptionCount (int)
            // NetworkVarNames: m_nPotentialVotes (int)
            // NetworkVarNames: m_bIsYesNoVote (bool)
            pub mod c_vote_controller {
                pub const M_I_ACTIVE_ISSUE_INDEX: usize = 0x560; // int32
                pub const M_I_ONLY_TEAM_TO_VOTE: usize = 0x564; // int32
                pub const M_N_VOTE_OPTION_COUNT: usize = 0x568; // int32[5]
                pub const M_N_POTENTIAL_VOTES: usize = 0x57C; // int32
                pub const M_B_VOTES_DIRTY: usize = 0x580; // bool
                pub const M_B_TYPE_DIRTY: usize = 0x581; // bool
                pub const M_B_IS_YES_NO_VOTE: usize = 0x582; // bool
            }
            // Parent: c_base_entity
            // Fields count: 17
            //
            // Metadata:
            // NetworkVarNames: m_nDraftType (int)
            // NetworkVarNames: m_nTeamWinningCoinToss (int)
            // NetworkVarNames: m_nTeamWithFirstChoice (int)
            // NetworkVarNames: m_nVoteMapIdsList (int)
            // NetworkVarNames: m_nAccountIDs (int)
            // NetworkVarNames: m_nMapId0 (int)
            // NetworkVarNames: m_nMapId1 (int)
            // NetworkVarNames: m_nMapId2 (int)
            // NetworkVarNames: m_nMapId3 (int)
            // NetworkVarNames: m_nMapId4 (int)
            // NetworkVarNames: m_nMapId5 (int)
            // NetworkVarNames: m_nStartingSide0 (int)
            // NetworkVarNames: m_nCurrentPhase (int)
            // NetworkVarNames: m_nPhaseStartTick (int)
            // NetworkVarNames: m_nPhaseDurationTicks (int)
            pub mod c_map_veto_pick_controller {
                pub const M_N_DRAFT_TYPE: usize = 0x560; // int32
                pub const M_N_TEAM_WINNING_COIN_TOSS: usize = 0x564; // int32
                pub const M_N_TEAM_WITH_FIRST_CHOICE: usize = 0x568; // int32[64]
                pub const M_N_VOTE_MAP_IDS_LIST: usize = 0x668; // int32[7]
                pub const M_N_ACCOUNT_I_DS: usize = 0x684; // int32[64]
                pub const M_N_MAP_ID0: usize = 0x784; // int32[64]
                pub const M_N_MAP_ID1: usize = 0x884; // int32[64]
                pub const M_N_MAP_ID2: usize = 0x984; // int32[64]
                pub const M_N_MAP_ID3: usize = 0xA84; // int32[64]
                pub const M_N_MAP_ID4: usize = 0xB84; // int32[64]
                pub const M_N_MAP_ID5: usize = 0xC84; // int32[64]
                pub const M_N_STARTING_SIDE0: usize = 0xD84; // int32[64]
                pub const M_N_CURRENT_PHASE: usize = 0xE84; // int32
                pub const M_N_PHASE_START_TICK: usize = 0xE88; // int32
                pub const M_N_PHASE_DURATION_TICKS: usize = 0xE8C; // int32
                pub const M_N_POST_DATA_UPDATE_TICK: usize = 0xE90; // int32
                pub const M_B_DISABLED_HUD: usize = 0xE94; // bool
            }
            // Parent: None
            // Fields count: 0
            pub mod c_player_spray_decal_render_helper {
            }
            // Parent: c_csgo_map_preview_camera_path
            // Fields count: 7
            pub mod c_csgo_team_preview_camera {
                pub const M_N_VARIANT: usize = 0x5B0; // int32
                pub const M_B_DOF_ENABLED: usize = 0x5B4; // bool
                pub const M_FL_DOF_NEAR_BLURRY: usize = 0x5B8; // float32
                pub const M_FL_DOF_NEAR_CRISP: usize = 0x5BC; // float32
                pub const M_FL_DOF_FAR_CRISP: usize = 0x5C0; // float32
                pub const M_FL_DOF_FAR_BLURRY: usize = 0x5C4; // float32
                pub const M_FL_DOF_TILT_TO_GROUND: usize = 0x5C8; // float32
            }
            // Parent: c_csgo_team_preview_camera
            // Fields count: 0
            pub mod c_csgo_team_select_camera {
            }
            // Parent: c_csgo_team_preview_camera
            // Fields count: 0
            pub mod c_csgo_terrorist_team_intro_camera {
            }
            // Parent: c_csgo_team_preview_camera
            // Fields count: 0
            pub mod c_csgo_terrorist_wingman_intro_camera {
            }
            // Parent: c_csgo_team_preview_camera
            // Fields count: 0
            pub mod c_csgo_counter_terrorist_team_intro_camera {
            }
            // Parent: c_csgo_team_preview_camera
            // Fields count: 0
            pub mod c_csgo_counter_terrorist_wingman_intro_camera {
            }
            // Parent: c_csgo_team_preview_camera
            // Fields count: 0
            pub mod c_csgo_end_of_match_camera {
            }
            // Parent: c_csgo_team_preview_character_position
            // Fields count: 0
            pub mod c_csgo_end_of_match_character_position {
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_csgo_end_of_match_lineup_endpoint {
            }
            // Parent: c_csgo_end_of_match_lineup_endpoint
            // Fields count: 0
            pub mod c_csgo_end_of_match_lineup_start {
            }
            // Parent: c_csgo_end_of_match_lineup_endpoint
            // Fields count: 0
            pub mod c_csgo_end_of_match_lineup_end {
            }
            // Parent: c_base_entity
            // Fields count: 2
            pub mod c_csm_fov_override {
                pub const M_CAMERA_NAME: usize = 0x550; // CUtlString
                pub const M_FL_CSM_FOV_OVERRIDE_VALUE: usize = 0x558; // float32
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_point_entity {
            }
            // Parent: c_base_entity
            // Fields count: 24
            //
            // Metadata:
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_flBrightness (float)
            // NetworkVarNames: m_hCubemapTexture (HRenderTextureStrong)
            // NetworkVarNames: m_bCustomCubemapTexture (bool)
            // NetworkVarNames: m_hLightProbeTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightIndicesTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightScalarsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightShadowsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_vBoxMins (Vector)
            // NetworkVarNames: m_vBoxMaxs (Vector)
            // NetworkVarNames: m_bMoveable (bool)
            // NetworkVarNames: m_nHandshake (int)
            // NetworkVarNames: m_nEnvCubeMapArrayIndex (int)
            // NetworkVarNames: m_nPriority (int)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_flEdgeFadeDist (float)
            // NetworkVarNames: m_vEdgeFadeDists (Vector)
            // NetworkVarNames: m_nLightProbeSizeX (int)
            // NetworkVarNames: m_nLightProbeSizeY (int)
            // NetworkVarNames: m_nLightProbeSizeZ (int)
            // NetworkVarNames: m_nLightProbeAtlasX (int)
            // NetworkVarNames: m_nLightProbeAtlasY (int)
            // NetworkVarNames: m_nLightProbeAtlasZ (int)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod c_env_combined_light_probe_volume {
                pub const M_COLOR: usize = 0x15A8; // Color
                pub const M_FL_BRIGHTNESS: usize = 0x15AC; // float32
                pub const M_H_CUBEMAP_TEXTURE: usize = 0x15B0; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_B_CUSTOM_CUBEMAP_TEXTURE: usize = 0x15B8; // bool
                pub const M_H_LIGHT_PROBE_TEXTURE: usize = 0x15C0; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_H_LIGHT_PROBE_DIRECT_LIGHT_INDICES_TEXTURE: usize = 0x15C8; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_H_LIGHT_PROBE_DIRECT_LIGHT_SCALARS_TEXTURE: usize = 0x15D0; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_H_LIGHT_PROBE_DIRECT_LIGHT_SHADOWS_TEXTURE: usize = 0x15D8; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_V_BOX_MINS: usize = 0x15E0; // 
                pub const M_V_BOX_MAXS: usize = 0x15EC; // 
                pub const M_B_MOVEABLE: usize = 0x15F8; // bool
                pub const M_N_HANDSHAKE: usize = 0x15FC; // int32
                pub const M_N_ENV_CUBE_MAP_ARRAY_INDEX: usize = 0x1600; // int32
                pub const M_N_PRIORITY: usize = 0x1604; // int32
                pub const M_B_START_DISABLED: usize = 0x1608; // bool
                pub const M_FL_EDGE_FADE_DIST: usize = 0x160C; // float32
                pub const M_V_EDGE_FADE_DISTS: usize = 0x1610; // 
                pub const M_N_LIGHT_PROBE_SIZE_X: usize = 0x161C; // int32
                pub const M_N_LIGHT_PROBE_SIZE_Y: usize = 0x1620; // int32
                pub const M_N_LIGHT_PROBE_SIZE_Z: usize = 0x1624; // int32
                pub const M_N_LIGHT_PROBE_ATLAS_X: usize = 0x1628; // int32
                pub const M_N_LIGHT_PROBE_ATLAS_Y: usize = 0x162C; // int32
                pub const M_N_LIGHT_PROBE_ATLAS_Z: usize = 0x1630; // int32
                pub const M_B_ENABLED: usize = 0x1649; // bool
            }
            // Parent: c_base_entity
            // Fields count: 18
            //
            // Metadata:
            // NetworkVarNames: m_hCubemapTexture (HRenderTextureStrong)
            // NetworkVarNames: m_bCustomCubemapTexture (bool)
            // NetworkVarNames: m_flInfluenceRadius (float)
            // NetworkVarNames: m_vBoxProjectMins (Vector)
            // NetworkVarNames: m_vBoxProjectMaxs (Vector)
            // NetworkVarNames: m_bMoveable (bool)
            // NetworkVarNames: m_nHandshake (int)
            // NetworkVarNames: m_nEnvCubeMapArrayIndex (int)
            // NetworkVarNames: m_nPriority (int)
            // NetworkVarNames: m_flEdgeFadeDist (float)
            // NetworkVarNames: m_vEdgeFadeDists (Vector)
            // NetworkVarNames: m_flDiffuseScale (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bDefaultEnvMap (bool)
            // NetworkVarNames: m_bDefaultSpecEnvMap (bool)
            // NetworkVarNames: m_bIndoorCubeMap (bool)
            // NetworkVarNames: m_bCopyDiffuseFromDefaultCubemap (bool)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod c_env_cubemap {
                pub const M_H_CUBEMAP_TEXTURE: usize = 0x5D0; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_B_CUSTOM_CUBEMAP_TEXTURE: usize = 0x5D8; // bool
                pub const M_FL_INFLUENCE_RADIUS: usize = 0x5DC; // float32
                pub const M_V_BOX_PROJECT_MINS: usize = 0x5E0; // 
                pub const M_V_BOX_PROJECT_MAXS: usize = 0x5EC; // 
                pub const M_B_MOVEABLE: usize = 0x5F8; // bool
                pub const M_N_HANDSHAKE: usize = 0x5FC; // int32
                pub const M_N_ENV_CUBE_MAP_ARRAY_INDEX: usize = 0x600; // int32
                pub const M_N_PRIORITY: usize = 0x604; // int32
                pub const M_FL_EDGE_FADE_DIST: usize = 0x608; // float32
                pub const M_V_EDGE_FADE_DISTS: usize = 0x60C; // 
                pub const M_FL_DIFFUSE_SCALE: usize = 0x618; // float32
                pub const M_B_START_DISABLED: usize = 0x61C; // bool
                pub const M_B_DEFAULT_ENV_MAP: usize = 0x61D; // bool
                pub const M_B_DEFAULT_SPEC_ENV_MAP: usize = 0x61E; // bool
                pub const M_B_INDOOR_CUBE_MAP: usize = 0x61F; // bool
                pub const M_B_COPY_DIFFUSE_FROM_DEFAULT_CUBEMAP: usize = 0x620; // bool
                pub const M_B_ENABLED: usize = 0x630; // bool
            }
            // Parent: c_env_cubemap
            // Fields count: 0
            pub mod c_env_cubemap_box {
            }
            // Parent: c_base_entity
            // Fields count: 18
            //
            // Metadata:
            // NetworkVarNames: m_flEndDistance (float)
            // NetworkVarNames: m_flStartDistance (float)
            // NetworkVarNames: m_flFogFalloffExponent (float)
            // NetworkVarNames: m_bHeightFogEnabled (bool)
            // NetworkVarNames: m_flFogHeightWidth (float)
            // NetworkVarNames: m_flFogHeightEnd (float)
            // NetworkVarNames: m_flFogHeightStart (float)
            // NetworkVarNames: m_flFogHeightExponent (float)
            // NetworkVarNames: m_flLODBias (float)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_flFogMaxOpacity (float)
            // NetworkVarNames: m_nCubemapSourceType (int)
            // NetworkVarNames: m_hSkyMaterial (HMaterialStrong)
            // NetworkVarNames: m_iszSkyEntity (string_t)
            // NetworkVarNames: m_hFogCubemapTexture (HRenderTextureStrong)
            // NetworkVarNames: m_bHasHeightFogEnd (bool)
            pub mod c_env_cubemap_fog {
                pub const M_FL_END_DISTANCE: usize = 0x550; // float32
                pub const M_FL_START_DISTANCE: usize = 0x554; // float32
                pub const M_FL_FOG_FALLOFF_EXPONENT: usize = 0x558; // float32
                pub const M_B_HEIGHT_FOG_ENABLED: usize = 0x55C; // bool
                pub const M_FL_FOG_HEIGHT_WIDTH: usize = 0x560; // float32
                pub const M_FL_FOG_HEIGHT_END: usize = 0x564; // float32
                pub const M_FL_FOG_HEIGHT_START: usize = 0x568; // float32
                pub const M_FL_FOG_HEIGHT_EXPONENT: usize = 0x56C; // float32
                pub const M_FL_LOD_BIAS: usize = 0x570; // float32
                pub const M_B_ACTIVE: usize = 0x574; // bool
                pub const M_B_START_DISABLED: usize = 0x575; // bool
                pub const M_FL_FOG_MAX_OPACITY: usize = 0x578; // float32
                pub const M_N_CUBEMAP_SOURCE_TYPE: usize = 0x57C; // int32
                pub const M_H_SKY_MATERIAL: usize = 0x580; // CStrongHandle< InfoForResourceTypeIMaterial2 >
                pub const M_ISZ_SKY_ENTITY: usize = 0x588; // CUtlSymbolLarge
                pub const M_H_FOG_CUBEMAP_TEXTURE: usize = 0x590; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_B_HAS_HEIGHT_FOG_END: usize = 0x598; // bool
                pub const M_B_FIRST_TIME: usize = 0x599; // bool
            }
            // Parent: c_base_entity
            // Fields count: 16
            //
            // Metadata:
            // NetworkVarNames: m_hGradientFogTexture (HRenderTextureStrong)
            // NetworkVarNames: m_flFogStartDistance (float)
            // NetworkVarNames: m_flFogEndDistance (float)
            // NetworkVarNames: m_bHeightFogEnabled (bool)
            // NetworkVarNames: m_flFogStartHeight (float)
            // NetworkVarNames: m_flFogEndHeight (float)
            // NetworkVarNames: m_flFarZ (float)
            // NetworkVarNames: m_flFogMaxOpacity (float)
            // NetworkVarNames: m_flFogFalloffExponent (float)
            // NetworkVarNames: m_flFogVerticalExponent (float)
            // NetworkVarNames: m_fogColor (Color)
            // NetworkVarNames: m_flFogStrength (float)
            // NetworkVarNames: m_flFadeTime (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bIsEnabled (bool)
            pub mod c_gradient_fog {
                pub const M_H_GRADIENT_FOG_TEXTURE: usize = 0x550; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_FL_FOG_START_DISTANCE: usize = 0x558; // float32
                pub const M_FL_FOG_END_DISTANCE: usize = 0x55C; // float32
                pub const M_B_HEIGHT_FOG_ENABLED: usize = 0x560; // bool
                pub const M_FL_FOG_START_HEIGHT: usize = 0x564; // float32
                pub const M_FL_FOG_END_HEIGHT: usize = 0x568; // float32
                pub const M_FL_FAR_Z: usize = 0x56C; // float32
                pub const M_FL_FOG_MAX_OPACITY: usize = 0x570; // float32
                pub const M_FL_FOG_FALLOFF_EXPONENT: usize = 0x574; // float32
                pub const M_FL_FOG_VERTICAL_EXPONENT: usize = 0x578; // float32
                pub const M_FOG_COLOR: usize = 0x57C; // Color
                pub const M_FL_FOG_STRENGTH: usize = 0x580; // float32
                pub const M_FL_FADE_TIME: usize = 0x584; // float32
                pub const M_B_START_DISABLED: usize = 0x588; // bool
                pub const M_B_IS_ENABLED: usize = 0x589; // bool
                pub const M_B_GRADIENT_FOG_NEEDS_TEXTURES: usize = 0x58A; // bool
            }
            // Parent: c_base_entity
            // Fields count: 17
            //
            // Metadata:
            // NetworkVarNames: m_hLightProbeTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightIndicesTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightScalarsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightShadowsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_vBoxMins (Vector)
            // NetworkVarNames: m_vBoxMaxs (Vector)
            // NetworkVarNames: m_bMoveable (bool)
            // NetworkVarNames: m_nHandshake (int)
            // NetworkVarNames: m_nPriority (int)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_nLightProbeSizeX (int)
            // NetworkVarNames: m_nLightProbeSizeY (int)
            // NetworkVarNames: m_nLightProbeSizeZ (int)
            // NetworkVarNames: m_nLightProbeAtlasX (int)
            // NetworkVarNames: m_nLightProbeAtlasY (int)
            // NetworkVarNames: m_nLightProbeAtlasZ (int)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod c_env_light_probe_volume {
                pub const M_H_LIGHT_PROBE_TEXTURE: usize = 0x1528; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_H_LIGHT_PROBE_DIRECT_LIGHT_INDICES_TEXTURE: usize = 0x1530; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_H_LIGHT_PROBE_DIRECT_LIGHT_SCALARS_TEXTURE: usize = 0x1538; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_H_LIGHT_PROBE_DIRECT_LIGHT_SHADOWS_TEXTURE: usize = 0x1540; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_V_BOX_MINS: usize = 0x1548; // 
                pub const M_V_BOX_MAXS: usize = 0x1554; // 
                pub const M_B_MOVEABLE: usize = 0x1560; // bool
                pub const M_N_HANDSHAKE: usize = 0x1564; // int32
                pub const M_N_PRIORITY: usize = 0x1568; // int32
                pub const M_B_START_DISABLED: usize = 0x156C; // bool
                pub const M_N_LIGHT_PROBE_SIZE_X: usize = 0x1570; // int32
                pub const M_N_LIGHT_PROBE_SIZE_Y: usize = 0x1574; // int32
                pub const M_N_LIGHT_PROBE_SIZE_Z: usize = 0x1578; // int32
                pub const M_N_LIGHT_PROBE_ATLAS_X: usize = 0x157C; // int32
                pub const M_N_LIGHT_PROBE_ATLAS_Y: usize = 0x1580; // int32
                pub const M_N_LIGHT_PROBE_ATLAS_Z: usize = 0x1584; // int32
                pub const M_B_ENABLED: usize = 0x1591; // bool
            }
            // Parent: c_base_entity
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: m_flVisibilityStrength (float)
            // NetworkVarNames: m_flFogDistanceMultiplier (float)
            // NetworkVarNames: m_flFogMaxDensityMultiplier (float)
            // NetworkVarNames: m_flFadeTime (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bIsEnabled (bool)
            pub mod c_player_visibility {
                pub const M_FL_VISIBILITY_STRENGTH: usize = 0x550; // float32
                pub const M_FL_FOG_DISTANCE_MULTIPLIER: usize = 0x554; // float32
                pub const M_FL_FOG_MAX_DENSITY_MULTIPLIER: usize = 0x558; // float32
                pub const M_FL_FADE_TIME: usize = 0x55C; // float32
                pub const M_B_START_DISABLED: usize = 0x560; // bool
                pub const M_B_IS_ENABLED: usize = 0x561; // bool
            }
            // Parent: c_base_entity
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_flAutoExposureMin (float)
            // NetworkVarNames: m_flAutoExposureMax (float)
            // NetworkVarNames: m_flTonemapPercentTarget (float)
            // NetworkVarNames: m_flTonemapPercentBrightPixels (float)
            // NetworkVarNames: m_flTonemapMinAvgLum (float)
            // NetworkVarNames: m_flExposureAdaptationSpeedUp (float)
            // NetworkVarNames: m_flExposureAdaptationSpeedDown (float)
            // NetworkVarNames: m_flTonemapEVSmoothingRange (float)
            pub mod c_tonemap_controller2 {
                pub const M_FL_AUTO_EXPOSURE_MIN: usize = 0x550; // float32
                pub const M_FL_AUTO_EXPOSURE_MAX: usize = 0x554; // float32
                pub const M_FL_TONEMAP_PERCENT_TARGET: usize = 0x558; // float32
                pub const M_FL_TONEMAP_PERCENT_BRIGHT_PIXELS: usize = 0x55C; // float32
                pub const M_FL_TONEMAP_MIN_AVG_LUM: usize = 0x560; // float32
                pub const M_FL_EXPOSURE_ADAPTATION_SPEED_UP: usize = 0x564; // float32
                pub const M_FL_EXPOSURE_ADAPTATION_SPEED_DOWN: usize = 0x568; // float32
                pub const M_FL_TONEMAP_EV_SMOOTHING_RANGE: usize = 0x56C; // float32
            }
            // Parent: c_base_entity
            // Fields count: 28
            //
            // Metadata:
            // NetworkVarNames: m_flScattering (float)
            // NetworkVarNames: m_flAnisotropy (float)
            // NetworkVarNames: m_flFadeSpeed (float)
            // NetworkVarNames: m_flDrawDistance (float)
            // NetworkVarNames: m_flFadeInStart (float)
            // NetworkVarNames: m_flFadeInEnd (float)
            // NetworkVarNames: m_flIndirectStrength (float)
            // NetworkVarNames: m_nIndirectTextureDimX (int)
            // NetworkVarNames: m_nIndirectTextureDimY (int)
            // NetworkVarNames: m_nIndirectTextureDimZ (int)
            // NetworkVarNames: m_vBoxMins (Vector)
            // NetworkVarNames: m_vBoxMaxs (Vector)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_flStartAnisoTime (GameTime_t)
            // NetworkVarNames: m_flStartScatterTime (GameTime_t)
            // NetworkVarNames: m_flStartDrawDistanceTime (GameTime_t)
            // NetworkVarNames: m_flStartAnisotropy (float)
            // NetworkVarNames: m_flStartScattering (float)
            // NetworkVarNames: m_flStartDrawDistance (float)
            // NetworkVarNames: m_flDefaultAnisotropy (float)
            // NetworkVarNames: m_flDefaultScattering (float)
            // NetworkVarNames: m_flDefaultDrawDistance (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bEnableIndirect (bool)
            // NetworkVarNames: m_bIsMaster (bool)
            // NetworkVarNames: m_hFogIndirectTexture (HRenderTextureStrong)
            // NetworkVarNames: m_nForceRefreshCount (int)
            pub mod c_env_volumetric_fog_controller {
                pub const M_FL_SCATTERING: usize = 0x550; // float32
                pub const M_FL_ANISOTROPY: usize = 0x554; // float32
                pub const M_FL_FADE_SPEED: usize = 0x558; // float32
                pub const M_FL_DRAW_DISTANCE: usize = 0x55C; // float32
                pub const M_FL_FADE_IN_START: usize = 0x560; // float32
                pub const M_FL_FADE_IN_END: usize = 0x564; // float32
                pub const M_FL_INDIRECT_STRENGTH: usize = 0x568; // float32
                pub const M_N_INDIRECT_TEXTURE_DIM_X: usize = 0x56C; // int32
                pub const M_N_INDIRECT_TEXTURE_DIM_Y: usize = 0x570; // int32
                pub const M_N_INDIRECT_TEXTURE_DIM_Z: usize = 0x574; // int32
                pub const M_V_BOX_MINS: usize = 0x578; // 
                pub const M_V_BOX_MAXS: usize = 0x584; // 
                pub const M_B_ACTIVE: usize = 0x590; // bool
                pub const M_FL_START_ANISO_TIME: usize = 0x594; // GameTime_t
                pub const M_FL_START_SCATTER_TIME: usize = 0x598; // GameTime_t
                pub const M_FL_START_DRAW_DISTANCE_TIME: usize = 0x59C; // GameTime_t
                pub const M_FL_START_ANISOTROPY: usize = 0x5A0; // float32
                pub const M_FL_START_SCATTERING: usize = 0x5A4; // float32
                pub const M_FL_START_DRAW_DISTANCE: usize = 0x5A8; // float32
                pub const M_FL_DEFAULT_ANISOTROPY: usize = 0x5AC; // float32
                pub const M_FL_DEFAULT_SCATTERING: usize = 0x5B0; // float32
                pub const M_FL_DEFAULT_DRAW_DISTANCE: usize = 0x5B4; // float32
                pub const M_B_START_DISABLED: usize = 0x5B8; // bool
                pub const M_B_ENABLE_INDIRECT: usize = 0x5B9; // bool
                pub const M_B_IS_MASTER: usize = 0x5BA; // bool
                pub const M_H_FOG_INDIRECT_TEXTURE: usize = 0x5C0; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_N_FORCE_REFRESH_COUNT: usize = 0x5C8; // int32
                pub const M_B_FIRST_TIME: usize = 0x5CC; // bool
            }
            // Parent: c_base_entity
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_vBoxMins (Vector)
            // NetworkVarNames: m_vBoxMaxs (Vector)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_flStrength (float)
            // NetworkVarNames: m_nFalloffShape (int)
            // NetworkVarNames: m_flFalloffExponent (float)
            pub mod c_env_volumetric_fog_volume {
                pub const M_B_ACTIVE: usize = 0x550; // bool
                pub const M_V_BOX_MINS: usize = 0x554; // 
                pub const M_V_BOX_MAXS: usize = 0x560; // 
                pub const M_B_START_DISABLED: usize = 0x56C; // bool
                pub const M_FL_STRENGTH: usize = 0x570; // float32
                pub const M_N_FALLOFF_SHAPE: usize = 0x574; // int32
                pub const M_FL_FALLOFF_EXPONENT: usize = 0x578; // float32
            }
            // Parent: c_base_entity
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_fog (fogparams_t)
            pub mod c_fog_controller {
                pub const M_FOG: usize = 0x550; // fogparams_t
                pub const M_B_USE_ANGLES: usize = 0x5B8; // bool
                pub const M_I_CHANGED_VARIABLES: usize = 0x5BC; // int32
            }
            // Parent: c_point_entity
            // Fields count: 0
            pub mod c_info_target {
            }
            // Parent: c_point_entity
            // Fields count: 0
            pub mod c_info_particle_target {
            }
            // Parent: c_base_entity
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_nMode (int)
            // NetworkVarNames: m_vBoxSize (Vector)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod c_info_visibility_box {
                pub const M_N_MODE: usize = 0x554; // int32
                pub const M_V_BOX_SIZE: usize = 0x558; // 
                pub const M_B_ENABLED: usize = 0x564; // bool
            }
            // Parent: c_base_entity
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_worldName (string_t)
            // NetworkVarNames: m_layerName (string_t)
            // NetworkVarNames: m_bWorldLayerVisible (bool)
            // NetworkVarNames: m_bEntitiesSpawned (bool)
            pub mod c_info_world_layer {
                pub const M_P_OUTPUT_ON_ENTITIES_SPAWNED: usize = 0x550; // CEntityIOOutput
                pub const M_WORLD_NAME: usize = 0x578; // CUtlSymbolLarge
                pub const M_LAYER_NAME: usize = 0x580; // CUtlSymbolLarge
                pub const M_B_WORLD_LAYER_VISIBLE: usize = 0x588; // bool
                pub const M_B_ENTITIES_SPAWNED: usize = 0x589; // bool
                pub const M_B_CREATE_AS_CHILD_SPAWN_GROUP: usize = 0x58A; // bool
                pub const M_H_LAYER_SPAWN_GROUP: usize = 0x58C; // uint32
                pub const M_B_WORLD_LAYER_ACTUALLY_VISIBLE: usize = 0x590; // bool
            }
            // Parent: c_base_entity
            // Fields count: 25
            //
            // Metadata:
            // NetworkVarNames: m_FOV (float)
            // NetworkVarNames: m_Resolution (float)
            // NetworkVarNames: m_bFogEnable (bool)
            // NetworkVarNames: m_FogColor (Color)
            // NetworkVarNames: m_flFogStart (float)
            // NetworkVarNames: m_flFogEnd (float)
            // NetworkVarNames: m_flFogMaxDensity (float)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_bUseScreenAspectRatio (bool)
            // NetworkVarNames: m_flAspectRatio (float)
            // NetworkVarNames: m_bNoSky (bool)
            // NetworkVarNames: m_fBrightness (float)
            // NetworkVarNames: m_flZFar (float)
            // NetworkVarNames: m_flZNear (float)
            // NetworkVarNames: m_bCanHLTVUse (bool)
            // NetworkVarNames: m_bDofEnabled (bool)
            // NetworkVarNames: m_flDofNearBlurry (float)
            // NetworkVarNames: m_flDofNearCrisp (float)
            // NetworkVarNames: m_flDofFarCrisp (float)
            // NetworkVarNames: m_flDofFarBlurry (float)
            // NetworkVarNames: m_flDofTiltToGround (float)
            pub mod c_point_camera {
                pub const M_FOV: usize = 0x550; // float32
                pub const M_RESOLUTION: usize = 0x554; // float32
                pub const M_B_FOG_ENABLE: usize = 0x558; // bool
                pub const M_FOG_COLOR: usize = 0x559; // Color
                pub const M_FL_FOG_START: usize = 0x560; // float32
                pub const M_FL_FOG_END: usize = 0x564; // float32
                pub const M_FL_FOG_MAX_DENSITY: usize = 0x568; // float32
                pub const M_B_ACTIVE: usize = 0x56C; // bool
                pub const M_B_USE_SCREEN_ASPECT_RATIO: usize = 0x56D; // bool
                pub const M_FL_ASPECT_RATIO: usize = 0x570; // float32
                pub const M_B_NO_SKY: usize = 0x574; // bool
                pub const M_F_BRIGHTNESS: usize = 0x578; // float32
                pub const M_FL_Z_FAR: usize = 0x57C; // float32
                pub const M_FL_Z_NEAR: usize = 0x580; // float32
                pub const M_B_CAN_HLTV_USE: usize = 0x584; // bool
                pub const M_B_DOF_ENABLED: usize = 0x585; // bool
                pub const M_FL_DOF_NEAR_BLURRY: usize = 0x588; // float32
                pub const M_FL_DOF_NEAR_CRISP: usize = 0x58C; // float32
                pub const M_FL_DOF_FAR_CRISP: usize = 0x590; // float32
                pub const M_FL_DOF_FAR_BLURRY: usize = 0x594; // float32
                pub const M_FL_DOF_TILT_TO_GROUND: usize = 0x598; // float32
                pub const M_TARGET_FOV: usize = 0x59C; // float32
                pub const M_DEGREES_PER_SECOND: usize = 0x5A0; // float32
                pub const M_B_IS_ON: usize = 0x5A4; // bool
                pub const M_P_NEXT: usize = 0x5A8; // C_PointCamera*
            }
            // Parent: c_point_camera
            // Fields count: 1
            pub mod c_point_camera_vfov {
                pub const M_FL_VERTICAL_FOV: usize = 0x5B0; // float32
            }
            // Parent: c_logical_entity
            // Fields count: 12
            pub mod c_point_template {
                pub const M_ISZ_WORLD_NAME: usize = 0x550; // CUtlSymbolLarge
                pub const M_ISZ_SOURCE2_ENTITY_LUMP_NAME: usize = 0x558; // CUtlSymbolLarge
                pub const M_ISZ_ENTITY_FILTER_NAME: usize = 0x560; // CUtlSymbolLarge
                pub const M_FL_TIMEOUT_INTERVAL: usize = 0x568; // float32
                pub const M_B_ASYNCHRONOUSLY_SPAWN_ENTITIES: usize = 0x56C; // bool
                pub const M_P_OUTPUT_ON_SPAWNED: usize = 0x570; // CEntityIOOutput
                pub const M_CLIENT_ONLY_ENTITY_BEHAVIOR: usize = 0x598; // PointTemplateClientOnlyEntityBehavior_t
                pub const M_OWNER_SPAWN_GROUP_TYPE: usize = 0x59C; // PointTemplateOwnerSpawnGroupType_t
                pub const M_CREATED_SPAWN_GROUP_HANDLES: usize = 0x5A0; // CUtlVector< uint32 >
                pub const M_SPAWNED_ENTITY_HANDLES: usize = 0x5B8; // CUtlVector< CEntityHandle >
                pub const M_SCRIPT_SPAWN_CALLBACK: usize = 0x5D0; // HSCRIPT
                pub const M_SCRIPT_CALLBACK_SCOPE: usize = 0x5D8; // HSCRIPT
            }
            // Parent: c_base_entity
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_iszSoundAreaType (string_t)
            // NetworkVarNames: m_vPos (Vector)
            pub mod c_sound_area_entity_base {
                pub const M_B_DISABLED: usize = 0x550; // bool
                pub const M_B_WAS_ENABLED: usize = 0x558; // bool
                pub const M_ISZ_SOUND_AREA_TYPE: usize = 0x560; // CUtlSymbolLarge
                pub const M_V_POS: usize = 0x568; // 
            }
            // Parent: c_sound_area_entity_base
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_flRadius (float)
            pub mod c_sound_area_entity_sphere {
                pub const M_FL_RADIUS: usize = 0x578; // float32
            }
            // Parent: c_sound_area_entity_base
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_vMin (Vector)
            // NetworkVarNames: m_vMax (Vector)
            pub mod c_sound_area_entity_oriented_box {
                pub const M_V_MIN: usize = 0x578; // 
                pub const M_V_MAX: usize = 0x584; // 
            }
            // Parent: c_base_entity
            // Fields count: 4
            //
            // Metadata:
            // MNetworkIncludeByName
            // NetworkVarNames: m_aPlayerControllers (CHandle< CBasePlayerController >)
            // NetworkVarNames: m_aPlayers (CHandle< C_BasePlayerPawn >)
            // NetworkVarNames: m_iScore (int32)
            // NetworkVarNames: m_szTeamname (char)
            pub mod c_team {
                pub const M_A_PLAYER_CONTROLLERS: usize = 0x550; // C_NetworkUtlVectorBase< CHandle< CBasePlayerController > >
                pub const M_A_PLAYERS: usize = 0x568; // C_NetworkUtlVectorBase< CHandle< C_BasePlayerPawn > >
                pub const M_I_SCORE: usize = 0x580; // int32
                pub const M_SZ_TEAMNAME: usize = 0x584; // char[129]
            }
            // Parent: c_base_entity
            // Fields count: 15
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkUserGroupProxy
            // MNetworkUserGroupProxy
            // MNetworkIncludeByUserGroup
            // NetworkVarNames: m_nTickBase (uint32)
            // NetworkVarNames: m_hPawn (CHandle< CBasePlayerPawn>)
            // NetworkVarNames: m_iConnected (PlayerConnectedState)
            // NetworkVarNames: m_iszPlayerName (char)
            // NetworkVarNames: m_steamID (uint64)
            // NetworkVarNames: m_iDesiredFOV (uint32)
            // MNetworkReplayCompatField
            pub mod c_base_player_controller {
                pub const M_N_FINAL_PREDICTED_TICK: usize = 0x558; // int32
                pub const M_COMMAND_CONTEXT: usize = 0x560; // C_CommandContext
                pub const M_N_IN_BUTTONS_WHICH_ARE_TOGGLES: usize = 0x5F8; // uint64
                pub const M_N_TICK_BASE: usize = 0x600; // uint32
                pub const M_H_PAWN: usize = 0x604; // CHandle< C_BasePlayerPawn >
                pub const M_H_PREDICTED_PAWN: usize = 0x608; // CHandle< C_BasePlayerPawn >
                pub const M_N_SPLIT_SCREEN_SLOT: usize = 0x60C; // CSplitScreenSlot
                pub const M_H_SPLIT_OWNER: usize = 0x610; // CHandle< CBasePlayerController >
                pub const M_H_SPLIT_SCREEN_PLAYERS: usize = 0x618; // CUtlVector< CHandle< CBasePlayerController > >
                pub const M_B_IS_HLTV: usize = 0x630; // bool
                pub const M_I_CONNECTED: usize = 0x634; // PlayerConnectedState
                pub const M_ISZ_PLAYER_NAME: usize = 0x638; // char[128]
                pub const M_STEAM_ID: usize = 0x6C0; // uint64
                pub const M_B_IS_LOCAL_PLAYER_CONTROLLER: usize = 0x6C8; // bool
                pub const M_I_DESIRED_FOV: usize = 0x6CC; // uint32
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_base_player_v_data {
                pub const M_S_MODEL_NAME: usize = 0x28; // CResourceNameTyped< CWeakHandle< InfoForResourceTypeCModel > >
                pub const M_FL_HEAD_DAMAGE_MULTIPLIER: usize = 0x108; // CSkillFloat
                pub const M_FL_CHEST_DAMAGE_MULTIPLIER: usize = 0x118; // CSkillFloat
                pub const M_FL_STOMACH_DAMAGE_MULTIPLIER: usize = 0x128; // CSkillFloat
                pub const M_FL_ARM_DAMAGE_MULTIPLIER: usize = 0x138; // CSkillFloat
                pub const M_FL_LEG_DAMAGE_MULTIPLIER: usize = 0x148; // CSkillFloat
                pub const M_FL_HOLD_BREATH_TIME: usize = 0x158; // float32
                pub const M_FL_DROWNING_DAMAGE_INTERVAL: usize = 0x15C; // float32
                pub const M_N_DROWNING_DAMAGE_INITIAL: usize = 0x160; // int32
                pub const M_N_DROWNING_DAMAGE_MAX: usize = 0x164; // int32
                pub const M_N_WATER_SPEED: usize = 0x168; // int32
                pub const M_FL_USE_RANGE: usize = 0x16C; // float32
                pub const M_FL_USE_ANGLE_TOLERANCE: usize = 0x170; // float32
                pub const M_FL_CROUCH_TIME: usize = 0x174; // float32
            }
            // Parent: None
            // Fields count: 20
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_base_player_weapon_v_data {
                pub const M_SZ_WORLD_MODEL: usize = 0x28; // CResourceNameTyped< CWeakHandle< InfoForResourceTypeCModel > >
                pub const M_B_BUILT_RIGHT_HANDED: usize = 0x108; // bool
                pub const M_B_ALLOW_FLIPPING: usize = 0x109; // bool
                pub const M_S_MUZZLE_ATTACHMENT: usize = 0x110; // CUtlString
                pub const M_SZ_MUZZLE_FLASH_PARTICLE: usize = 0x118; // CResourceNameTyped< CWeakHandle< InfoForResourceTypeIParticleSystemDefinition > >
                pub const M_I_FLAGS: usize = 0x1F8; // ItemFlagTypes_t
                pub const M_N_PRIMARY_AMMO_TYPE: usize = 0x1F9; // AmmoIndex_t
                pub const M_N_SECONDARY_AMMO_TYPE: usize = 0x1FA; // AmmoIndex_t
                pub const M_I_MAX_CLIP1: usize = 0x1FC; // int32
                pub const M_I_MAX_CLIP2: usize = 0x200; // int32
                pub const M_I_DEFAULT_CLIP1: usize = 0x204; // int32
                pub const M_I_DEFAULT_CLIP2: usize = 0x208; // int32
                pub const M_I_WEIGHT: usize = 0x20C; // int32
                pub const M_B_AUTO_SWITCH_TO: usize = 0x210; // bool
                pub const M_B_AUTO_SWITCH_FROM: usize = 0x211; // bool
                pub const M_I_RUMBLE_EFFECT: usize = 0x214; // RumbleEffect_t
                pub const M_B_LINKED_COOLDOWNS: usize = 0x218; // bool
                pub const M_A_SHOOT_SOUNDS: usize = 0x220; // CUtlMap< WeaponSound_t, CSoundEventName >
                pub const M_I_SLOT: usize = 0x240; // int32
                pub const M_I_POSITION: usize = 0x244; // int32
            }
            // Parent: c_skeleton_animation_controller
            // Fields count: 13
            //
            // Metadata:
            // NetworkVarNames: m_animGraphNetworkedVars (CAnimGraphNetworkedVariables)
            // NetworkVarNames: m_hSequence (HSequence)
            // NetworkVarNames: m_flSeqStartTime (GameTime_t)
            // NetworkVarNames: m_flSeqFixedCycle (float)
            // NetworkVarNames: m_nAnimLoopMode (AnimLoopMode_t)
            pub mod c_base_anim_graph_controller {
                pub const M_ANIM_GRAPH_NETWORKED_VARS: usize = 0x18; // CAnimGraphNetworkedVariables
                pub const M_B_SEQUENCE_FINISHED: usize = 0x14A8; // bool
                pub const M_FL_SOUND_SYNC_TIME: usize = 0x14AC; // float32
                pub const M_H_SEQUENCE: usize = 0x14B0; // HSequence
                pub const M_FL_SEQ_START_TIME: usize = 0x14B4; // GameTime_t
                pub const M_FL_SEQ_FIXED_CYCLE: usize = 0x14B8; // float32
                pub const M_N_ANIM_LOOP_MODE: usize = 0x14BC; // AnimLoopMode_t
                pub const M_FL_PLAYBACK_RATE: usize = 0x14C0; // CNetworkedQuantizedFloat
                pub const M_N_NOTIFY_STATE: usize = 0x14CC; // SequenceFinishNotifyState_t
                pub const M_B_NETWORKED_ANIMATION_INPUTS_CHANGED: usize = 0x14CE; // bool
                pub const M_B_NETWORKED_SEQUENCE_CHANGED: usize = 0x14CF; // bool
                pub const M_B_LAST_UPDATE_SKIPPED: usize = 0x14D0; // bool
                pub const M_FL_PREV_ANIM_UPDATE_TIME: usize = 0x14D4; // GameTime_t
            }
            // Parent: c_base_entity
            // Fields count: 31
            //
            // Metadata:
            // NetworkVarNames: m_CRenderComponent (CRenderComponent::Storage_t)
            // NetworkVarNames: m_CHitboxComponent (CHitboxComponent::Storage_t)
            // NetworkVarNames: m_nRenderMode (RenderMode_t)
            // NetworkVarNames: m_nRenderFX (RenderFx_t)
            // NetworkVarNames: m_clrRender (Color)
            // NetworkVarNames: m_vecRenderAttributes (EntityRenderAttribute_t)
            // NetworkVarNames: m_bRenderToCubemaps (bool)
            // NetworkVarNames: m_Collision (CCollisionProperty)
            // NetworkVarNames: m_Glow (CGlowProperty)
            // NetworkVarNames: m_flGlowBackfaceMult (float)
            // NetworkVarNames: m_fadeMinDist (float32)
            // NetworkVarNames: m_fadeMaxDist (float32)
            // NetworkVarNames: m_flFadeScale (float32)
            // NetworkVarNames: m_flShadowStrength (float32)
            // NetworkVarNames: m_nObjectCulling (uint8)
            // NetworkVarNames: m_nAddDecal (int)
            // NetworkVarNames: m_vDecalPosition (Vector)
            // NetworkVarNames: m_vDecalForwardAxis (Vector)
            // NetworkVarNames: m_flDecalHealBloodRate (float)
            // NetworkVarNames: m_flDecalHealHeightRate (float)
            // NetworkVarNames: m_ConfigEntitiesToPropagateMaterialDecalsTo (CHandle< C_BaseModelEntity >)
            pub mod c_base_model_entity {
                pub const M_C_RENDER_COMPONENT: usize = 0xA20; // CRenderComponent*
                pub const M_C_HITBOX_COMPONENT: usize = 0xA28; // CHitboxComponent
                pub const M_B_INIT_MODEL_EFFECTS: usize = 0xA70; // bool
                pub const M_B_IS_STATIC_PROP: usize = 0xA71; // bool
                pub const M_N_LAST_ADD_DECAL: usize = 0xA74; // int32
                pub const M_N_DECALS_ADDED: usize = 0xA78; // int32
                pub const M_I_OLD_HEALTH: usize = 0xA7C; // int32
                pub const M_N_RENDER_MODE: usize = 0xA80; // RenderMode_t
                pub const M_N_RENDER_FX: usize = 0xA81; // RenderFx_t
                pub const M_B_ALLOW_FADE_IN_VIEW: usize = 0xA82; // bool
                pub const M_CLR_RENDER: usize = 0xA83; // Color
                pub const M_VEC_RENDER_ATTRIBUTES: usize = 0xA88; // C_UtlVectorEmbeddedNetworkVar< EntityRenderAttribute_t >
                pub const M_B_RENDER_TO_CUBEMAPS: usize = 0xAF0; // bool
                pub const M_COLLISION: usize = 0xAF8; // CCollisionProperty
                pub const M_GLOW: usize = 0xBA8; // CGlowProperty
                pub const M_FL_GLOW_BACKFACE_MULT: usize = 0xC00; // float32
                pub const M_FADE_MIN_DIST: usize = 0xC04; // float32
                pub const M_FADE_MAX_DIST: usize = 0xC08; // float32
                pub const M_FL_FADE_SCALE: usize = 0xC0C; // float32
                pub const M_FL_SHADOW_STRENGTH: usize = 0xC10; // float32
                pub const M_N_OBJECT_CULLING: usize = 0xC14; // uint8
                pub const M_N_ADD_DECAL: usize = 0xC18; // int32
                pub const M_V_DECAL_POSITION: usize = 0xC1C; // 
                pub const M_V_DECAL_FORWARD_AXIS: usize = 0xC28; // 
                pub const M_FL_DECAL_HEAL_BLOOD_RATE: usize = 0xC34; // float32
                pub const M_FL_DECAL_HEAL_HEIGHT_RATE: usize = 0xC38; // float32
                pub const M_CONFIG_ENTITIES_TO_PROPAGATE_MATERIAL_DECALS_TO: usize = 0xC40; // C_NetworkUtlVectorBase< CHandle< C_BaseModelEntity > >
                pub const M_VEC_VIEW_OFFSET: usize = 0xC58; // CNetworkViewOffsetVector
                pub const M_P_CLIENT_ALPHA_PROPERTY: usize = 0xC88; // CClientAlphaProperty*
                pub const M_CLIENT_OVERRIDE_TINT: usize = 0xC90; // Color
                pub const M_B_USE_CLIENT_OVERRIDE_TINT: usize = 0xC94; // bool
            }
            // Parent: c_base_model_entity
            // Fields count: 0
            pub mod c_server_only_model_entity {
            }
            // Parent: c_base_model_entity
            // Fields count: 0
            pub mod c_model_point_entity {
            }
            // Parent: c_logical_entity
            // Fields count: 7
            pub mod c_logic_relay {
                pub const M_ON_TRIGGER: usize = 0x550; // CEntityIOOutput
                pub const M_ON_SPAWN: usize = 0x578; // CEntityIOOutput
                pub const M_B_DISABLED: usize = 0x5A0; // bool
                pub const M_B_WAIT_FOR_REFIRE: usize = 0x5A1; // bool
                pub const M_B_TRIGGER_ONCE: usize = 0x5A2; // bool
                pub const M_B_FAST_RETRIGGER: usize = 0x5A3; // bool
                pub const M_B_PASSTHOUGH_CALLER: usize = 0x5A4; // bool
            }
            // Parent: c_base_model_entity
            // Fields count: 24
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_szSnapshotFileName (char)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_bFrozen (bool)
            // NetworkVarNames: m_flFreezeTransitionDuration (float)
            // NetworkVarNames: m_nStopType (int)
            // NetworkVarNames: m_bAnimateDuringGameplayPause (bool)
            // NetworkVarNames: m_iEffectIndex (HParticleSystemDefinitionStrong)
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_flPreSimTime (float32)
            // NetworkVarNames: m_vServerControlPoints (Vector)
            // NetworkVarNames: m_iServerControlPointAssignments (uint8)
            // NetworkVarNames: m_hControlPointEnts (CHandle< CBaseEntity >)
            // NetworkVarNames: m_bNoSave (bool)
            // NetworkVarNames: m_bNoFreeze (bool)
            // NetworkVarNames: m_bNoRamp (bool)
            pub mod c_particle_system {
                pub const M_SZ_SNAPSHOT_FILE_NAME: usize = 0xCD0; // char[512]
                pub const M_B_ACTIVE: usize = 0xED0; // bool
                pub const M_B_FROZEN: usize = 0xED1; // bool
                pub const M_FL_FREEZE_TRANSITION_DURATION: usize = 0xED4; // float32
                pub const M_N_STOP_TYPE: usize = 0xED8; // int32
                pub const M_B_ANIMATE_DURING_GAMEPLAY_PAUSE: usize = 0xEDC; // bool
                pub const M_I_EFFECT_INDEX: usize = 0xEE0; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
                pub const M_FL_START_TIME: usize = 0xEE8; // GameTime_t
                pub const M_FL_PRE_SIM_TIME: usize = 0xEEC; // float32
                pub const M_V_SERVER_CONTROL_POINTS: usize = 0xEF0; // Vector[4]
                pub const M_I_SERVER_CONTROL_POINT_ASSIGNMENTS: usize = 0xF20; // uint8[4]
                pub const M_H_CONTROL_POINT_ENTS: usize = 0xF24; // CHandle< C_BaseEntity >[64]
                pub const M_B_NO_SAVE: usize = 0x1024; // bool
                pub const M_B_NO_FREEZE: usize = 0x1025; // bool
                pub const M_B_NO_RAMP: usize = 0x1026; // bool
                pub const M_B_START_ACTIVE: usize = 0x1027; // bool
                pub const M_ISZ_EFFECT_NAME: usize = 0x1028; // CUtlSymbolLarge
                pub const M_ISZ_CONTROL_POINT_NAMES: usize = 0x1030; // CUtlSymbolLarge[64]
                pub const M_N_DATA_CP: usize = 0x1230; // int32
                pub const M_VEC_DATA_CP_VALUE: usize = 0x1234; // 
                pub const M_N_TINT_CP: usize = 0x1240; // int32
                pub const M_CLR_TINT: usize = 0x1244; // Color
                pub const M_B_OLD_ACTIVE: usize = 0x1268; // bool
                pub const M_B_OLD_FROZEN: usize = 0x1269; // bool
            }
            // Parent: c_base_entity
            // Fields count: 16
            //
            // Metadata:
            // NetworkVarNames: m_flParticleSpacing (float)
            // NetworkVarNames: m_flSlack (float)
            // NetworkVarNames: m_flRadius (float)
            // NetworkVarNames: m_ColorTint (Color)
            // NetworkVarNames: m_nEffectState (int)
            // NetworkVarNames: m_iEffectIndex (HParticleSystemDefinitionStrong)
            // NetworkVarNames: m_PathNodes_Position (Vector)
            // NetworkVarNames: m_PathNodes_TangentIn (Vector)
            // NetworkVarNames: m_PathNodes_TangentOut (Vector)
            // NetworkVarNames: m_PathNodes_Color (Vector)
            // NetworkVarNames: m_PathNodes_PinEnabled (bool)
            // NetworkVarNames: m_PathNodes_RadiusScale (float)
            pub mod c_path_particle_rope {
                pub const M_B_START_ACTIVE: usize = 0x550; // bool
                pub const M_FL_MAX_SIMULATION_TIME: usize = 0x554; // float32
                pub const M_ISZ_EFFECT_NAME: usize = 0x558; // CUtlSymbolLarge
                pub const M_PATH_NODES_NAME: usize = 0x560; // CUtlVector< CUtlSymbolLarge >
                pub const M_FL_PARTICLE_SPACING: usize = 0x578; // float32
                pub const M_FL_SLACK: usize = 0x57C; // float32
                pub const M_FL_RADIUS: usize = 0x580; // float32
                pub const M_COLOR_TINT: usize = 0x584; // Color
                pub const M_N_EFFECT_STATE: usize = 0x588; // int32
                pub const M_I_EFFECT_INDEX: usize = 0x590; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
                pub const M_PATH_NODES_POSITION: usize = 0x598; // C_NetworkUtlVectorBase< Vector >
                pub const M_PATH_NODES_TANGENT_IN: usize = 0x5B0; // C_NetworkUtlVectorBase< Vector >
                pub const M_PATH_NODES_TANGENT_OUT: usize = 0x5C8; // C_NetworkUtlVectorBase< Vector >
                pub const M_PATH_NODES_COLOR: usize = 0x5E0; // C_NetworkUtlVectorBase< Vector >
                pub const M_PATH_NODES_PIN_ENABLED: usize = 0x5F8; // C_NetworkUtlVectorBase< bool >
                pub const M_PATH_NODES_RADIUS_SCALE: usize = 0x610; // C_NetworkUtlVectorBase< float32 >
            }
            // Parent: c_path_particle_rope
            // Fields count: 0
            pub mod c_path_particle_rope_alias_path_particle_rope_clientside {
            }
            // Parent: c_base_model_entity
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_Flags (uint8)
            // NetworkVarNames: m_LightStyle (uint8)
            // NetworkVarNames: m_Radius (float32)
            // NetworkVarNames: m_Exponent (int32)
            // NetworkVarNames: m_InnerAngle (float32)
            // NetworkVarNames: m_OuterAngle (float32)
            // NetworkVarNames: m_SpotRadius (float32)
            pub mod c_dynamic_light {
                pub const M_FLAGS: usize = 0xCD0; // uint8
                pub const M_LIGHT_STYLE: usize = 0xCD1; // uint8
                pub const M_RADIUS: usize = 0xCD4; // float32
                pub const M_EXPONENT: usize = 0xCD8; // int32
                pub const M_INNER_ANGLE: usize = 0xCDC; // float32
                pub const M_OUTER_ANGLE: usize = 0xCE0; // float32
                pub const M_SPOT_RADIUS: usize = 0xCE4; // float32
            }
            // Parent: c_point_entity
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_iszOverlayNames (string_t)
            // NetworkVarNames: m_flOverlayTimes (float32)
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_iDesiredOverlay (int32)
            // NetworkVarNames: m_bIsActive (bool)
            pub mod c_env_screen_overlay {
                pub const M_ISZ_OVERLAY_NAMES: usize = 0x550; // CUtlSymbolLarge[10]
                pub const M_FL_OVERLAY_TIMES: usize = 0x5A0; // float32[10]
                pub const M_FL_START_TIME: usize = 0x5C8; // GameTime_t
                pub const M_I_DESIRED_OVERLAY: usize = 0x5CC; // int32
                pub const M_B_IS_ACTIVE: usize = 0x5D0; // bool
                pub const M_B_WAS_ACTIVE: usize = 0x5D1; // bool
                pub const M_I_CACHED_DESIRED_OVERLAY: usize = 0x5D4; // int32
                pub const M_I_CURRENT_OVERLAY: usize = 0x5D8; // int32
                pub const M_FL_CURRENT_OVERLAY_TIME: usize = 0x5DC; // GameTime_t
            }
            // Parent: c_base_model_entity
            // Fields count: 3
            pub mod c_func_track_train {
                pub const M_N_LONG_AXIS: usize = 0xCD0; // int32
                pub const M_FL_RADIUS: usize = 0xCD4; // float32
                pub const M_FL_LINE_LENGTH: usize = 0xCD8; // float32
            }
            // Parent: c_glow_overlay
            // Fields count: 7
            pub mod c_light_glow_overlay {
                pub const M_VEC_ORIGIN: usize = 0xD0; // 
                pub const M_VEC_DIRECTION: usize = 0xDC; // 
                pub const M_N_MIN_DIST: usize = 0xE8; // int32
                pub const M_N_MAX_DIST: usize = 0xEC; // int32
                pub const M_N_OUTER_MAX_DIST: usize = 0xF0; // int32
                pub const M_B_ONE_SIDED: usize = 0xF4; // bool
                pub const M_B_MODULATE_BY_DOT: usize = 0xF5; // bool
            }
            // Parent: c_base_model_entity
            // Fields count: 8
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_nHorizontalSize (uint32)
            // NetworkVarNames: m_nVerticalSize (uint32)
            // NetworkVarNames: m_nMinDist (uint32)
            // NetworkVarNames: m_nMaxDist (uint32)
            // NetworkVarNames: m_nOuterMaxDist (uint32)
            // NetworkVarNames: m_flGlowProxySize (float32)
            // NetworkVarNames: m_flHDRColorScale (float32)
            pub mod c_light_glow {
                pub const M_N_HORIZONTAL_SIZE: usize = 0xCD0; // uint32
                pub const M_N_VERTICAL_SIZE: usize = 0xCD4; // uint32
                pub const M_N_MIN_DIST: usize = 0xCD8; // uint32
                pub const M_N_MAX_DIST: usize = 0xCDC; // uint32
                pub const M_N_OUTER_MAX_DIST: usize = 0xCE0; // uint32
                pub const M_FL_GLOW_PROXY_SIZE: usize = 0xCE4; // float32
                pub const M_FL_HDR_COLOR_SCALE: usize = 0xCE8; // float32
                pub const M_GLOW: usize = 0xCF0; // C_LightGlowOverlay
            }
            // Parent: c_base_entity
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_iCurrentMaxRagdollCount (int8)
            pub mod c_ragdoll_manager {
                pub const M_I_CURRENT_MAX_RAGDOLL_COUNT: usize = 0x550; // int8
            }
            // Parent: c_base_model_entity
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_flLightScale (float32)
            // NetworkVarNames: m_Radius (float32)
            pub mod c_spotlight_end {
                pub const M_FL_LIGHT_SCALE: usize = 0xCD0; // float32
                pub const M_RADIUS: usize = 0xCD4; // float32
            }
            // Parent: c_base_entity
            // Fields count: 25
            //
            // Metadata:
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_bUpdateOnClient (bool)
            // NetworkVarNames: m_nInputType (ValueRemapperInputType_t)
            // NetworkVarNames: m_hRemapLineStart (CHandle< CBaseEntity>)
            // NetworkVarNames: m_hRemapLineEnd (CHandle< CBaseEntity>)
            // NetworkVarNames: m_flMaximumChangePerSecond (float)
            // NetworkVarNames: m_flDisengageDistance (float)
            // NetworkVarNames: m_flEngageDistance (float)
            // NetworkVarNames: m_bRequiresUseKey (bool)
            // NetworkVarNames: m_nOutputType (ValueRemapperOutputType_t)
            // NetworkVarNames: m_hOutputEntities (CHandle< C_BaseEntity >)
            // NetworkVarNames: m_nHapticsType (ValueRemapperHapticsType_t)
            // NetworkVarNames: m_nMomentumType (ValueRemapperMomentumType_t)
            // NetworkVarNames: m_flMomentumModifier (float)
            // NetworkVarNames: m_flSnapValue (float)
            // NetworkVarNames: m_nRatchetType (ValueRemapperRatchetType_t)
            // NetworkVarNames: m_flInputOffset (float)
            pub mod c_point_value_remapper {
                pub const M_B_DISABLED: usize = 0x550; // bool
                pub const M_B_DISABLED_OLD: usize = 0x551; // bool
                pub const M_B_UPDATE_ON_CLIENT: usize = 0x552; // bool
                pub const M_N_INPUT_TYPE: usize = 0x554; // ValueRemapperInputType_t
                pub const M_H_REMAP_LINE_START: usize = 0x558; // CHandle< C_BaseEntity >
                pub const M_H_REMAP_LINE_END: usize = 0x55C; // CHandle< C_BaseEntity >
                pub const M_FL_MAXIMUM_CHANGE_PER_SECOND: usize = 0x560; // float32
                pub const M_FL_DISENGAGE_DISTANCE: usize = 0x564; // float32
                pub const M_FL_ENGAGE_DISTANCE: usize = 0x568; // float32
                pub const M_B_REQUIRES_USE_KEY: usize = 0x56C; // bool
                pub const M_N_OUTPUT_TYPE: usize = 0x570; // ValueRemapperOutputType_t
                pub const M_H_OUTPUT_ENTITIES: usize = 0x578; // C_NetworkUtlVectorBase< CHandle< C_BaseEntity > >
                pub const M_N_HAPTICS_TYPE: usize = 0x590; // ValueRemapperHapticsType_t
                pub const M_N_MOMENTUM_TYPE: usize = 0x594; // ValueRemapperMomentumType_t
                pub const M_FL_MOMENTUM_MODIFIER: usize = 0x598; // float32
                pub const M_FL_SNAP_VALUE: usize = 0x59C; // float32
                pub const M_FL_CURRENT_MOMENTUM: usize = 0x5A0; // float32
                pub const M_N_RATCHET_TYPE: usize = 0x5A4; // ValueRemapperRatchetType_t
                pub const M_FL_RATCHET_OFFSET: usize = 0x5A8; // float32
                pub const M_FL_INPUT_OFFSET: usize = 0x5AC; // float32
                pub const M_B_ENGAGED: usize = 0x5B0; // bool
                pub const M_B_FIRST_UPDATE: usize = 0x5B1; // bool
                pub const M_FL_PREVIOUS_VALUE: usize = 0x5B4; // float32
                pub const M_FL_PREVIOUS_UPDATE_TICK_TIME: usize = 0x5B8; // GameTime_t
                pub const M_VEC_PREVIOUS_TEST_POINT: usize = 0x5BC; // 
            }
            // Parent: c_model_point_entity
            // Fields count: 12
            //
            // Metadata:
            // NetworkVarNames: m_messageText (char)
            // NetworkVarNames: m_FontName (char)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bFullbright (bool)
            // NetworkVarNames: m_flWorldUnitsPerPx (float)
            // NetworkVarNames: m_flFontSize (float)
            // NetworkVarNames: m_flDepthOffset (float)
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_nJustifyHorizontal (PointWorldTextJustifyHorizontal_t)
            // NetworkVarNames: m_nJustifyVertical (PointWorldTextJustifyVertical_t)
            // NetworkVarNames: m_nReorientMode (PointWorldTextReorientMode_t)
            pub mod c_point_world_text {
                pub const M_B_FORCE_RECREATE_NEXT_UPDATE: usize = 0xCD8; // bool
                pub const M_MESSAGE_TEXT: usize = 0xCE8; // char[512]
                pub const M_FONT_NAME: usize = 0xEE8; // char[64]
                pub const M_B_ENABLED: usize = 0xF28; // bool
                pub const M_B_FULLBRIGHT: usize = 0xF29; // bool
                pub const M_FL_WORLD_UNITS_PER_PX: usize = 0xF2C; // float32
                pub const M_FL_FONT_SIZE: usize = 0xF30; // float32
                pub const M_FL_DEPTH_OFFSET: usize = 0xF34; // float32
                pub const M_COLOR: usize = 0xF38; // Color
                pub const M_N_JUSTIFY_HORIZONTAL: usize = 0xF3C; // PointWorldTextJustifyHorizontal_t
                pub const M_N_JUSTIFY_VERTICAL: usize = 0xF40; // PointWorldTextJustifyVertical_t
                pub const M_N_REORIENT_MODE: usize = 0xF44; // PointWorldTextReorientMode_t
            }
            // Parent: c_base_entity
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_Handle (CHandle< CBaseEntity>)
            // NetworkVarNames: m_bSendHandle (bool)
            pub mod c_handle_test {
                pub const M_HANDLE: usize = 0x550; // CHandle< C_BaseEntity >
                pub const M_B_SEND_HANDLE: usize = 0x554; // bool
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_client_script_entity {
            }
            // Parent: c_base_entity
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_EnvWindShared (CEnvWindShared)
            pub mod c_env_wind {
                pub const M_ENV_WIND_SHARED: usize = 0x550; // C_EnvWindShared
            }
            // Parent: c_base_model_entity
            // Fields count: 0
            pub mod c_base_toggle {
            }
            // Parent: c_base_toggle
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_glowEntity (CHandle< C_BaseModelEntity>)
            // NetworkVarNames: m_usable (bool)
            // NetworkVarNames: m_szDisplayText (string_t)
            pub mod c_base_button {
                pub const M_GLOW_ENTITY: usize = 0xCD0; // CHandle< C_BaseModelEntity >
                pub const M_USABLE: usize = 0xCD4; // bool
                pub const M_SZ_DISPLAY_TEXT: usize = 0xCD8; // CUtlSymbolLarge
            }
            // Parent: c_base_model_entity
            // Fields count: 0
            pub mod c_precipitation_blocker {
            }
            // Parent: c_base_model_entity
            // Fields count: 13
            //
            // Metadata:
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_flFadeInStart (float32)
            // NetworkVarNames: m_flFadeInLength (float32)
            // NetworkVarNames: m_flFadeOutModelStart (float32)
            // NetworkVarNames: m_flFadeOutModelLength (float32)
            // NetworkVarNames: m_flFadeOutStart (float32)
            // NetworkVarNames: m_flFadeOutLength (float32)
            // NetworkVarNames: m_nDissolveType (EntityDisolveType_t)
            // NetworkVarNames: m_vDissolverOrigin (Vector)
            // NetworkVarNames: m_nMagnitude (uint32)
            pub mod c_entity_dissolve {
                pub const M_FL_START_TIME: usize = 0xCD8; // GameTime_t
                pub const M_FL_FADE_IN_START: usize = 0xCDC; // float32
                pub const M_FL_FADE_IN_LENGTH: usize = 0xCE0; // float32
                pub const M_FL_FADE_OUT_MODEL_START: usize = 0xCE4; // float32
                pub const M_FL_FADE_OUT_MODEL_LENGTH: usize = 0xCE8; // float32
                pub const M_FL_FADE_OUT_START: usize = 0xCEC; // float32
                pub const M_FL_FADE_OUT_LENGTH: usize = 0xCF0; // float32
                pub const M_FL_NEXT_SPARK_TIME: usize = 0xCF4; // GameTime_t
                pub const M_N_DISSOLVE_TYPE: usize = 0xCF8; // EntityDisolveType_t
                pub const M_V_DISSOLVER_ORIGIN: usize = 0xCFC; // 
                pub const M_N_MAGNITUDE: usize = 0xD08; // uint32
                pub const M_B_CORE_EXPLODE: usize = 0xD0C; // bool
                pub const M_B_LINKED_TO_SERVER_ENT: usize = 0xD0D; // bool
            }
            // Parent: c_model_point_entity
            // Fields count: 0
            pub mod c_env_projected_texture {
            }
            // Parent: c_base_model_entity
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_hDecalMaterial (HMaterialStrong)
            // NetworkVarNames: m_flWidth (float)
            // NetworkVarNames: m_flHeight (float)
            // NetworkVarNames: m_flDepth (float)
            // NetworkVarNames: m_nRenderOrder (uint32)
            // NetworkVarNames: m_bProjectOnWorld (bool)
            // NetworkVarNames: m_bProjectOnCharacters (bool)
            // NetworkVarNames: m_bProjectOnWater (bool)
            // NetworkVarNames: m_flDepthSortBias (float)
            pub mod c_env_decal {
                pub const M_H_DECAL_MATERIAL: usize = 0xCD0; // CStrongHandle< InfoForResourceTypeIMaterial2 >
                pub const M_FL_WIDTH: usize = 0xCD8; // float32
                pub const M_FL_HEIGHT: usize = 0xCDC; // float32
                pub const M_FL_DEPTH: usize = 0xCE0; // float32
                pub const M_N_RENDER_ORDER: usize = 0xCE4; // uint32
                pub const M_B_PROJECT_ON_WORLD: usize = 0xCE8; // bool
                pub const M_B_PROJECT_ON_CHARACTERS: usize = 0xCE9; // bool
                pub const M_B_PROJECT_ON_WATER: usize = 0xCEA; // bool
                pub const M_FL_DEPTH_SORT_BIAS: usize = 0xCEC; // float32
            }
            // Parent: c_glow_overlay
            // Fields count: 4
            pub mod c_fire_overlay {
                pub const M_P_OWNER: usize = 0xD0; // C_FireSmoke*
                pub const M_V_BASE_COLORS: usize = 0xD8; // Vector[4]
                pub const M_FL_SCALE: usize = 0x108; // float32
                pub const M_N_GUID: usize = 0x10C; // int32
            }
            // Parent: c_base_model_entity
            // Fields count: 0
            pub mod c_func_brush {
            }
            // Parent: c_func_brush
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_EffectName (string_t)
            // NetworkVarNames: m_bState (bool)
            pub mod c_func_electrified_volume {
                pub const M_N_AMBIENT_EFFECT: usize = 0xCD0; // ParticleIndex_t
                pub const M_EFFECT_NAME: usize = 0xCD8; // CUtlSymbolLarge
                pub const M_B_STATE: usize = 0xCE0; // bool
            }
            // Parent: c_base_model_entity
            // Fields count: 0
            //
            // Metadata:
            // MNetworkOverride
            // MNetworkOverride
            pub mod c_func_rotating {
            }
            // Parent: c_base_model_entity
            // Fields count: 0
            pub mod c_breakable {
            }
            // Parent: c_breakable
            // Fields count: 0
            pub mod c_phys_box {
            }
            // Parent: c_base_model_entity
            // Fields count: 41
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_flScrollSpeed (float32)
            // NetworkVarNames: m_RopeFlags (uint16)
            // NetworkVarNames: m_iRopeMaterialModelIndex (HMaterialStrong)
            // NetworkVarNames: m_nSegments (uint8)
            // NetworkVarNames: m_hStartPoint (CHandle< C_BaseEntity>)
            // NetworkVarNames: m_hEndPoint (CHandle< C_BaseEntity>)
            // NetworkVarNames: m_iStartAttachment (AttachmentHandle_t)
            // NetworkVarNames: m_iEndAttachment (AttachmentHandle_t)
            // NetworkVarNames: m_Subdiv (uint8)
            // NetworkVarNames: m_RopeLength (int16)
            // NetworkVarNames: m_Slack (int16)
            // NetworkVarNames: m_TextureScale (float32)
            // NetworkVarNames: m_fLockedPoints (uint8)
            // NetworkVarNames: m_nChangeCount (uint8)
            // NetworkVarNames: m_Width (float32)
            // NetworkVarNames: m_bConstrainBetweenEndpoints (bool)
            pub mod c_rope_keyframe {
                pub const M_LINKS_TOUCHING_SOMETHING: usize = 0xCD8; // CBitVec< 10 >
                pub const M_N_LINKS_TOUCHING_SOMETHING: usize = 0xCDC; // int32
                pub const M_B_APPLY_WIND: usize = 0xCE0; // bool
                pub const M_F_PREV_LOCKED_POINTS: usize = 0xCE4; // int32
                pub const M_I_FORCE_POINT_MOVE_COUNTER: usize = 0xCE8; // int32
                pub const M_B_PREV_END_POINT_POS: usize = 0xCEC; // bool[2]
                pub const M_V_PREV_END_POINT_POS: usize = 0xCF0; // Vector[2]
                pub const M_FL_CUR_SCROLL: usize = 0xD08; // float32
                pub const M_FL_SCROLL_SPEED: usize = 0xD0C; // float32
                pub const M_ROPE_FLAGS: usize = 0xD10; // uint16
                pub const M_I_ROPE_MATERIAL_MODEL_INDEX: usize = 0xD18; // CStrongHandle< InfoForResourceTypeIMaterial2 >
                pub const M_LIGHT_VALUES: usize = 0xF90; // Vector[10]
                pub const M_N_SEGMENTS: usize = 0x1008; // uint8
                pub const M_H_START_POINT: usize = 0x100C; // CHandle< C_BaseEntity >
                pub const M_H_END_POINT: usize = 0x1010; // CHandle< C_BaseEntity >
                pub const M_I_START_ATTACHMENT: usize = 0x1014; // AttachmentHandle_t
                pub const M_I_END_ATTACHMENT: usize = 0x1015; // AttachmentHandle_t
                pub const M_SUBDIV: usize = 0x1016; // uint8
                pub const M_ROPE_LENGTH: usize = 0x1018; // int16
                pub const M_SLACK: usize = 0x101A; // int16
                pub const M_TEXTURE_SCALE: usize = 0x101C; // float32
                pub const M_F_LOCKED_POINTS: usize = 0x1020; // uint8
                pub const M_N_CHANGE_COUNT: usize = 0x1021; // uint8
                pub const M_WIDTH: usize = 0x1024; // float32
                pub const M_PHYSICS_DELEGATE: usize = 0x1028; // C_RopeKeyframe::CPhysicsDelegate
                pub const M_H_MATERIAL: usize = 0x1038; // CStrongHandle< InfoForResourceTypeIMaterial2 >
                pub const M_TEXTURE_HEIGHT: usize = 0x1040; // int32
                pub const M_VEC_IMPULSE: usize = 0x1044; // 
                pub const M_VEC_PREVIOUS_IMPULSE: usize = 0x1050; // 
                pub const M_FL_CURRENT_GUST_TIMER: usize = 0x105C; // float32
                pub const M_FL_CURRENT_GUST_LIFETIME: usize = 0x1060; // float32
                pub const M_FL_TIME_TO_NEXT_GUST: usize = 0x1064; // float32
                pub const M_V_WIND_DIR: usize = 0x1068; // 
                pub const M_V_COLOR_MOD: usize = 0x1074; // 
                pub const M_V_CACHED_END_POINT_ATTACHMENT_POS: usize = 0x1080; // Vector[2]
                pub const M_V_CACHED_END_POINT_ATTACHMENT_ANGLE: usize = 0x1098; // QAngle[2]
                pub const M_B_CONSTRAIN_BETWEEN_ENDPOINTS: usize = 0x10B0; // bool
                pub const M_B_END_POINT_ATTACHMENT_POSITIONS_DIRTY: usize = 0x0; // bitfield:1
                pub const M_B_END_POINT_ATTACHMENT_ANGLES_DIRTY: usize = 0x0; // bitfield:1
                pub const M_B_NEW_DATA_THIS_FRAME: usize = 0x0; // bitfield:1
                pub const M_B_PHYSICS_INITTED: usize = 0x0; // bitfield:1
            }
            // Parent: c_point_entity
            // Fields count: 12
            //
            // Metadata:
            // NetworkVarNames: m_bIsPlayingBack (bool)
            // NetworkVarNames: m_bPaused (bool)
            // NetworkVarNames: m_bMultiplayer (bool)
            // NetworkVarNames: m_bAutogenerated (bool)
            // NetworkVarNames: m_flForceClientTime (float32)
            // NetworkVarNames: m_nSceneStringIndex (uint16)
            // NetworkVarNames: m_hActorList (CHandle< C_BaseFlex >)
            pub mod c_scene_entity {
                pub const M_B_IS_PLAYING_BACK: usize = 0x558; // bool
                pub const M_B_PAUSED: usize = 0x559; // bool
                pub const M_B_MULTIPLAYER: usize = 0x55A; // bool
                pub const M_B_AUTOGENERATED: usize = 0x55B; // bool
                pub const M_FL_FORCE_CLIENT_TIME: usize = 0x55C; // float32
                pub const M_N_SCENE_STRING_INDEX: usize = 0x560; // uint16
                pub const M_B_CLIENT_ONLY: usize = 0x562; // bool
                pub const M_H_OWNER: usize = 0x564; // CHandle< C_BaseFlex >
                pub const M_H_ACTOR_LIST: usize = 0x568; // C_NetworkUtlVectorBase< CHandle< C_BaseFlex > >
                pub const M_B_WAS_PLAYING: usize = 0x580; // bool
                pub const M_QUEUED_EVENTS: usize = 0x590; // CUtlVector< C_SceneEntity::QueuedEvents_t >
                pub const M_FL_CURRENT_TIME: usize = 0x5A8; // float32
            }
            // Parent: c_glow_overlay
            // Fields count: 1
            pub mod c_sun_glow_overlay {
                pub const M_B_MODULATE_BY_DOT: usize = 0xD0; // bool
            }
            // Parent: c_base_model_entity
            // Fields count: 18
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkExcludeByName
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_vDirection (Vector)
            // NetworkVarNames: m_iszEffectName (string_t)
            // NetworkVarNames: m_iszSSEffectName (string_t)
            // NetworkVarNames: m_clrOverlay (Color)
            // NetworkVarNames: m_bOn (bool)
            // NetworkVarNames: m_bmaxColor (bool)
            // NetworkVarNames: m_flSize (float32)
            // NetworkVarNames: m_flHazeScale (float32)
            // NetworkVarNames: m_flRotation (float32)
            // NetworkVarNames: m_flHDRColorScale (float32)
            // NetworkVarNames: m_flAlphaHaze (float32)
            // NetworkVarNames: m_flAlphaScale (float32)
            // NetworkVarNames: m_flAlphaHdr (float32)
            // NetworkVarNames: m_flFarZScale (float32)
            pub mod c_sun {
                pub const M_FX_SS_SUN_FLARE_EFFECT_INDEX: usize = 0xCD0; // ParticleIndex_t
                pub const M_FX_SUN_FLARE_EFFECT_INDEX: usize = 0xCD4; // ParticleIndex_t
                pub const M_FDIST_NORMALIZE: usize = 0xCD8; // float32
                pub const M_V_SUN_POS: usize = 0xCDC; // 
                pub const M_V_DIRECTION: usize = 0xCE8; // 
                pub const M_ISZ_EFFECT_NAME: usize = 0xCF8; // CUtlSymbolLarge
                pub const M_ISZ_SS_EFFECT_NAME: usize = 0xD00; // CUtlSymbolLarge
                pub const M_CLR_OVERLAY: usize = 0xD08; // Color
                pub const M_B_ON: usize = 0xD0C; // bool
                pub const M_BMAX_COLOR: usize = 0xD0D; // bool
                pub const M_FL_SIZE: usize = 0xD10; // float32
                pub const M_FL_HAZE_SCALE: usize = 0xD14; // float32
                pub const M_FL_ROTATION: usize = 0xD18; // float32
                pub const M_FL_HDR_COLOR_SCALE: usize = 0xD1C; // float32
                pub const M_FL_ALPHA_HAZE: usize = 0xD20; // float32
                pub const M_FL_ALPHA_SCALE: usize = 0xD24; // float32
                pub const M_FL_ALPHA_HDR: usize = 0xD28; // float32
                pub const M_FL_FAR_Z_SCALE: usize = 0xD2C; // float32
            }
            // Parent: c_base_toggle
            // Fields count: 2
            //
            // Metadata:
            // MNetworkIncludeByName
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_bClientSidePredicted (bool)
            pub mod c_base_trigger {
                pub const M_B_DISABLED: usize = 0xCD0; // bool
                pub const M_B_CLIENT_SIDE_PREDICTED: usize = 0xCD1; // bool
            }
            // Parent: c_base_model_entity
            // Fields count: 0
            pub mod c_trigger_volume {
            }
        }
        // Module: engine2.dll
        // Classes count: 48
        // Enums count: 4
        pub mod engine2 {
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum SpawnDebugOverrideStateT {
                SpawnDebugOverrideNone = 0,
                SpawnDebugOverrideForceEnabled = 1,
                SpawnDebugOverrideForceDisabled = 2
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum SpawnDebugRestrictionOverrideStateT {
                SpawnDebugRestrictNone = 0,
                SpawnDebugRestrictIgnoreManagerDistanceReqs = 1,
                SpawnDebugRestrictIgnoreTemplateDistanceLosReqs = 2,
                SpawnDebugRestrictIgnoreTemplateCooldownLimits = 4,
                SpawnDebugRestrictIgnoreTargetCooldownLimits = 8
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum EntityDormancyTypeT {
                EntityNotDormant = 0,
                EntityDormant = 1,
                EntitySuspended = 2
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum EntityIoTargetTypeT {
                EntityIoTargetInvalid = -1,
                EntityIoTargetEntityname = 2,
                EntityIoTargetEhandle = 6,
                EntityIoTargetEntitynameOrClassname = 7
            }
            // Parent: None
            // Fields count: 4
            pub mod engine_loop_state_t {
                pub const M_N_PLAT_WINDOW_WIDTH: usize = 0x18; // int32
                pub const M_N_PLAT_WINDOW_HEIGHT: usize = 0x1C; // int32
                pub const M_N_RENDER_WIDTH: usize = 0x20; // int32
                pub const M_N_RENDER_HEIGHT: usize = 0x24; // int32
            }
            // Parent: None
            // Fields count: 0
            pub mod event_mod_initialized_t {
            }
            // Parent: None
            // Fields count: 1
            pub mod event_frame_boundary_t {
                pub const M_FL_FRAME_TIME: usize = 0x0; // float32
            }
            // Parent: None
            // Fields count: 1
            pub mod event_profile_storage_available_t {
                pub const M_N_SPLIT_SCREEN_SLOT: usize = 0x0; // CSplitScreenSlot
            }
            // Parent: None
            // Fields count: 0
            pub mod event_split_screen_state_changed_t {
            }
            // Parent: None
            // Fields count: 8
            pub mod event_set_time_t {
                pub const M_LOOP_STATE: usize = 0x0; // EngineLoopState_t
                pub const M_N_CLIENT_OUTPUT_FRAMES: usize = 0x28; // int32
                pub const M_FL_REAL_TIME: usize = 0x30; // float64
                pub const M_FL_RENDER_TIME: usize = 0x38; // float64
                pub const M_FL_RENDER_FRAME_TIME: usize = 0x40; // float64
                pub const M_FL_RENDER_FRAME_TIME_UNBOUNDED: usize = 0x48; // float64
                pub const M_FL_RENDER_FRAME_TIME_UNSCALED: usize = 0x50; // float64
                pub const M_FL_TICK_REMAINDER: usize = 0x58; // float64
            }
            // Parent: None
            // Fields count: 2
            pub mod event_client_poll_input_t {
                pub const M_LOOP_STATE: usize = 0x0; // EngineLoopState_t
                pub const M_FL_REAL_TIME: usize = 0x28; // float32
            }
            // Parent: None
            // Fields count: 4
            pub mod event_client_process_input_t {
                pub const M_LOOP_STATE: usize = 0x0; // EngineLoopState_t
                pub const M_FL_REAL_TIME: usize = 0x28; // float32
                pub const M_FL_TICK_INTERVAL: usize = 0x2C; // float32
                pub const M_FL_TICK_START_TIME: usize = 0x30; // float64
            }
            // Parent: None
            // Fields count: 3
            pub mod event_client_process_game_input_t {
                pub const M_LOOP_STATE: usize = 0x0; // EngineLoopState_t
                pub const M_FL_REAL_TIME: usize = 0x28; // float32
                pub const M_FL_FRAME_TIME: usize = 0x2C; // float32
            }
            // Parent: None
            // Fields count: 6
            pub mod event_client_pre_output_t {
                pub const M_LOOP_STATE: usize = 0x0; // EngineLoopState_t
                pub const M_FL_RENDER_TIME: usize = 0x28; // float64
                pub const M_FL_RENDER_FRAME_TIME: usize = 0x30; // float64
                pub const M_FL_RENDER_FRAME_TIME_UNBOUNDED: usize = 0x38; // float64
                pub const M_FL_REAL_TIME: usize = 0x40; // float32
                pub const M_B_RENDER_ONLY: usize = 0x44; // bool
            }
            // Parent: None
            // Fields count: 1
            pub mod event_client_scene_system_thread_state_change_t {
                pub const M_B_THREADS_ACTIVE: usize = 0x0; // bool
            }
            // Parent: None
            // Fields count: 5
            pub mod event_client_output_t {
                pub const M_LOOP_STATE: usize = 0x0; // EngineLoopState_t
                pub const M_FL_RENDER_TIME: usize = 0x28; // float32
                pub const M_FL_REAL_TIME: usize = 0x2C; // float32
                pub const M_FL_RENDER_FRAME_TIME_UNBOUNDED: usize = 0x30; // float32
                pub const M_B_RENDER_ONLY: usize = 0x34; // bool
            }
            // Parent: None
            // Fields count: 5
            pub mod event_client_post_output_t {
                pub const M_LOOP_STATE: usize = 0x0; // EngineLoopState_t
                pub const M_FL_RENDER_TIME: usize = 0x28; // float64
                pub const M_FL_RENDER_FRAME_TIME: usize = 0x30; // float32
                pub const M_FL_RENDER_FRAME_TIME_UNBOUNDED: usize = 0x34; // float32
                pub const M_B_RENDER_ONLY: usize = 0x38; // bool
            }
            // Parent: None
            // Fields count: 4
            pub mod event_client_frame_simulate_t {
                pub const M_LOOP_STATE: usize = 0x0; // EngineLoopState_t
                pub const M_FL_REAL_TIME: usize = 0x28; // float32
                pub const M_FL_FRAME_TIME: usize = 0x2C; // float32
                pub const M_FL_WHEN_SCHEDULE_SEND_TICK_PACKET: usize = 0x30; // float64
            }
            // Parent: None
            // Fields count: 3
            pub mod event_simple_loop_frame_update_t {
                pub const M_LOOP_STATE: usize = 0x0; // EngineLoopState_t
                pub const M_FL_REAL_TIME: usize = 0x28; // float32
                pub const M_FL_FRAME_TIME: usize = 0x2C; // float32
            }
            // Parent: None
            // Fields count: 3
            pub mod event_simulate_t {
                pub const M_LOOP_STATE: usize = 0x0; // EngineLoopState_t
                pub const M_B_FIRST_TICK: usize = 0x28; // bool
                pub const M_B_LAST_TICK: usize = 0x29; // bool
            }
            // Parent: event_simulate_t
            // Fields count: 4
            pub mod event_advance_tick_t {
                pub const M_N_CURRENT_TICK: usize = 0x30; // int32
                pub const M_N_CURRENT_TICK_THIS_FRAME: usize = 0x34; // int32
                pub const M_N_TOTAL_TICKS_THIS_FRAME: usize = 0x38; // int32
                pub const M_N_TOTAL_TICKS: usize = 0x3C; // int32
            }
            // Parent: event_simulate_t
            // Fields count: 4
            pub mod event_post_advance_tick_t {
                pub const M_N_CURRENT_TICK: usize = 0x30; // int32
                pub const M_N_CURRENT_TICK_THIS_FRAME: usize = 0x34; // int32
                pub const M_N_TOTAL_TICKS_THIS_FRAME: usize = 0x38; // int32
                pub const M_N_TOTAL_TICKS: usize = 0x3C; // int32
            }
            // Parent: event_advance_tick_t
            // Fields count: 0
            pub mod event_server_advance_tick_t {
            }
            // Parent: event_post_advance_tick_t
            // Fields count: 0
            pub mod event_server_post_advance_tick_t {
            }
            // Parent: event_advance_tick_t
            // Fields count: 0
            pub mod event_client_advance_tick_t {
            }
            // Parent: event_post_advance_tick_t
            // Fields count: 0
            pub mod event_client_post_advance_tick_t {
            }
            // Parent: None
            // Fields count: 0
            pub mod event_client_prediction_post_netupdate_t {
            }
            // Parent: None
            // Fields count: 1
            pub mod event_client_poll_networking_t {
                pub const M_N_TICK_COUNT: usize = 0x0; // int32
            }
            // Parent: None
            // Fields count: 0
            pub mod event_client_process_networking_t {
            }
            // Parent: event_simulate_t
            // Fields count: 0
            pub mod event_client_pre_simulate_t {
            }
            // Parent: event_simulate_t
            // Fields count: 0
            pub mod event_client_simulate_t {
            }
            // Parent: event_simulate_t
            // Fields count: 0
            pub mod event_server_poll_networking_t {
            }
            // Parent: event_simulate_t
            // Fields count: 0
            pub mod event_server_process_networking_t {
            }
            // Parent: event_simulate_t
            // Fields count: 0
            pub mod event_server_simulate_t {
            }
            // Parent: event_simulate_t
            // Fields count: 0
            pub mod event_server_post_simulate_t {
            }
            // Parent: event_simulate_t
            // Fields count: 0
            pub mod event_client_post_simulate_t {
            }
            // Parent: event_simulate_t
            // Fields count: 0
            pub mod event_client_pause_simulate_t {
            }
            // Parent: None
            // Fields count: 1
            pub mod event_post_data_update_t {
                pub const M_N_COUNT: usize = 0x0; // int32
            }
            // Parent: None
            // Fields count: 1
            pub mod event_pre_data_update_t {
                pub const M_N_COUNT: usize = 0x0; // int32
            }
            // Parent: None
            // Fields count: 1
            pub mod event_app_shutdown_t {
                pub const M_N_DUMMY0: usize = 0x0; // int32
            }
            // Parent: None
            // Fields count: 1
            pub mod c_network_var_chainer {
                pub const M_PATH_INDEX: usize = 0x20; // ChangeAccessorFieldPathIndex_t
            }
            // Parent: None
            // Fields count: 0
            pub mod c_variant_default_allocator {
            }
            // Parent: None
            // Fields count: 0
            pub mod ent_output_t {
            }
            // Parent: None
            // Fields count: 7
            pub mod ent_component_info_t {
                pub const M_P_NAME: usize = 0x0; // char*
                pub const M_P_CPP_CLASSNAME: usize = 0x8; // char*
                pub const M_P_NETWORK_DATA_REFERENCED_DESCRIPTION: usize = 0x10; // char*
                pub const M_P_NETWORK_DATA_REFERENCED_PTR_PROP_DESCRIPTION: usize = 0x18; // char*
                pub const M_N_RUNTIME_INDEX: usize = 0x20; // int32
                pub const M_N_FLAGS: usize = 0x24; // uint32
                pub const M_P_BASE_CLASS_COMPONENT_HELPER: usize = 0x60; // CEntityComponentHelper*
            }
            // Parent: None
            // Fields count: 0
            pub mod c_entity_component {
            }
            // Parent: None
            // Fields count: 0
            pub mod ent_input_t {
            }
            // Parent: None
            // Fields count: 4
            pub mod c_entity_component_helper {
                pub const M_FLAGS: usize = 0x8; // uint32
                pub const M_P_INFO: usize = 0x10; // EntComponentInfo_t*
                pub const M_N_PRIORITY: usize = 0x18; // int32
                pub const M_P_NEXT: usize = 0x20; // CEntityComponentHelper*
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_nameStringableIndex (int32)
            pub mod c_entity_identity {
                pub const M_NAME_STRINGABLE_INDEX: usize = 0x14; // int32
                pub const M_NAME: usize = 0x18; // CUtlSymbolLarge
                pub const M_DESIGNER_NAME: usize = 0x20; // CUtlSymbolLarge
                pub const M_FLAGS: usize = 0x30; // uint32
                pub const M_WORLD_GROUP_ID: usize = 0x38; // WorldGroupId_t
                pub const M_F_DATA_OBJECT_TYPES: usize = 0x3C; // uint32
                pub const M_PATH_INDEX: usize = 0x40; // ChangeAccessorFieldPathIndex_t
                pub const M_P_PREV: usize = 0x58; // CEntityIdentity*
                pub const M_P_NEXT: usize = 0x60; // CEntityIdentity*
                pub const M_P_PREV_BY_CLASS: usize = 0x68; // CEntityIdentity*
                pub const M_P_NEXT_BY_CLASS: usize = 0x70; // CEntityIdentity*
            }
            // Parent: None
            // Fields count: 0
            pub mod c_empty_entity_instance {
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_pEntity (CEntityIdentity *)
            // NetworkVarNames: m_CScriptComponent (CScriptComponent::Storage_t)
            pub mod c_entity_instance {
                pub const M_ISZ_PRIVATE_V_SCRIPTS: usize = 0x8; // CUtlSymbolLarge
                pub const M_P_ENTITY: usize = 0x10; // CEntityIdentity*
                pub const M_C_SCRIPT_COMPONENT: usize = 0x28; // CScriptComponent*
                pub const M_B_VISIBLEIN_PVS: usize = 0x30; // bool
            }
            // Parent: None
            // Fields count: 1
            pub mod c_entity_io_output {
                pub const M_VALUE: usize = 0x18; // CVariantBase< CVariantDefaultAllocator >
            }
            // Parent: c_entity_component
            // Fields count: 1
            pub mod c_script_component {
                pub const M_SCRIPT_CLASS_NAME: usize = 0x30; // CUtlSymbolLarge
            }
        }
        // Module: host.dll
        // Classes count: 2
        // Enums count: 0
        pub mod host {
            // Parent: None
            // Fields count: 1
            pub mod c_anim_script_base {
                pub const M_B_IS_VALID: usize = 0x8; // bool
            }
            // Parent: c_anim_script_base
            // Fields count: 1
            pub mod empty_test_script {
                pub const M_H_TEST: usize = 0x10; // CAnimScriptParam< float32 >
            }
        }
        // Module: materialsystem2.dll
        // Classes count: 13
        // Enums count: 5
        pub mod materialsystem2 {
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum HorizJustificationE {
                HorizJustificationLeft = 0,
                HorizJustificationCenter = 1,
                HorizJustificationRight = 2,
                HorizJustificationNone = 3
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum VertJustificationE {
                VertJustificationTop = 0,
                VertJustificationCenter = 1,
                VertJustificationBottom = 2,
                VertJustificationNone = 3
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum LayoutPositionTypeE {
                LayoutpositiontypeViewportRelative = 0,
                LayoutpositiontypeFractional = 1,
                LayoutpositiontypeNone = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum BloomBlendModeT {
                BloomBlendAdd = 0,
                BloomBlendScreen = 1,
                BloomBlendBlur = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ViewFadeModeT {
                ViewFadeConstantColor = 0,
                ViewFadeModulate = 1,
                ViewFadeMod2x = 2
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_param_t {
                pub const M_NAME: usize = 0x0; // CUtlString
            }
            // Parent: material_param_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_param_int_t {
                pub const M_N_VALUE: usize = 0x8; // int32
            }
            // Parent: material_param_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_param_float_t {
                pub const M_FL_VALUE: usize = 0x8; // float32
            }
            // Parent: material_param_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_param_vector_t {
                pub const M_VALUE: usize = 0x8; // Vector4D
            }
            // Parent: material_param_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_param_texture_t {
                pub const M_P_VALUE: usize = 0x8; // CStrongHandle< InfoForResourceTypeCTextureBase >
            }
            // Parent: material_param_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_param_string_t {
                pub const M_VALUE: usize = 0x8; // CUtlString
            }
            // Parent: material_param_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_param_buffer_t {
                pub const M_VALUE: usize = 0x8; // CUtlBinaryBlock
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_resource_data_t {
                pub const M_MATERIAL_NAME: usize = 0x0; // CUtlString
                pub const M_SHADER_NAME: usize = 0x8; // CUtlString
                pub const M_INT_PARAMS: usize = 0x10; // CUtlVector< MaterialParamInt_t >
                pub const M_FLOAT_PARAMS: usize = 0x28; // CUtlVector< MaterialParamFloat_t >
                pub const M_VECTOR_PARAMS: usize = 0x40; // CUtlVector< MaterialParamVector_t >
                pub const M_TEXTURE_PARAMS: usize = 0x58; // CUtlVector< MaterialParamTexture_t >
                pub const M_DYNAMIC_PARAMS: usize = 0x70; // CUtlVector< MaterialParamBuffer_t >
                pub const M_DYNAMIC_TEXTURE_PARAMS: usize = 0x88; // CUtlVector< MaterialParamBuffer_t >
                pub const M_INT_ATTRIBUTES: usize = 0xA0; // CUtlVector< MaterialParamInt_t >
                pub const M_FLOAT_ATTRIBUTES: usize = 0xB8; // CUtlVector< MaterialParamFloat_t >
                pub const M_VECTOR_ATTRIBUTES: usize = 0xD0; // CUtlVector< MaterialParamVector_t >
                pub const M_TEXTURE_ATTRIBUTES: usize = 0xE8; // CUtlVector< MaterialParamTexture_t >
                pub const M_STRING_ATTRIBUTES: usize = 0x100; // CUtlVector< MaterialParamString_t >
                pub const M_RENDER_ATTRIBUTES_USED: usize = 0x118; // CUtlVector< CUtlString >
            }
            // Parent: None
            // Fields count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod post_processing_tonemap_parameters_t {
                pub const M_FL_EXPOSURE_BIAS: usize = 0x0; // float32
                pub const M_FL_SHOULDER_STRENGTH: usize = 0x4; // float32
                pub const M_FL_LINEAR_STRENGTH: usize = 0x8; // float32
                pub const M_FL_LINEAR_ANGLE: usize = 0xC; // float32
                pub const M_FL_TOE_STRENGTH: usize = 0x10; // float32
                pub const M_FL_TOE_NUM: usize = 0x14; // float32
                pub const M_FL_TOE_DENOM: usize = 0x18; // float32
                pub const M_FL_WHITE_POINT: usize = 0x1C; // float32
                pub const M_FL_LUMINANCE_SOURCE: usize = 0x20; // float32
                pub const M_FL_EXPOSURE_BIAS_SHADOWS: usize = 0x24; // float32
                pub const M_FL_EXPOSURE_BIAS_HIGHLIGHTS: usize = 0x28; // float32
                pub const M_FL_MIN_SHADOW_LUM: usize = 0x2C; // float32
                pub const M_FL_MAX_SHADOW_LUM: usize = 0x30; // float32
                pub const M_FL_MIN_HIGHLIGHT_LUM: usize = 0x34; // float32
                pub const M_FL_MAX_HIGHLIGHT_LUM: usize = 0x38; // float32
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod post_processing_bloom_parameters_t {
                pub const M_BLEND_MODE: usize = 0x0; // BloomBlendMode_t
                pub const M_FL_BLOOM_STRENGTH: usize = 0x4; // float32
                pub const M_FL_SCREEN_BLOOM_STRENGTH: usize = 0x8; // float32
                pub const M_FL_BLUR_BLOOM_STRENGTH: usize = 0xC; // float32
                pub const M_FL_BLOOM_THRESHOLD: usize = 0x10; // float32
                pub const M_FL_BLOOM_THRESHOLD_WIDTH: usize = 0x14; // float32
                pub const M_FL_SKYBOX_BLOOM_STRENGTH: usize = 0x18; // float32
                pub const M_FL_BLOOM_START_VALUE: usize = 0x1C; // float32
                pub const M_FL_BLUR_WEIGHT: usize = 0x20; // float32[5]
                pub const M_V_BLUR_TINT: usize = 0x34; // Vector[5]
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod post_processing_vignette_parameters_t {
                pub const M_FL_VIGNETTE_STRENGTH: usize = 0x0; // float32
                pub const M_V_CENTER: usize = 0x4; // Vector2D
                pub const M_FL_RADIUS: usize = 0xC; // float32
                pub const M_FL_ROUNDNESS: usize = 0x10; // float32
                pub const M_FL_FEATHER: usize = 0x14; // float32
                pub const M_V_COLOR_TINT: usize = 0x18; // 
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod post_processing_local_contrast_parameters_t {
                pub const M_FL_LOCAL_CONTRAST_STRENGTH: usize = 0x0; // float32
                pub const M_FL_LOCAL_CONTRAST_EDGE_STRENGTH: usize = 0x4; // float32
                pub const M_FL_LOCAL_CONTRAST_VIGNETTE_START: usize = 0x8; // float32
                pub const M_FL_LOCAL_CONTRAST_VIGNETTE_END: usize = 0xC; // float32
                pub const M_FL_LOCAL_CONTRAST_VIGNETTE_BLUR: usize = 0x10; // float32
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod post_processing_resource_t {
                pub const M_B_HAS_TONEMAP_PARAMS: usize = 0x0; // bool
                pub const M_TONE_MAP_PARAMS: usize = 0x4; // PostProcessingTonemapParameters_t
                pub const M_B_HAS_BLOOM_PARAMS: usize = 0x40; // bool
                pub const M_BLOOM_PARAMS: usize = 0x44; // PostProcessingBloomParameters_t
                pub const M_B_HAS_VIGNETTE_PARAMS: usize = 0xB4; // bool
                pub const M_VIGNETTE_PARAMS: usize = 0xB8; // PostProcessingVignetteParameters_t
                pub const M_B_HAS_LOCAL_CONTRAST_PARAMS: usize = 0xDC; // bool
                pub const M_LOCAL_CONSTRAST_PARAMS: usize = 0xE0; // PostProcessingLocalContrastParameters_t
                pub const M_N_COLOR_CORRECTION_VOLUME_DIM: usize = 0xF4; // int32
                pub const M_COLOR_CORRECTION_VOLUME_DATA: usize = 0xF8; // CUtlBinaryBlock
                pub const M_B_HAS_COLOR_CORRECTION: usize = 0x110; // bool
            }
        }
        // Module: meshsystem.dll
        // Classes count: 0
        // Enums count: 0
        pub mod meshsystem {
        }
        // Module: networksystem.dll
        // Classes count: 1
        // Enums count: 0
        pub mod networksystem {
            // Parent: None
            // Fields count: 1
            pub mod change_accessor_field_path_index_t {
                pub const M_VALUE: usize = 0x0; // int16
            }
        }
        // Module: panorama.dll
        // Classes count: 0
        // Enums count: 2
        pub mod panorama {
            // Alignment: 4
            // Members count: 13
            #[repr(i32)]
            pub enum ELayoutNodeType {
                Root = 0,
                Styles = 1,
                ScriptBody = 2,
                Scripts = 3,
                Snippets = 4,
                Include = 5,
                Snippet = 6,
                Panel = 7,
                PanelAttribute = 8,
                PanelAttributeValue = 9,
                ReferenceContent = 10,
                ReferenceCompiled = 11,
                ReferencePassthrough = 12
            }
            // Alignment: 4
            // Members count: 15
            #[repr(i32)]
            pub enum EStyleNodeType {
                Root = 0,
                Expression = 1,
                Property = 2,
                Define = 3,
                Import = 4,
                Keyframes = 5,
                KeyframeSelector = 6,
                StyleSelector = 7,
                Whitespace = 8,
                ExpressionText = 9,
                ExpressionUrl = 10,
                ExpressionConcat = 11,
                ReferenceContent = 12,
                ReferenceCompiled = 13,
                ReferencePassthrough = 14
            }
        }
        // Module: particles.dll
        // Classes count: 250
        // Enums count: 71
        pub mod particles {
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum ParticleControlPointAxisT {
                ParticleCpAxisX = 0,
                ParticleCpAxisY = 1,
                ParticleCpAxisZ = 2,
                ParticleCpAxisNegativeX = 3,
                ParticleCpAxisNegativeY = 4,
                ParticleCpAxisNegativeZ = 5
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum ParticleImpulseTypeT {
                ImpulseTypeNone = 0,
                ImpulseTypeGeneric = 1,
                ImpulseTypeRope = 2,
                ImpulseTypeExplosion = 4,
                ImpulseTypeExplosionUnderwater = 8,
                ImpulseTypeParticleSystem = 16
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleFalloffFunctionT {
                ParticleFalloffConstant = 0,
                ParticleFalloffLinear = 1,
                ParticleFalloffExponential = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum AnimationTypeT {
                AnimationTypeFixedRate = 0,
                AnimationTypeFitLifetime = 1,
                AnimationTypeManualFrames = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ClosestPointTestTypeT {
                ParticleClosestTypeBox = 0,
                ParticleClosestTypeCapsule = 1,
                ParticleClosestTypeHybrid = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum InheritableBoolTypeT {
                InheritableBoolInherit = 0,
                InheritableBoolFalse = 1,
                InheritableBoolTrue = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ParticleHitboxBiasTypeT {
                ParticleHitboxBiasEntity = 0,
                ParticleHitboxBiasHitbox = 1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ParticleAttrBoxFlagsT {
                ParticleAttrBoxFlagsNone = 0,
                ParticleAttrBoxFlagsWater = 1
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum PFuncVisualizationTypeT {
                PfuncVisualizationSphereWireframe = 0,
                PfuncVisualizationSphereSolid = 1,
                PfuncVisualizationBox = 2,
                PfuncVisualizationRing = 3,
                PfuncVisualizationPlane = 4,
                PfuncVisualizationLine = 5,
                PfuncVisualizationCylinder = 6
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum PetGroundTypeT {
                PetGroundNone = 0,
                PetGroundGrid = 1,
                PetGroundPlane = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum SpriteCardShaderTypeT {
                SpritecardShaderBase = 0,
                SpritecardShaderCustom = 1
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum ParticleTopologyT {
                ParticleTopologyPoints = 0,
                ParticleTopologyLines = 1,
                ParticleTopologyTris = 2,
                ParticleTopologyQuads = 3,
                ParticleTopologyCubes = 4
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ParticleDetailLevelT {
                ParticledetailLow = 0,
                ParticledetailMedium = 1,
                ParticledetailHigh = 2,
                ParticledetailUltra = 3
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ParticleTraceSetT {
                ParticleTraceSetAll = 0,
                ParticleTraceSetStatic = 1,
                ParticleTraceSetStaticAndKeyframed = 2,
                ParticleTraceSetDynamic = 3
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum ParticleCollisionModeT {
                CollisionModePerParticleTrace = 3,
                CollisionModeUseNearestTrace = 2,
                CollisionModePerFramePlaneset = 1,
                CollisionModeInitialTraceDown = 0,
                CollisionModeDisabled = -1
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum ParticleColorBlendModeT {
                ParticleblendDefault = 0,
                ParticleblendOverlay = 1,
                ParticleblendDarken = 2,
                ParticleblendLighten = 3,
                ParticleblendMultiply = 4
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum Detail2ComboT {
                Detail2ComboUninitialized = -1,
                Detail2ComboOff = 0,
                Detail2ComboAdd = 1,
                Detail2ComboAddSelfIllum = 2,
                Detail2ComboMod2x = 3,
                Detail2ComboMul = 4,
                Detail2ComboCrossfade = 5
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum DetailComboT {
                DetailComboOff = 0,
                DetailComboAdd = 1,
                DetailComboAddSelfIllum = 2,
                DetailComboMod2x = 3
            }
            // Alignment: 4
            // Members count: 9
            #[repr(i32)]
            pub enum ScalarExpressionTypeT {
                ScalarExpressionUninitialized = -1,
                ScalarExpressionAdd = 0,
                ScalarExpressionSubtract = 1,
                ScalarExpressionMul = 2,
                ScalarExpressionDivide = 3,
                ScalarExpressionInput1 = 4,
                ScalarExpressionMin = 5,
                ScalarExpressionMax = 6,
                ScalarExpressionMod = 7
            }
            // Alignment: 4
            // Members count: 9
            #[repr(i32)]
            pub enum VectorExpressionTypeT {
                VectorExpressionUninitialized = -1,
                VectorExpressionAdd = 0,
                VectorExpressionSubtract = 1,
                VectorExpressionMul = 2,
                VectorExpressionDivide = 3,
                VectorExpressionInput1 = 4,
                VectorExpressionMin = 5,
                VectorExpressionMax = 6,
                VectorExpressionCrossproduct = 7
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum VectorFloatExpressionTypeT {
                VectorFloatExpressionUninitialized = -1,
                VectorFloatExpressionDotproduct = 0,
                VectorFloatExpressionDistance = 1,
                VectorFloatExpressionDistancesqr = 2,
                VectorFloatExpressionInput1Length = 3,
                VectorFloatExpressionInput1Lengthsqr = 4,
                VectorFloatExpressionInput1Noise = 5
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum MissingParentInheritBehaviorT {
                MissingParentDoNothing = -1,
                MissingParentKill = 0,
                MissingParentFindNew = 1,
                MissingParentSameIndex = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum HitboxLerpTypeT {
                HitboxLerpLifetime = 0,
                HitboxLerpConstant = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleSelectionT {
                ParticleSelectionFirst = 0,
                ParticleSelectionLast = 1,
                ParticleSelectionNumber = 2
            }
            // Alignment: 4
            // Members count: 12
            #[repr(i32)]
            pub enum ParticlePinDistanceT {
                ParticlePinDistanceNone = -1,
                ParticlePinDistanceNeighbor = 0,
                ParticlePinDistanceFarthest = 1,
                ParticlePinDistanceFirst = 2,
                ParticlePinDistanceLast = 3,
                ParticlePinDistanceCenter = 5,
                ParticlePinDistanceCp = 6,
                ParticlePinDistanceCpPairEither = 7,
                ParticlePinDistanceCpPairBoth = 8,
                ParticlePinSpeed = 9,
                ParticlePinCollectionAge = 10,
                ParticlePinFloatValue = 11
            }
            // Alignment: 4
            // Members count: 13
            #[repr(i32)]
            pub enum ParticleColorBlendTypeT {
                ParticleColorBlendMultiply = 0,
                ParticleColorBlendMultiply2x = 1,
                ParticleColorBlendDivide = 2,
                ParticleColorBlendAdd = 3,
                ParticleColorBlendSubtract = 4,
                ParticleColorBlendMod2x = 5,
                ParticleColorBlendScreen = 6,
                ParticleColorBlendMax = 7,
                ParticleColorBlendMin = 8,
                ParticleColorBlendReplace = 9,
                ParticleColorBlendAverage = 10,
                ParticleColorBlendNegate = 11,
                ParticleColorBlendLuminance = 12
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum ParticleSetMethodT {
                ParticleSetReplaceValue = 0,
                ParticleSetScaleInitialValue = 1,
                ParticleSetAddToInitialValue = 2,
                ParticleSetRampCurrentValue = 3,
                ParticleSetScaleCurrentValue = 4,
                ParticleSetAddToCurrentValue = 5
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleDirectionNoiseTypeT {
                ParticleDirNoisePerlin = 0,
                ParticleDirNoiseCurl = 1,
                ParticleDirNoiseWorleyBasic = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleRotationLockTypeT {
                ParticleRotationLockNone = 0,
                ParticleRotationLockRotations = 1,
                ParticleRotationLockNormal = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleEndcapModeT {
                ParticleEndcapAlwaysOn = -1,
                ParticleEndcapEndcapOff = 0,
                ParticleEndcapEndcapOn = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleLightingQualityT {
                ParticleLightingPerParticle = 0,
                ParticleLightingPerVertex = 1,
                ParticleLightingPerPixel = -1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum StandardLightingAttenuationStyleT {
                LightStyleOld = 0,
                LightStyleNew = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleTraceMissBehaviorT {
                ParticleTraceMissBehaviorNone = 0,
                ParticleTraceMissBehaviorKill = 1,
                ParticleTraceMissBehaviorTraceEnd = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ParticleOrientationSetModeT {
                ParticleOrientationSetFromVelocity = 0,
                ParticleOrientationSetFromRotations = 1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ParticleLightnintBranchBehaviorT {
                ParticleLightningBranchCurrentDir = 0,
                ParticleLightningBranchEndpointDir = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleLightFogLightingModeT {
                ParticleLightFogLightingModeNone = 0,
                ParticleLightFogLightingModeDynamic = 2,
                ParticleLightFogLightingModeDynamicNoshadows = 4
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleSequenceCropOverrideT {
                ParticleSequenceCropOverrideDefault = -1,
                ParticleSequenceCropOverrideForceOff = 0,
                ParticleSequenceCropOverrideForceOn = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleParentSetModeT {
                ParticleSetParentNo = 0,
                ParticleSetParentImmediate = 1,
                ParticleSetParentRoot = 1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum MaterialProxyTypeT {
                MaterialProxyStatusEffect = 0,
                MaterialProxyTint = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum BBoxVolumeTypeT {
                BboxVolume = 0,
                BboxDimensions = 1,
                BboxMinsMaxs = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ParticleHitboxDataSelectionT {
                ParticleHitboxAverageSpeed = 0,
                ParticleHitboxCount = 1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum SnapshotIndexTypeT {
                SnapshotIndexIncrement = 0,
                SnapshotIndexDirect = 1
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum ParticleOrientationChoiceListT {
                ParticleOrientationScreenAligned = 0,
                ParticleOrientationScreenZAligned = 1,
                ParticleOrientationWorldZAligned = 2,
                ParticleOrientationAlignToParticleNormal = 3,
                ParticleOrientationScreenalignToParticleNormal = 4,
                ParticleOrientationFull3axisRotation = 5
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum ParticleOutputBlendModeT {
                ParticleOutputBlendModeAlpha = 0,
                ParticleOutputBlendModeAdd = 1,
                ParticleOutputBlendModeBlendAdd = 2,
                ParticleOutputBlendModeHalfBlendAdd = 3,
                ParticleOutputBlendModeNegHalfBlendAdd = 4,
                ParticleOutputBlendModeMod2x = 5,
                ParticleOutputBlendModeLighten = 6
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ParticleAlphaReferenceTypeT {
                ParticleAlphaReferenceAlphaAlpha = 0,
                ParticleAlphaReferenceOpaqueAlpha = 1,
                ParticleAlphaReferenceAlphaOpaque = 2,
                ParticleAlphaReferenceOpaqueOpaque = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum BlurFilterTypeT {
                BlurfilterGaussian = 0,
                BlurfilterBox = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ParticleLightTypeChoiceListT {
                ParticleLightTypePoint = 0,
                ParticleLightTypeSpot = 1,
                ParticleLightTypeFx = 2,
                ParticleLightTypeCapsule = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ParticleLightUnitChoiceListT {
                ParticleLightUnitCandelas = 0,
                ParticleLightUnitLumens = 1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ParticleOmni2LightTypeChoiceListT {
                ParticleOmni2LightTypePoint = 0,
                ParticleOmni2LightTypeSphere = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleLightBehaviorChoiceListT {
                ParticleLightBehaviorFollowDirection = 0,
                ParticleLightBehaviorRope = 1,
                ParticleLightBehaviorTrails = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleDepthFeatheringModeT {
                ParticleDepthFeatheringOff = 0,
                ParticleDepthFeatheringOnOptional = 1,
                ParticleDepthFeatheringOnRequired = 2
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ParticleVrHandChoiceListT {
                ParticleVrhandLeft = 0,
                ParticleVrhandRight = 1,
                ParticleVrhandCp = 2,
                ParticleVrhandCpObject = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ParticleSortingChoiceListT {
                ParticleSortingNearest = 0,
                ParticleSortingCreationTime = 1
            }
            // Alignment: 4
            // Members count: 10
            #[repr(i32)]
            pub enum SpriteCardTextureTypeT {
                SpritecardTextureDiffuse = 0,
                SpritecardTextureZoom = 1,
                SpritecardTexture1dColorLookup = 2,
                SpritecardTextureUvdistortion = 3,
                SpritecardTextureUvdistortionZoom = 4,
                SpritecardTextureNormalmap = 5,
                SpritecardTextureAnimmotionvec = 6,
                SpritecardTextureSphericalHarmonicsA = 7,
                SpritecardTextureSphericalHarmonicsB = 8,
                SpritecardTextureSphericalHarmonicsC = 9
            }
            // Alignment: 4
            // Members count: 15
            #[repr(i32)]
            pub enum SpriteCardTextureChannelT {
                SpritecardTextureChannelMixRgb = 0,
                SpritecardTextureChannelMixRgba = 1,
                SpritecardTextureChannelMixA = 2,
                SpritecardTextureChannelMixRgbA = 3,
                SpritecardTextureChannelMixRgbAlphamask = 4,
                SpritecardTextureChannelMixRgbRgbmask = 5,
                SpritecardTextureChannelMixRgbaRgbalpha = 6,
                SpritecardTextureChannelMixARgbalpha = 7,
                SpritecardTextureChannelMixRgbARgbalpha = 8,
                SpritecardTextureChannelMixR = 9,
                SpritecardTextureChannelMixG = 10,
                SpritecardTextureChannelMixB = 11,
                SpritecardTextureChannelMixRalpha = 12,
                SpritecardTextureChannelMixGalpha = 13,
                SpritecardTextureChannelMixBalpha = 14
            }
            // Alignment: 4
            // Members count: 14
            #[repr(i32)]
            pub enum SpriteCardPerParticleScaleT {
                SpritecardTexturePpScaleNone = 0,
                SpritecardTexturePpScaleParticleAge = 1,
                SpritecardTexturePpScaleAnimationFrame = 2,
                SpritecardTexturePpScaleShaderExtraData1 = 3,
                SpritecardTexturePpScaleShaderExtraData2 = 4,
                SpritecardTexturePpScaleParticleAlpha = 5,
                SpritecardTexturePpScaleShaderRadius = 6,
                SpritecardTexturePpScaleRoll = 7,
                SpritecardTexturePpScaleYaw = 8,
                SpritecardTexturePpScalePitch = 9,
                SpritecardTexturePpScaleRandom = 10,
                SpritecardTexturePpScaleNegRandom = 11,
                SpritecardTexturePpScaleRandomTime = 12,
                SpritecardTexturePpScaleNegRandomTime = 13
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum ParticleTextureLayerBlendTypeT {
                SpritecardTextureBlendMultiply = 0,
                SpritecardTextureBlendMod2x = 1,
                SpritecardTextureBlendReplace = 2,
                SpritecardTextureBlendAdd = 3,
                SpritecardTextureBlendSubtract = 4,
                SpritecardTextureBlendAverage = 5,
                SpritecardTextureBlendLuminance = 6
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum ParticlePostProcessPriorityGroupT {
                ParticlePostProcessPriorityLevelVolume = 0,
                ParticlePostProcessPriorityLevelOverride = 1,
                ParticlePostProcessPriorityGameplayEffect = 2,
                ParticlePostProcessPriorityGameplayStateLow = 3,
                ParticlePostProcessPriorityGameplayStateHigh = 4,
                ParticlePostProcessPriorityGlobalUi = 5
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ParticleFogTypeT {
                ParticleFogGameDefault = 0,
                ParticleFogEnabled = 1,
                ParticleFogDisabled = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum TextureRepetitionModeT {
                TextureRepetitionParticle = 0,
                TextureRepetitionPath = 1
            }
            // Alignment: 4
            // Members count: 24
            #[repr(i32)]
            pub enum ParticleFloatTypeT {
                PfTypeInvalid = -1,
                PfTypeLiteral = 0,
                PfTypeNamedValue = 1,
                PfTypeRandomUniform = 2,
                PfTypeRandomBiased = 3,
                PfTypeCollectionAge = 4,
                PfTypeEndcapAge = 5,
                PfTypeControlPointComponent = 6,
                PfTypeControlPointChangeAge = 7,
                PfTypeControlPointSpeed = 8,
                PfTypeParticleDetailLevel = 9,
                PfTypeConcurrentDefCount = 10,
                PfTypeClosestCameraDistance = 11,
                PfTypeRendererCameraDistance = 12,
                PfTypeRendererCameraDotProduct = 13,
                PfTypeParticleNoise = 14,
                PfTypeParticleAge = 15,
                PfTypeParticleAgeNormalized = 16,
                PfTypeParticleFloat = 17,
                PfTypeParticleVectorComponent = 18,
                PfTypeParticleSpeed = 19,
                PfTypeParticleNumber = 20,
                PfTypeParticleNumberNormalized = 21,
                PfTypeCount = 22
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum ParticleFloatBiasTypeT {
                PfBiasTypeInvalid = -1,
                PfBiasTypeStandard = 0,
                PfBiasTypeGain = 1,
                PfBiasTypeExponential = 2,
                PfBiasTypeCount = 3
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum PfNoiseTypeT {
                PfNoiseTypePerlin = 0,
                PfNoiseTypeSimplex = 1,
                PfNoiseTypeWorley = 2,
                PfNoiseTypeCurl = 3
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum PfNoiseModifierT {
                PfNoiseModifierNone = 0,
                PfNoiseModifierLines = 1,
                PfNoiseModifierClumps = 2,
                PfNoiseModifierRings = 3
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum PfNoiseTurbulenceT {
                PfNoiseTurbNone = 0,
                PfNoiseTurbHighlight = 1,
                PfNoiseTurbFeedback = 2,
                PfNoiseTurbLoopy = 3,
                PfNoiseTurbContrast = 4,
                PfNoiseTurbAlternate = 5
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ParticleFloatRandomModeT {
                PfRandomModeInvalid = -1,
                PfRandomModeConstant = 0,
                PfRandomModeVarying = 1,
                PfRandomModeCount = 2
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ParticleFloatInputModeT {
                PfInputModeInvalid = -1,
                PfInputModeClamped = 0,
                PfInputModeLooped = 1,
                PfInputModeCount = 2
            }
            // Alignment: 4
            // Members count: 8
            #[repr(i32)]
            pub enum ParticleFloatMapTypeT {
                PfMapTypeInvalid = -1,
                PfMapTypeDirect = 0,
                PfMapTypeMult = 1,
                PfMapTypeRemap = 2,
                PfMapTypeRemapBiased = 3,
                PfMapTypeCurve = 4,
                PfMapTypeNotched = 5,
                PfMapTypeCount = 6
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum ParticleTransformTypeT {
                PtTypeInvalid = 0,
                PtTypeNamedValue = 1,
                PtTypeControlPoint = 2,
                PtTypeControlPointRange = 3,
                PtTypeCount = 4
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum ParticleModelTypeT {
                PmTypeInvalid = 0,
                PmTypeNamedValueModel = 1,
                PmTypeNamedValueEhandle = 2,
                PmTypeControlPoint = 3,
                PmTypeCount = 4
            }
            // Alignment: 4
            // Members count: 19
            #[repr(i32)]
            pub enum ParticleVecTypeT {
                PvecTypeInvalid = -1,
                PvecTypeLiteral = 0,
                PvecTypeLiteralColor = 1,
                PvecTypeNamedValue = 2,
                PvecTypeParticleVector = 3,
                PvecTypeParticleVelocity = 4,
                PvecTypeCpValue = 5,
                PvecTypeCpRelativePosition = 6,
                PvecTypeCpRelativeDir = 7,
                PvecTypeCpRelativeRandomDir = 8,
                PvecTypeFloatComponents = 9,
                PvecTypeFloatInterpClamped = 10,
                PvecTypeFloatInterpOpen = 11,
                PvecTypeFloatInterpGradient = 12,
                PvecTypeRandomUniform = 13,
                PvecTypeRandomUniformOffset = 14,
                PvecTypeCpDelta = 15,
                PvecTypeClosestCameraPosition = 16,
                PvecTypeCount = 17
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_random_number_generator_parameters {
                pub const M_B_DISTRIBUTE_EVENLY: usize = 0x0; // bool
                pub const M_N_SEED: usize = 0x4; // int32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_variable_t {
                pub const M_STR_VARIABLE: usize = 0x0; // CUtlString
                pub const M_N_VARIABLE_FIELD: usize = 0x8; // ParticleAttributeIndex_t
                pub const M_FL_SCALE: usize = 0xC; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod render_projected_material_t {
                pub const M_H_MATERIAL: usize = 0x0; // CStrongHandle< InfoForResourceTypeIMaterial2 >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod particle_preview_body_group_t {
                pub const M_BODY_GROUP_NAME: usize = 0x0; // CUtlString
                pub const M_N_VALUE: usize = 0x8; // int32
            }
            // Parent: None
            // Fields count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod particle_preview_state_t {
                pub const M_PREVIEW_MODEL: usize = 0x0; // CUtlString
                pub const M_N_MOD_SPECIFIC_DATA: usize = 0x8; // uint32
                pub const M_GROUND_TYPE: usize = 0xC; // PetGroundType_t
                pub const M_SEQUENCE_NAME: usize = 0x10; // CUtlString
                pub const M_N_FIRE_PARTICLE_ON_SEQUENCE_FRAME: usize = 0x18; // int32
                pub const M_HITBOX_SET_NAME: usize = 0x20; // CUtlString
                pub const M_MATERIAL_GROUP_NAME: usize = 0x28; // CUtlString
                pub const M_VEC_BODY_GROUPS: usize = 0x30; // CUtlVector< ParticlePreviewBodyGroup_t >
                pub const M_FL_PLAYBACK_SPEED: usize = 0x48; // float32
                pub const M_FL_PARTICLE_SIMULATION_RATE: usize = 0x4C; // float32
                pub const M_B_SHOULD_DRAW_HITBOXES: usize = 0x50; // bool
                pub const M_B_SHOULD_DRAW_ATTACHMENTS: usize = 0x51; // bool
                pub const M_B_SHOULD_DRAW_ATTACHMENT_NAMES: usize = 0x52; // bool
                pub const M_B_SHOULD_DRAW_CONTROL_POINT_AXES: usize = 0x53; // bool
                pub const M_B_ANIMATION_NON_LOOPING: usize = 0x54; // bool
                pub const M_VEC_PREVIEW_GRAVITY: usize = 0x58; // 
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod particle_control_point_driver_t {
                pub const M_I_CONTROL_POINT: usize = 0x0; // int32
                pub const M_I_ATTACH_TYPE: usize = 0x4; // ParticleAttachment_t
                pub const M_ATTACHMENT_NAME: usize = 0x8; // CUtlString
                pub const M_VEC_OFFSET: usize = 0x10; // 
                pub const M_ANG_OFFSET: usize = 0x1C; // QAngle
                pub const M_ENTITY_NAME: usize = 0x28; // CUtlString
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod particle_control_point_configuration_t {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_DRIVERS: usize = 0x8; // CUtlVector< ParticleControlPointDriver_t >
                pub const M_PREVIEW_STATE: usize = 0x20; // ParticlePreviewState_t
            }
            // Parent: None
            // Fields count: 0
            pub mod i_particle_collection {
            }
            // Parent: None
            // Fields count: 0
            pub mod i_control_point_editor_data {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod i_particle_system_definition {
            }
            // Parent: None
            // Fields count: 18
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_visibility_inputs {
                pub const M_FL_CAMERA_BIAS: usize = 0x0; // float32
                pub const M_N_C_PIN: usize = 0x4; // int32
                pub const M_FL_PROXY_RADIUS: usize = 0x8; // float32
                pub const M_FL_INPUT_MIN: usize = 0xC; // float32
                pub const M_FL_INPUT_MAX: usize = 0x10; // float32
                pub const M_FL_NO_PIXEL_VISIBILITY_FALLBACK: usize = 0x14; // float32
                pub const M_FL_DISTANCE_INPUT_MIN: usize = 0x18; // float32
                pub const M_FL_DISTANCE_INPUT_MAX: usize = 0x1C; // float32
                pub const M_FL_DOT_INPUT_MIN: usize = 0x20; // float32
                pub const M_FL_DOT_INPUT_MAX: usize = 0x24; // float32
                pub const M_B_DOT_CP_ANGLES: usize = 0x28; // bool
                pub const M_B_DOT_CAMERA_ANGLES: usize = 0x29; // bool
                pub const M_FL_ALPHA_SCALE_MIN: usize = 0x2C; // float32
                pub const M_FL_ALPHA_SCALE_MAX: usize = 0x30; // float32
                pub const M_FL_RADIUS_SCALE_MIN: usize = 0x34; // float32
                pub const M_FL_RADIUS_SCALE_MAX: usize = 0x38; // float32
                pub const M_FL_RADIUS_SCALE_FOV_BASE: usize = 0x3C; // float32
                pub const M_B_RIGHT_EYE: usize = 0x40; // bool
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_path_parameters {
                pub const M_N_START_CONTROL_POINT_NUMBER: usize = 0x0; // int32
                pub const M_N_END_CONTROL_POINT_NUMBER: usize = 0x4; // int32
                pub const M_N_BULGE_CONTROL: usize = 0x8; // int32
                pub const M_FL_BULGE: usize = 0xC; // float32
                pub const M_FL_MID_POINT: usize = 0x10; // float32
                pub const M_V_START_POINT_OFFSET: usize = 0x14; // 
                pub const M_V_MID_POINT_OFFSET: usize = 0x20; // 
                pub const M_V_END_OFFSET: usize = 0x2C; // 
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod particle_children_info_t {
                pub const M_CHILD_REF: usize = 0x0; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
                pub const M_FL_DELAY: usize = 0x8; // float32
                pub const M_B_END_CAP: usize = 0xC; // bool
                pub const M_B_DISABLE_CHILD: usize = 0xD; // bool
                pub const M_N_DETAIL_LEVEL: usize = 0x10; // ParticleDetailLevel_t
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod control_point_reference_t {
                pub const M_CONTROL_POINT_NAME_STRING: usize = 0x0; // int32
                pub const M_V_OFFSET_FROM_CONTROL_POINT: usize = 0x4; // 
                pub const M_B_OFFSET_IN_LOCAL_SPACE: usize = 0x10; // bool
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod model_reference_t {
                pub const M_MODEL: usize = 0x0; // CStrongHandle< InfoForResourceTypeCModel >
                pub const M_FL_RELATIVE_PROBABILITY_OF_SPAWN: usize = 0x8; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod sequence_weighted_list_t {
                pub const M_N_SEQUENCE: usize = 0x0; // int32
                pub const M_FL_RELATIVE_WEIGHT: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod collision_group_context_t {
                pub const M_N_COLLISION_GROUP_NUMBER: usize = 0x0; // int32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod point_definition_t {
                pub const M_N_CONTROL_POINT: usize = 0x0; // int32
                pub const M_B_LOCAL_COORDS: usize = 0x4; // bool
                pub const M_V_OFFSET: usize = 0x8; // 
            }
            // Parent: point_definition_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod point_definition_with_time_values_t {
                pub const M_FL_TIME_DURATION: usize = 0x14; // float32
            }
            // Parent: i_particle_system_definition
            // Fields count: 64
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_system_definition {
                pub const M_N_BEHAVIOR_VERSION: usize = 0x8; // int32
                pub const M_PRE_EMISSION_OPERATORS: usize = 0x10; // CUtlVector< CParticleFunctionPreEmission* >
                pub const M_EMITTERS: usize = 0x28; // CUtlVector< CParticleFunctionEmitter* >
                pub const M_INITIALIZERS: usize = 0x40; // CUtlVector< CParticleFunctionInitializer* >
                pub const M_OPERATORS: usize = 0x58; // CUtlVector< CParticleFunctionOperator* >
                pub const M_FORCE_GENERATORS: usize = 0x70; // CUtlVector< CParticleFunctionForce* >
                pub const M_CONSTRAINTS: usize = 0x88; // CUtlVector< CParticleFunctionConstraint* >
                pub const M_RENDERERS: usize = 0xA0; // CUtlVector< CParticleFunctionRenderer* >
                pub const M_CHILDREN: usize = 0xB8; // CUtlVector< ParticleChildrenInfo_t >
                pub const M_N_FIRST_MULTIPLE_OVERRIDE_BACKWARD_COMPAT: usize = 0x178; // int32
                pub const M_N_INITIAL_PARTICLES: usize = 0x210; // int32
                pub const M_N_MAX_PARTICLES: usize = 0x214; // int32
                pub const M_N_GROUP_ID: usize = 0x218; // int32
                pub const M_BOUNDING_BOX_MIN: usize = 0x21C; // 
                pub const M_BOUNDING_BOX_MAX: usize = 0x228; // 
                pub const M_FL_DEPTH_SORT_BIAS: usize = 0x234; // float32
                pub const M_N_SORT_OVERRIDE_POSITION_CP: usize = 0x238; // int32
                pub const M_B_INFINITE_BOUNDS: usize = 0x23C; // bool
                pub const M_B_ENABLE_NAMED_VALUES: usize = 0x23D; // bool
                pub const M_NAMED_VALUE_DOMAIN: usize = 0x240; // CUtlString
                pub const M_NAMED_VALUE_LOCALS: usize = 0x248; // CUtlVector< ParticleNamedValueSource_t* >
                pub const M_CONSTANT_COLOR: usize = 0x260; // Color
                pub const M_CONSTANT_NORMAL: usize = 0x264; // 
                pub const M_FL_CONSTANT_RADIUS: usize = 0x270; // float32
                pub const M_FL_CONSTANT_ROTATION: usize = 0x274; // float32
                pub const M_FL_CONSTANT_ROTATION_SPEED: usize = 0x278; // float32
                pub const M_FL_CONSTANT_LIFESPAN: usize = 0x27C; // float32
                pub const M_N_CONSTANT_SEQUENCE_NUMBER: usize = 0x280; // int32
                pub const M_N_CONSTANT_SEQUENCE_NUMBER1: usize = 0x284; // int32
                pub const M_N_SNAPSHOT_CONTROL_POINT: usize = 0x288; // int32
                pub const M_H_SNAPSHOT: usize = 0x290; // CStrongHandle< InfoForResourceTypeIParticleSnapshot >
                pub const M_PSZ_CULL_REPLACEMENT_NAME: usize = 0x298; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
                pub const M_FL_CULL_RADIUS: usize = 0x2A0; // float32
                pub const M_FL_CULL_FILL_COST: usize = 0x2A4; // float32
                pub const M_N_CULL_CONTROL_POINT: usize = 0x2A8; // int32
                pub const M_H_FALLBACK: usize = 0x2B0; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
                pub const M_N_FALLBACK_MAX_COUNT: usize = 0x2B8; // int32
                pub const M_H_LOW_VIOLENCE_DEF: usize = 0x2C0; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
                pub const M_H_REFERENCE_REPLACEMENT: usize = 0x2C8; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
                pub const M_FL_PRE_SIMULATION_TIME: usize = 0x2D0; // float32
                pub const M_FL_STOP_SIMULATION_AFTER_TIME: usize = 0x2D4; // float32
                pub const M_FL_MAXIMUM_TIME_STEP: usize = 0x2D8; // float32
                pub const M_FL_MAXIMUM_SIM_TIME: usize = 0x2DC; // float32
                pub const M_FL_MINIMUM_SIM_TIME: usize = 0x2E0; // float32
                pub const M_FL_MINIMUM_TIME_STEP: usize = 0x2E4; // float32
                pub const M_N_MINIMUM_FRAMES: usize = 0x2E8; // int32
                pub const M_N_MIN_CPU_LEVEL: usize = 0x2EC; // int32
                pub const M_N_MIN_GPU_LEVEL: usize = 0x2F0; // int32
                pub const M_FL_NO_DRAW_TIME_TO_GO_TO_SLEEP: usize = 0x2F4; // float32
                pub const M_FL_MAX_DRAW_DISTANCE: usize = 0x2F8; // float32
                pub const M_FL_START_FADE_DISTANCE: usize = 0x2FC; // float32
                pub const M_FL_MAX_CREATION_DISTANCE: usize = 0x300; // float32
                pub const M_N_AGGREGATION_MIN_AVAILABLE_PARTICLES: usize = 0x304; // int32
                pub const M_FL_AGGREGATE_RADIUS: usize = 0x308; // float32
                pub const M_B_SHOULD_BATCH: usize = 0x30C; // bool
                pub const M_B_SHOULD_HITBOXES_FALLBACK_TO_RENDER_BOUNDS: usize = 0x30D; // bool
                pub const M_B_SHOULD_HITBOXES_FALLBACK_TO_SNAPSHOT: usize = 0x30E; // bool
                pub const M_N_VIEW_MODEL_EFFECT: usize = 0x310; // InheritableBoolType_t
                pub const M_B_SCREEN_SPACE_EFFECT: usize = 0x314; // bool
                pub const M_PSZ_TARGET_LAYER_ID: usize = 0x318; // CUtlSymbolLarge
                pub const M_N_SKIP_RENDER_CONTROL_POINT: usize = 0x320; // int32
                pub const M_N_ALLOW_RENDER_CONTROL_POINT: usize = 0x324; // int32
                pub const M_B_SHOULD_SORT: usize = 0x328; // bool
                pub const M_CONTROL_POINT_CONFIGURATIONS: usize = 0x370; // CUtlVector< ParticleControlPointConfiguration_t >
            }
            // Parent: None
            // Fields count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_function {
                pub const M_FL_OP_STRENGTH: usize = 0x8; // CParticleCollectionFloatInput
                pub const M_N_OP_END_CAP_STATE: usize = 0x160; // ParticleEndcapMode_t
                pub const M_FL_OP_START_FADE_IN_TIME: usize = 0x164; // float32
                pub const M_FL_OP_END_FADE_IN_TIME: usize = 0x168; // float32
                pub const M_FL_OP_START_FADE_OUT_TIME: usize = 0x16C; // float32
                pub const M_FL_OP_END_FADE_OUT_TIME: usize = 0x170; // float32
                pub const M_FL_OP_FADE_OSCILLATE_PERIOD: usize = 0x174; // float32
                pub const M_B_NORMALIZE_TO_STOP_TIME: usize = 0x178; // bool
                pub const M_FL_OP_TIME_OFFSET_MIN: usize = 0x17C; // float32
                pub const M_FL_OP_TIME_OFFSET_MAX: usize = 0x180; // float32
                pub const M_N_OP_TIME_OFFSET_SEED: usize = 0x184; // int32
                pub const M_N_OP_TIME_SCALE_SEED: usize = 0x188; // int32
                pub const M_FL_OP_TIME_SCALE_MIN: usize = 0x18C; // float32
                pub const M_FL_OP_TIME_SCALE_MAX: usize = 0x190; // float32
                pub const M_B_DISABLE_OPERATOR: usize = 0x196; // bool
                pub const M_NOTES: usize = 0x198; // CUtlString
            }
            // Parent: c_particle_function
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_function_initializer {
                pub const M_N_ASSOCIATED_EMITTER_INDEX: usize = 0x1B8; // int32
            }
            // Parent: c_particle_function
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_function_emitter {
                pub const M_N_EMITTER_INDEX: usize = 0x1B8; // int32
            }
            // Parent: c_particle_function
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_function_constraint {
            }
            // Parent: c_particle_function
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_function_operator {
            }
            // Parent: c_particle_function
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_function_force {
            }
            // Parent: c_particle_function_operator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_function_pre_emission {
                pub const M_B_RUN_ONCE: usize = 0x1C0; // bool
            }
            // Parent: c_particle_function
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_function_renderer {
                pub const VISIBILITY_INPUTS: usize = 0x1B8; // CParticleVisibilityInputs
                pub const M_B_CANNOT_BE_REFRACTED: usize = 0x1FC; // bool
                pub const M_B_SKIP_RENDERING_ON_MOBILE: usize = 0x1FD; // bool
            }
            // Parent: None
            // Fields count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod texture_controls_t {
                pub const M_FL_FINAL_TEXTURE_SCALE_U: usize = 0x0; // CParticleCollectionRendererFloatInput
                pub const M_FL_FINAL_TEXTURE_SCALE_V: usize = 0x158; // CParticleCollectionRendererFloatInput
                pub const M_FL_FINAL_TEXTURE_OFFSET_U: usize = 0x2B0; // CParticleCollectionRendererFloatInput
                pub const M_FL_FINAL_TEXTURE_OFFSET_V: usize = 0x408; // CParticleCollectionRendererFloatInput
                pub const M_FL_FINAL_TEXTURE_UV_ROTATION: usize = 0x560; // CParticleCollectionRendererFloatInput
                pub const M_FL_ZOOM_SCALE: usize = 0x6B8; // CParticleCollectionRendererFloatInput
                pub const M_FL_DISTORTION: usize = 0x810; // CParticleCollectionRendererFloatInput
                pub const M_B_RANDOMIZE_OFFSETS: usize = 0x968; // bool
                pub const M_B_CLAMP_U_VS: usize = 0x969; // bool
                pub const M_N_PER_PARTICLE_BLEND: usize = 0x96C; // SpriteCardPerParticleScale_t
                pub const M_N_PER_PARTICLE_SCALE: usize = 0x970; // SpriteCardPerParticleScale_t
                pub const M_N_PER_PARTICLE_OFFSET_U: usize = 0x974; // SpriteCardPerParticleScale_t
                pub const M_N_PER_PARTICLE_OFFSET_V: usize = 0x978; // SpriteCardPerParticleScale_t
                pub const M_N_PER_PARTICLE_ROTATION: usize = 0x97C; // SpriteCardPerParticleScale_t
                pub const M_N_PER_PARTICLE_ZOOM: usize = 0x980; // SpriteCardPerParticleScale_t
                pub const M_N_PER_PARTICLE_DISTORTION: usize = 0x984; // SpriteCardPerParticleScale_t
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod texture_group_t {
                pub const M_B_ENABLED: usize = 0x0; // bool
                pub const M_B_REPLACE_TEXTURE_WITH_GRADIENT: usize = 0x1; // bool
                pub const M_H_TEXTURE: usize = 0x8; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_GRADIENT: usize = 0x10; // CColorGradient
                pub const M_N_TEXTURE_TYPE: usize = 0x28; // SpriteCardTextureType_t
                pub const M_N_TEXTURE_CHANNELS: usize = 0x2C; // SpriteCardTextureChannel_t
                pub const M_N_TEXTURE_BLEND_MODE: usize = 0x30; // ParticleTextureLayerBlendType_t
                pub const M_FL_TEXTURE_BLEND: usize = 0x38; // CParticleCollectionRendererFloatInput
                pub const M_TEXTURE_CONTROLS: usize = 0x190; // TextureControls_t
            }
            // Parent: c_particle_function_renderer
            // Fields count: 62
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_base_renderer_source2 {
                pub const M_FL_RADIUS_SCALE: usize = 0x200; // CParticleCollectionRendererFloatInput
                pub const M_FL_ALPHA_SCALE: usize = 0x358; // CParticleCollectionRendererFloatInput
                pub const M_FL_ROLL_SCALE: usize = 0x4B0; // CParticleCollectionRendererFloatInput
                pub const M_N_ALPHA2_FIELD: usize = 0x608; // ParticleAttributeIndex_t
                pub const M_VEC_COLOR_SCALE: usize = 0x610; // CParticleCollectionRendererVecInput
                pub const M_N_COLOR_BLEND_TYPE: usize = 0xC68; // ParticleColorBlendType_t
                pub const M_N_SHADER_TYPE: usize = 0xC6C; // SpriteCardShaderType_t
                pub const M_STR_SHADER_OVERRIDE: usize = 0xC70; // CUtlString
                pub const M_FL_CENTER_X_OFFSET: usize = 0xC78; // CParticleCollectionRendererFloatInput
                pub const M_FL_CENTER_Y_OFFSET: usize = 0xDD0; // CParticleCollectionRendererFloatInput
                pub const M_FL_BUMP_STRENGTH: usize = 0xF28; // float32
                pub const M_N_CROP_TEXTURE_OVERRIDE: usize = 0xF2C; // ParticleSequenceCropOverride_t
                pub const M_VEC_TEXTURES_INPUT: usize = 0xF30; // CUtlVector< TextureGroup_t >
                pub const M_FL_ANIMATION_RATE: usize = 0xF48; // float32
                pub const M_N_ANIMATION_TYPE: usize = 0xF4C; // AnimationType_t
                pub const M_B_ANIMATE_IN_FPS: usize = 0xF50; // bool
                pub const M_FL_SELF_ILLUM_AMOUNT: usize = 0xF58; // CParticleCollectionRendererFloatInput
                pub const M_FL_DIFFUSE_AMOUNT: usize = 0x10B0; // CParticleCollectionRendererFloatInput
                pub const M_FL_DIFFUSE_CLAMP: usize = 0x1208; // CParticleCollectionRendererFloatInput
                pub const M_N_LIGHTING_CONTROL_POINT: usize = 0x1360; // int32
                pub const M_N_SELF_ILLUM_PER_PARTICLE: usize = 0x1364; // ParticleAttributeIndex_t
                pub const M_N_OUTPUT_BLEND_MODE: usize = 0x1368; // ParticleOutputBlendMode_t
                pub const M_B_GAMMA_CORRECT_VERTEX_COLORS: usize = 0x136C; // bool
                pub const M_B_SATURATE_COLOR_PRE_ALPHA_BLEND: usize = 0x136D; // bool
                pub const M_FL_ADD_SELF_AMOUNT: usize = 0x1370; // CParticleCollectionRendererFloatInput
                pub const M_FL_DESATURATION: usize = 0x14C8; // CParticleCollectionRendererFloatInput
                pub const M_FL_OVERBRIGHT_FACTOR: usize = 0x1620; // CParticleCollectionRendererFloatInput
                pub const M_N_HSV_SHIFT_CONTROL_POINT: usize = 0x1778; // int32
                pub const M_N_FOG_TYPE: usize = 0x177C; // ParticleFogType_t
                pub const M_FL_FOG_AMOUNT: usize = 0x1780; // CParticleCollectionRendererFloatInput
                pub const M_B_TINT_BY_FOW: usize = 0x18D8; // bool
                pub const M_B_TINT_BY_GLOBAL_LIGHT: usize = 0x18D9; // bool
                pub const M_N_PER_PARTICLE_ALPHA_REFERENCE: usize = 0x18DC; // SpriteCardPerParticleScale_t
                pub const M_N_PER_PARTICLE_ALPHA_REF_WINDOW: usize = 0x18E0; // SpriteCardPerParticleScale_t
                pub const M_N_ALPHA_REFERENCE_TYPE: usize = 0x18E4; // ParticleAlphaReferenceType_t
                pub const M_FL_ALPHA_REFERENCE_SOFTNESS: usize = 0x18E8; // CParticleCollectionRendererFloatInput
                pub const M_FL_SOURCE_ALPHA_VALUE_TO_MAP_TO_ZERO: usize = 0x1A40; // CParticleCollectionRendererFloatInput
                pub const M_FL_SOURCE_ALPHA_VALUE_TO_MAP_TO_ONE: usize = 0x1B98; // CParticleCollectionRendererFloatInput
                pub const M_B_REFRACT: usize = 0x1CF0; // bool
                pub const M_B_REFRACT_SOLID: usize = 0x1CF1; // bool
                pub const M_FL_REFRACT_AMOUNT: usize = 0x1CF8; // CParticleCollectionRendererFloatInput
                pub const M_N_REFRACT_BLUR_RADIUS: usize = 0x1E50; // int32
                pub const M_N_REFRACT_BLUR_TYPE: usize = 0x1E54; // BlurFilterType_t
                pub const M_B_ONLY_RENDER_IN_EFFECTS_BLOOM_PASS: usize = 0x1E58; // bool
                pub const M_B_ONLY_RENDER_IN_EFFECTS_WATER_PASS: usize = 0x1E59; // bool
                pub const M_B_USE_MIXED_RESOLUTION_RENDERING: usize = 0x1E5A; // bool
                pub const M_B_ONLY_RENDER_IN_EFFECS_GAME_OVERLAY: usize = 0x1E5B; // bool
                pub const M_STENCIL_TEST_ID: usize = 0x1E5C; // char[128]
                pub const M_B_STENCIL_TEST_EXCLUDE: usize = 0x1EDC; // bool
                pub const M_STENCIL_WRITE_ID: usize = 0x1EDD; // char[128]
                pub const M_B_WRITE_STENCIL_ON_DEPTH_PASS: usize = 0x1F5D; // bool
                pub const M_B_WRITE_STENCIL_ON_DEPTH_FAIL: usize = 0x1F5E; // bool
                pub const M_B_REVERSE_Z_BUFFERING: usize = 0x1F5F; // bool
                pub const M_B_DISABLE_Z_BUFFERING: usize = 0x1F60; // bool
                pub const M_N_FEATHERING_MODE: usize = 0x1F64; // ParticleDepthFeatheringMode_t
                pub const M_FL_FEATHERING_MIN_DIST: usize = 0x1F68; // CParticleCollectionRendererFloatInput
                pub const M_FL_FEATHERING_MAX_DIST: usize = 0x20C0; // CParticleCollectionRendererFloatInput
                pub const M_FL_FEATHERING_FILTER: usize = 0x2218; // CParticleCollectionRendererFloatInput
                pub const M_FL_DEPTH_BIAS: usize = 0x2370; // CParticleCollectionRendererFloatInput
                pub const M_N_SORT_METHOD: usize = 0x24C8; // ParticleSortingChoiceList_t
                pub const M_B_BLEND_FRAMES_SEQ0: usize = 0x24CC; // bool
                pub const M_B_MAX_LUMINANCE_BLENDING_SEQUENCE0: usize = 0x24CD; // bool
            }
            // Parent: c_base_renderer_source2
            // Fields count: 27
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_render_sprites {
                pub const M_N_SEQUENCE_OVERRIDE: usize = 0x2710; // CParticleCollectionRendererFloatInput
                pub const M_N_ORIENTATION_TYPE: usize = 0x2868; // ParticleOrientationChoiceList_t
                pub const M_N_ORIENTATION_CONTROL_POINT: usize = 0x286C; // int32
                pub const M_B_USE_YAW_WITH_NORMAL_ALIGNED: usize = 0x2870; // bool
                pub const M_FL_MIN_SIZE: usize = 0x2878; // CParticleCollectionRendererFloatInput
                pub const M_FL_MAX_SIZE: usize = 0x29D0; // CParticleCollectionRendererFloatInput
                pub const M_FL_ALPHA_ADJUST_WITH_SIZE_ADJUST: usize = 0x2B28; // CParticleCollectionRendererFloatInput
                pub const M_FL_START_FADE_SIZE: usize = 0x2C80; // CParticleCollectionRendererFloatInput
                pub const M_FL_END_FADE_SIZE: usize = 0x2DD8; // CParticleCollectionRendererFloatInput
                pub const M_FL_START_FADE_DOT: usize = 0x2F30; // float32
                pub const M_FL_END_FADE_DOT: usize = 0x2F34; // float32
                pub const M_B_DISTANCE_ALPHA: usize = 0x2F38; // bool
                pub const M_B_SOFT_EDGES: usize = 0x2F39; // bool
                pub const M_FL_EDGE_SOFTNESS_START: usize = 0x2F3C; // float32
                pub const M_FL_EDGE_SOFTNESS_END: usize = 0x2F40; // float32
                pub const M_B_OUTLINE: usize = 0x2F44; // bool
                pub const M_OUTLINE_COLOR: usize = 0x2F45; // Color
                pub const M_N_OUTLINE_ALPHA: usize = 0x2F4C; // int32
                pub const M_FL_OUTLINE_START0: usize = 0x2F50; // float32
                pub const M_FL_OUTLINE_START1: usize = 0x2F54; // float32
                pub const M_FL_OUTLINE_END0: usize = 0x2F58; // float32
                pub const M_FL_OUTLINE_END1: usize = 0x2F5C; // float32
                pub const M_N_LIGHTING_MODE: usize = 0x2F60; // ParticleLightingQuality_t
                pub const M_FL_LIGHTING_TESSELLATION: usize = 0x2F68; // CParticleCollectionRendererFloatInput
                pub const M_FL_LIGHTING_DIRECTIONALITY: usize = 0x30C0; // CParticleCollectionRendererFloatInput
                pub const M_B_PARTICLE_SHADOWS: usize = 0x3218; // bool
                pub const M_FL_SHADOW_DENSITY: usize = 0x321C; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod float_input_material_variable_t {
                pub const M_STR_VARIABLE: usize = 0x0; // CUtlString
                pub const M_FL_INPUT: usize = 0x8; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod vec_input_material_variable_t {
                pub const M_STR_VARIABLE: usize = 0x0; // CUtlString
                pub const M_VEC_INPUT: usize = 0x8; // CParticleCollectionVecInput
            }
            // Parent: c_particle_function_renderer
            // Fields count: 22
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_render_cables {
                pub const M_FL_RADIUS_SCALE: usize = 0x200; // CParticleCollectionFloatInput
                pub const M_FL_ALPHA_SCALE: usize = 0x358; // CParticleCollectionFloatInput
                pub const M_VEC_COLOR_SCALE: usize = 0x4B0; // CParticleCollectionVecInput
                pub const M_N_COLOR_BLEND_TYPE: usize = 0xB08; // ParticleColorBlendType_t
                pub const M_H_MATERIAL: usize = 0xB10; // CStrongHandle< InfoForResourceTypeIMaterial2 >
                pub const M_N_TEXTURE_REPETITION_MODE: usize = 0xB18; // TextureRepetitionMode_t
                pub const M_FL_TEXTURE_REPEATS_PER_SEGMENT: usize = 0xB20; // CParticleCollectionFloatInput
                pub const M_FL_TEXTURE_REPEATS_CIRCUMFERENCE: usize = 0xC78; // CParticleCollectionFloatInput
                pub const M_FL_COLOR_MAP_OFFSET_V: usize = 0xDD0; // CParticleCollectionFloatInput
                pub const M_FL_COLOR_MAP_OFFSET_U: usize = 0xF28; // CParticleCollectionFloatInput
                pub const M_FL_NORMAL_MAP_OFFSET_V: usize = 0x1080; // CParticleCollectionFloatInput
                pub const M_FL_NORMAL_MAP_OFFSET_U: usize = 0x11D8; // CParticleCollectionFloatInput
                pub const M_B_DRAW_CABLE_CAPS: usize = 0x1330; // bool
                pub const M_FL_CAP_ROUNDNESS: usize = 0x1334; // float32
                pub const M_FL_CAP_OFFSET_AMOUNT: usize = 0x1338; // float32
                pub const M_FL_TESS_SCALE: usize = 0x133C; // float32
                pub const M_N_MIN_TESSELATION: usize = 0x1340; // int32
                pub const M_N_MAX_TESSELATION: usize = 0x1344; // int32
                pub const M_N_ROUNDNESS: usize = 0x1348; // int32
                pub const M_LIGHTING_TRANSFORM: usize = 0x1350; // CParticleTransformInput
                pub const M_MATERIAL_FLOAT_VARS: usize = 0x13B8; // CUtlVector< FloatInputMaterialVariable_t >
                pub const M_MATERIAL_VEC_VARS: usize = 0x13E8; // CUtlVector< VecInputMaterialVariable_t >
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_particle_input {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyCustomEditor
            pub mod c_per_particle_float_input {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyCustomEditor
            pub mod c_particle_collection_float_input {
            }
            // Parent: c_particle_collection_float_input
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyCustomEditor
            pub mod c_particle_collection_renderer_float_input {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyCustomEditor
            pub mod c_particle_remap_float_input {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyCustomEditor
            pub mod c_per_particle_vec_input {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyCustomEditor
            pub mod c_particle_collection_vec_input {
            }
            // Parent: c_particle_collection_vec_input
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyCustomEditor
            pub mod c_particle_collection_renderer_vec_input {
            }
            // Parent: None
            // Fields count: 1
            pub mod particle_ehandle {
                pub const UNUSED: usize = 0x0; // int32
            }
            // Parent: None
            // Fields count: 1
            pub mod particle_world_handle {
                pub const UNUSED: usize = 0x0; // int32
            }
            // Parent: None
            // Fields count: 0
            pub mod i_particle_effect {
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod particle_named_value_configuration_t {
                pub const M_CONFIG_NAME: usize = 0x0; // CUtlString
                pub const M_CONFIG_VALUE: usize = 0x8; // KeyValues3
                pub const M_I_ATTACH_TYPE: usize = 0x18; // ParticleAttachment_t
                pub const M_BOUND_ENTITY_PATH: usize = 0x20; // CUtlString
                pub const M_STR_ENTITY_SCOPE: usize = 0x28; // CUtlString
                pub const M_STR_ATTACHMENT_NAME: usize = 0x30; // CUtlString
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod particle_named_value_source_t {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_IS_PUBLIC: usize = 0x8; // bool
                pub const M_VALUE_TYPE: usize = 0xC; // PulseValueType_t
                pub const M_DEFAULT_CONFIG: usize = 0x10; // ParticleNamedValueConfiguration_t
                pub const M_NAMED_CONFIGS: usize = 0x48; // CUtlVector< ParticleNamedValueConfiguration_t >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyCustomEditor
            pub mod c_particle_variable_ref {
                pub const M_VARIABLE_NAME: usize = 0x0; // CKV3MemberNameWithStorage
                pub const M_VARIABLE_TYPE: usize = 0x38; // PulseValueType_t
            }
            // Parent: None
            // Fields count: 0
            pub mod c_particle_property {
            }
            // Parent: i_particle_effect
            // Fields count: 32
            pub mod c_new_particle_effect {
                pub const M_P_NEXT: usize = 0x10; // CNewParticleEffect*
                pub const M_P_PREV: usize = 0x18; // CNewParticleEffect*
                pub const M_P_PARTICLES: usize = 0x20; // IParticleCollection*
                pub const M_P_DEBUG_NAME: usize = 0x28; // char*
                pub const M_B_DONT_REMOVE: usize = 0x0; // bitfield:1
                pub const M_B_REMOVE: usize = 0x0; // bitfield:1
                pub const M_B_NEEDS_B_BOX_UPDATE: usize = 0x0; // bitfield:1
                pub const M_B_IS_FIRST_FRAME: usize = 0x0; // bitfield:1
                pub const M_B_AUTO_UPDATE_B_BOX: usize = 0x0; // bitfield:1
                pub const M_B_ALLOCATED: usize = 0x0; // bitfield:1
                pub const M_B_SIMULATE: usize = 0x0; // bitfield:1
                pub const M_B_SHOULD_PERFORM_CULL_CHECK: usize = 0x0; // bitfield:1
                pub const M_B_FORCE_NO_DRAW: usize = 0x0; // bitfield:1
                pub const M_B_SHOULD_SAVE: usize = 0x0; // bitfield:1
                pub const M_B_DISABLE_AGGREGATION: usize = 0x0; // bitfield:1
                pub const M_B_SHOULD_SIMULATE_DURING_GAME_PAUSED: usize = 0x0; // bitfield:1
                pub const M_B_SHOULD_CHECK_FO_W: usize = 0x0; // bitfield:1
                pub const M_V_SORT_ORIGIN: usize = 0x40; // 
                pub const M_FL_SCALE: usize = 0x4C; // float32
                pub const M_H_OWNER: usize = 0x50; // PARTICLE_EHANDLE__*
                pub const M_P_OWNING_PARTICLE_PROPERTY: usize = 0x58; // CParticleProperty*
                pub const M_FL_FREEZE_TRANSITION_START: usize = 0x70; // float32
                pub const M_FL_FREEZE_TRANSITION_DURATION: usize = 0x74; // float32
                pub const M_FL_FREEZE_TRANSITION_OVERRIDE: usize = 0x78; // float32
                pub const M_B_FREEZE_TRANSITION_ACTIVE: usize = 0x7C; // bool
                pub const M_B_FREEZE_TARGET_STATE: usize = 0x7D; // bool
                pub const M_B_CAN_FREEZE: usize = 0x7E; // bool
                pub const M_LAST_MIN: usize = 0x80; // 
                pub const M_LAST_MAX: usize = 0x8C; // 
                pub const M_N_SPLIT_SCREEN_USER: usize = 0x98; // CSplitScreenSlot
                pub const M_VEC_AGGREGATION_CENTER: usize = 0x9C; // 
                pub const M_REF_COUNT: usize = 0xC0; // int32
            }
            // Parent: c_particle_function_constraint
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_constrain_distance {
                pub const M_F_MIN_DISTANCE: usize = 0x1C0; // CParticleCollectionFloatInput
                pub const M_F_MAX_DISTANCE: usize = 0x318; // CParticleCollectionFloatInput
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x470; // int32
                pub const M_CENTER_OFFSET: usize = 0x474; // 
                pub const M_B_GLOBAL_CENTER: usize = 0x480; // bool
            }
            // Parent: c_particle_function_constraint
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_collide_with_self {
                pub const M_FL_RADIUS_SCALE: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_FL_MINIMUM_SPEED: usize = 0x318; // CPerParticleFloatInput
            }
            // Parent: c_particle_function_constraint
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_collide_with_parent_particles {
                pub const M_FL_PARENT_RADIUS_SCALE: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_FL_RADIUS_SCALE: usize = 0x318; // CPerParticleFloatInput
            }
            // Parent: c_particle_function_constraint
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_constrain_distance_to_path {
                pub const M_F_MIN_DISTANCE: usize = 0x1C0; // float32
                pub const M_FL_MAX_DISTANCE0: usize = 0x1C4; // float32
                pub const M_FL_MAX_DISTANCE_MID: usize = 0x1C8; // float32
                pub const M_FL_MAX_DISTANCE1: usize = 0x1CC; // float32
                pub const M_PATH_PARAMETERS: usize = 0x1D0; // CPathParameters
                pub const M_FL_TRAVEL_TIME: usize = 0x210; // float32
                pub const M_N_FIELD_SCALE: usize = 0x214; // ParticleAttributeIndex_t
                pub const M_N_MANUAL_T_FIELD: usize = 0x218; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_constraint
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_constrain_distance_to_user_specified_path {
                pub const M_F_MIN_DISTANCE: usize = 0x1C0; // float32
                pub const M_FL_MAX_DISTANCE: usize = 0x1C4; // float32
                pub const M_FL_TIME_SCALE: usize = 0x1C8; // float32
                pub const M_B_LOOPED_PATH: usize = 0x1CC; // bool
                pub const M_POINT_LIST: usize = 0x1D0; // CUtlVector< PointDefinitionWithTimeValues_t >
            }
            // Parent: c_particle_function_constraint
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_planar_constraint {
                pub const M_POINT_ON_PLANE: usize = 0x1C0; // 
                pub const M_PLANE_NORMAL: usize = 0x1CC; // 
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1D8; // int32
                pub const M_B_GLOBAL_ORIGIN: usize = 0x1DC; // bool
                pub const M_B_GLOBAL_NORMAL: usize = 0x1DD; // bool
                pub const M_FL_RADIUS_SCALE: usize = 0x1E0; // CPerParticleFloatInput
                pub const M_FL_MAXIMUM_DISTANCE_TO_CP: usize = 0x338; // CParticleCollectionFloatInput
                pub const M_B_USE_OLD_CODE: usize = 0x490; // bool
            }
            // Parent: c_particle_function_constraint
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_world_collide_constraint {
            }
            // Parent: c_particle_function_constraint
            // Fields count: 27
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_world_trace_constraint {
                pub const M_N_CP: usize = 0x1C0; // int32
                pub const M_VEC_CP_OFFSET: usize = 0x1C4; // 
                pub const M_N_COLLISION_MODE: usize = 0x1D0; // ParticleCollisionMode_t
                pub const M_N_COLLISION_MODE_MIN: usize = 0x1D4; // ParticleCollisionMode_t
                pub const M_N_TRACE_SET: usize = 0x1D8; // ParticleTraceSet_t
                pub const M_COLLISION_GROUP_NAME: usize = 0x1DC; // char[128]
                pub const M_B_WORLD_ONLY: usize = 0x25C; // bool
                pub const M_B_BRUSH_ONLY: usize = 0x25D; // bool
                pub const M_B_INCLUDE_WATER: usize = 0x25E; // bool
                pub const M_N_IGNORE_CP: usize = 0x260; // int32
                pub const M_FL_CP_MOVEMENT_TOLERANCE: usize = 0x264; // float32
                pub const M_FL_RETEST_RATE: usize = 0x268; // float32
                pub const M_FL_TRACE_TOLERANCE: usize = 0x26C; // float32
                pub const M_FL_COLLISION_CONFIRMATION_SPEED: usize = 0x270; // float32
                pub const M_N_MAX_TRACES_PER_FRAME: usize = 0x274; // float32
                pub const M_FL_RADIUS_SCALE: usize = 0x278; // CPerParticleFloatInput
                pub const M_FL_BOUNCE_AMOUNT: usize = 0x3D0; // CPerParticleFloatInput
                pub const M_FL_SLIDE_AMOUNT: usize = 0x528; // CPerParticleFloatInput
                pub const M_FL_RANDOM_DIR_SCALE: usize = 0x680; // CPerParticleFloatInput
                pub const M_B_DECAY_BOUNCE: usize = 0x7D8; // bool
                pub const M_B_KILLON_CONTACT: usize = 0x7D9; // bool
                pub const M_FL_MIN_SPEED: usize = 0x7DC; // float32
                pub const M_B_SET_NORMAL: usize = 0x7E0; // bool
                pub const M_N_STICK_ON_COLLISION_FIELD: usize = 0x7E4; // ParticleAttributeIndex_t
                pub const M_FL_STOP_SPEED: usize = 0x7E8; // CPerParticleFloatInput
                pub const M_N_ENTITY_STICK_DATA_FIELD: usize = 0x940; // ParticleAttributeIndex_t
                pub const M_N_ENTITY_STICK_NORMAL_FIELD: usize = 0x944; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_constraint
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_box_constraint {
                pub const M_VEC_MIN: usize = 0x1C0; // CParticleCollectionVecInput
                pub const M_VEC_MAX: usize = 0x818; // CParticleCollectionVecInput
                pub const M_N_CP: usize = 0xE70; // int32
                pub const M_B_LOCAL_SPACE: usize = 0xE74; // bool
                pub const M_B_ACCOUNT_FOR_RADIUS: usize = 0xE75; // bool
            }
            // Parent: c_particle_function_constraint
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_shape_matching_constraint {
                pub const M_FL_SHAPE_RESTORATION_TIME: usize = 0x1C0; // float32
            }
            // Parent: c_particle_function_constraint
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_rope_spring_constraint {
                pub const M_FL_REST_LENGTH: usize = 0x1C0; // CParticleCollectionFloatInput
                pub const M_FL_MIN_DISTANCE: usize = 0x318; // CParticleCollectionFloatInput
                pub const M_FL_MAX_DISTANCE: usize = 0x470; // CParticleCollectionFloatInput
                pub const M_FL_ADJUSTMENT_SCALE: usize = 0x5C8; // float32
                pub const M_FL_INITIAL_RESTING_LENGTH: usize = 0x5D0; // CParticleCollectionFloatInput
            }
            // Parent: c_particle_function_constraint
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_spring_to_vector_constraint {
                pub const M_FL_REST_LENGTH: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_FL_MIN_DISTANCE: usize = 0x318; // CPerParticleFloatInput
                pub const M_FL_MAX_DISTANCE: usize = 0x470; // CPerParticleFloatInput
                pub const M_FL_RESTING_LENGTH: usize = 0x5C8; // CPerParticleFloatInput
                pub const M_VEC_ANCHOR_VECTOR: usize = 0x720; // CPerParticleVecInput
            }
            // Parent: c_particle_function_constraint
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_constrain_line_length {
                pub const M_FL_MIN_DISTANCE: usize = 0x1C0; // float32
                pub const M_FL_MAX_DISTANCE: usize = 0x1C4; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_ring_wave {
                pub const M_TRANSFORM_INPUT: usize = 0x1C0; // CParticleTransformInput
                pub const M_FL_PARTICLES_PER_ORBIT: usize = 0x228; // CParticleCollectionFloatInput
                pub const M_FL_INITIAL_RADIUS: usize = 0x380; // CPerParticleFloatInput
                pub const M_FL_THICKNESS: usize = 0x4D8; // CPerParticleFloatInput
                pub const M_FL_INITIAL_SPEED_MIN: usize = 0x630; // CPerParticleFloatInput
                pub const M_FL_INITIAL_SPEED_MAX: usize = 0x788; // CPerParticleFloatInput
                pub const M_FL_ROLL: usize = 0x8E0; // CPerParticleFloatInput
                pub const M_FL_PITCH: usize = 0xA38; // CPerParticleFloatInput
                pub const M_FL_YAW: usize = 0xB90; // CPerParticleFloatInput
                pub const M_B_EVEN_DISTRIBUTION: usize = 0xCE8; // bool
                pub const M_B_XY_VELOCITY_ONLY: usize = 0xCE9; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_spiral_sphere {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_N_OVERRIDE_CP: usize = 0x1C4; // int32
                pub const M_N_DENSITY: usize = 0x1C8; // int32
                pub const M_FL_INITIAL_RADIUS: usize = 0x1CC; // float32
                pub const M_FL_INITIAL_SPEED_MIN: usize = 0x1D0; // float32
                pub const M_FL_INITIAL_SPEED_MAX: usize = 0x1D4; // float32
                pub const M_B_USE_PARTICLE_COUNT: usize = 0x1D8; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_in_epitrochoid {
                pub const M_N_COMPONENT1: usize = 0x1C0; // int32
                pub const M_N_COMPONENT2: usize = 0x1C4; // int32
                pub const M_TRANSFORM_INPUT: usize = 0x1C8; // CParticleTransformInput
                pub const M_FL_PARTICLE_DENSITY: usize = 0x230; // CPerParticleFloatInput
                pub const M_FL_OFFSET: usize = 0x388; // CPerParticleFloatInput
                pub const M_FL_RADIUS1: usize = 0x4E0; // CPerParticleFloatInput
                pub const M_FL_RADIUS2: usize = 0x638; // CPerParticleFloatInput
                pub const M_B_USE_COUNT: usize = 0x790; // bool
                pub const M_B_USE_LOCAL_COORDS: usize = 0x791; // bool
                pub const M_B_OFFSET_EXISTING_POS: usize = 0x792; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_phyllotaxis {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_N_SCALE_CP: usize = 0x1C4; // int32
                pub const M_N_COMPONENT: usize = 0x1C8; // int32
                pub const M_F_RAD_CENT_CORE: usize = 0x1CC; // float32
                pub const M_F_RAD_PER_POINT: usize = 0x1D0; // float32
                pub const M_F_RAD_PER_POINT_TO: usize = 0x1D4; // float32
                pub const M_FPOINT_ANGLE: usize = 0x1D8; // float32
                pub const M_FSIZE_OVERALL: usize = 0x1DC; // float32
                pub const M_F_RAD_BIAS: usize = 0x1E0; // float32
                pub const M_F_MIN_RAD: usize = 0x1E4; // float32
                pub const M_F_DIST_BIAS: usize = 0x1E8; // float32
                pub const M_B_USE_LOCAL_COORDS: usize = 0x1EC; // bool
                pub const M_B_USE_WITH_CONT_EMIT: usize = 0x1ED; // bool
                pub const M_B_USE_ORIG_RADIUS: usize = 0x1EE; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_on_model {
                pub const M_MODEL_INPUT: usize = 0x1C0; // CParticleModelInput
                pub const M_TRANSFORM_INPUT: usize = 0x220; // CParticleTransformInput
                pub const M_N_FORCE_IN_MODEL: usize = 0x288; // int32
                pub const M_N_DESIRED_HITBOX: usize = 0x28C; // int32
                pub const M_N_HITBOX_VALUE_FROM_CONTROL_POINT_INDEX: usize = 0x290; // int32
                pub const M_VEC_HIT_BOX_SCALE: usize = 0x298; // CParticleCollectionVecInput
                pub const M_FL_BONE_VELOCITY: usize = 0x8F0; // float32
                pub const M_FL_MAX_BONE_VELOCITY: usize = 0x8F4; // float32
                pub const M_VEC_DIRECTION_BIAS: usize = 0x8F8; // CParticleCollectionVecInput
                pub const M_HITBOX_SET_NAME: usize = 0xF50; // char[128]
                pub const M_B_LOCAL_COORDS: usize = 0xFD0; // bool
                pub const M_B_USE_BONES: usize = 0xFD1; // bool
                pub const M_B_USE_MESH: usize = 0xFD2; // bool
                pub const M_FL_SHELL_SIZE: usize = 0xFD8; // CParticleCollectionFloatInput
            }
            // Parent: c_particle_function_initializer
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_on_model_at_height {
                pub const M_B_USE_BONES: usize = 0x1C0; // bool
                pub const M_B_FORCE_Z: usize = 0x1C1; // bool
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C4; // int32
                pub const M_N_HEIGHT_CP: usize = 0x1C8; // int32
                pub const M_B_USE_WATER_HEIGHT: usize = 0x1CC; // bool
                pub const M_FL_DESIRED_HEIGHT: usize = 0x1D0; // CParticleCollectionFloatInput
                pub const M_VEC_HIT_BOX_SCALE: usize = 0x328; // CParticleCollectionVecInput
                pub const M_VEC_DIRECTION_BIAS: usize = 0x980; // CParticleCollectionVecInput
                pub const M_N_BIAS_TYPE: usize = 0xFD8; // ParticleHitboxBiasType_t
                pub const M_B_LOCAL_COORDS: usize = 0xFDC; // bool
                pub const M_B_PREFER_MOVING_BOXES: usize = 0xFDD; // bool
                pub const M_HITBOX_SET_NAME: usize = 0xFDE; // char[128]
                pub const M_FL_HITBOX_VELOCITY_SCALE: usize = 0x1060; // CParticleCollectionFloatInput
                pub const M_FL_MAX_BONE_VELOCITY: usize = 0x11B8; // CParticleCollectionFloatInput
            }
            // Parent: c_particle_function_initializer
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_set_hitbox_to_closest {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_N_DESIRED_HITBOX: usize = 0x1C4; // int32
                pub const M_VEC_HIT_BOX_SCALE: usize = 0x1C8; // CParticleCollectionVecInput
                pub const M_HITBOX_SET_NAME: usize = 0x820; // char[128]
                pub const M_B_USE_BONES: usize = 0x8A0; // bool
                pub const M_B_USE_CLOSEST_POINT_ON_HITBOX: usize = 0x8A1; // bool
                pub const M_N_TEST_TYPE: usize = 0x8A4; // ClosestPointTestType_t
                pub const M_FL_HYBRID_RATIO: usize = 0x8A8; // CParticleCollectionFloatInput
                pub const M_B_UPDATE_POSITION: usize = 0xA00; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_set_hitbox_to_model {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_N_FORCE_IN_MODEL: usize = 0x1C4; // int32
                pub const M_N_DESIRED_HITBOX: usize = 0x1C8; // int32
                pub const M_VEC_HIT_BOX_SCALE: usize = 0x1D0; // CParticleCollectionVecInput
                pub const M_VEC_DIRECTION_BIAS: usize = 0x828; // 
                pub const M_B_MAINTAIN_HITBOX: usize = 0x834; // bool
                pub const M_B_USE_BONES: usize = 0x835; // bool
                pub const M_HITBOX_SET_NAME: usize = 0x836; // char[128]
                pub const M_FL_SHELL_SIZE: usize = 0x8B8; // CParticleCollectionFloatInput
            }
            // Parent: c_particle_function_initializer
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_within_sphere_transform {
                pub const M_F_RADIUS_MIN: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_F_RADIUS_MAX: usize = 0x318; // CPerParticleFloatInput
                pub const M_VEC_DISTANCE_BIAS: usize = 0x470; // CPerParticleVecInput
                pub const M_VEC_DISTANCE_BIAS_ABS: usize = 0xAC8; // 
                pub const M_TRANSFORM_INPUT: usize = 0xAD8; // CParticleTransformInput
                pub const M_F_SPEED_MIN: usize = 0xB40; // CPerParticleFloatInput
                pub const M_F_SPEED_MAX: usize = 0xC98; // CPerParticleFloatInput
                pub const M_F_SPEED_RAND_EXP: usize = 0xDF0; // float32
                pub const M_B_LOCAL_COORDS: usize = 0xDF4; // bool
                pub const M_FL_END_CP_GROWTH_TIME: usize = 0xDF8; // float32
                pub const M_LOCAL_COORDINATE_SYSTEM_SPEED_MIN: usize = 0xE00; // CPerParticleVecInput
                pub const M_LOCAL_COORDINATE_SYSTEM_SPEED_MAX: usize = 0x1458; // CPerParticleVecInput
                pub const M_N_FIELD_OUTPUT: usize = 0x1AB0; // ParticleAttributeIndex_t
                pub const M_N_FIELD_VELOCITY: usize = 0x1AB4; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_within_box {
                pub const M_VEC_MIN: usize = 0x1C0; // CPerParticleVecInput
                pub const M_VEC_MAX: usize = 0x818; // CPerParticleVecInput
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0xE70; // int32
                pub const M_B_LOCAL_SPACE: usize = 0xE74; // bool
                pub const M_RANDOMNESS_PARAMETERS: usize = 0xE78; // CRandomNumberGeneratorParameters
            }
            // Parent: c_particle_function_initializer
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_on_grid {
                pub const M_N_X_COUNT: usize = 0x1C0; // CParticleCollectionFloatInput
                pub const M_N_Y_COUNT: usize = 0x318; // CParticleCollectionFloatInput
                pub const M_N_Z_COUNT: usize = 0x470; // CParticleCollectionFloatInput
                pub const M_N_X_SPACING: usize = 0x5C8; // CParticleCollectionFloatInput
                pub const M_N_Y_SPACING: usize = 0x720; // CParticleCollectionFloatInput
                pub const M_N_Z_SPACING: usize = 0x878; // CParticleCollectionFloatInput
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x9D0; // int32
                pub const M_B_LOCAL_SPACE: usize = 0x9D4; // bool
                pub const M_B_CENTER: usize = 0x9D5; // bool
                pub const M_B_HOLLOW: usize = 0x9D6; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_position_offset {
                pub const M_OFFSET_MIN: usize = 0x1C0; // CPerParticleVecInput
                pub const M_OFFSET_MAX: usize = 0x818; // CPerParticleVecInput
                pub const M_TRANSFORM_INPUT: usize = 0xE70; // CParticleTransformInput
                pub const M_B_LOCAL_COORDS: usize = 0xED8; // bool
                pub const M_B_PROPORTIONAL: usize = 0xED9; // bool
                pub const M_RANDOMNESS_PARAMETERS: usize = 0xEDC; // CRandomNumberGeneratorParameters
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_position_offset_to_cp {
                pub const M_N_CONTROL_POINT_NUMBER_START: usize = 0x1C0; // int32
                pub const M_N_CONTROL_POINT_NUMBER_END: usize = 0x1C4; // int32
                pub const M_B_LOCAL_COORDS: usize = 0x1C8; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_position_place_on_ground {
                pub const M_FL_OFFSET: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_FL_MAX_TRACE_LENGTH: usize = 0x318; // CPerParticleFloatInput
                pub const M_COLLISION_GROUP_NAME: usize = 0x470; // char[128]
                pub const M_N_TRACE_SET: usize = 0x4F0; // ParticleTraceSet_t
                pub const M_N_TRACE_MISS_BEHAVIOR: usize = 0x500; // ParticleTraceMissBehavior_t
                pub const M_B_INCLUDE_WATER: usize = 0x504; // bool
                pub const M_B_SET_NORMAL: usize = 0x505; // bool
                pub const M_B_SET_PXYZ_ONLY: usize = 0x506; // bool
                pub const M_B_TRACE_ALONG_NORMAL: usize = 0x507; // bool
                pub const M_B_OFFSETON_COL_ONLY: usize = 0x508; // bool
                pub const M_FL_OFFSET_BY_RADIUS_FACTOR: usize = 0x50C; // float32
                pub const M_N_PRESERVE_OFFSET_CP: usize = 0x510; // int32
                pub const M_N_IGNORE_CP: usize = 0x514; // int32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_velocity_from_normal {
                pub const M_F_SPEED_MIN: usize = 0x1C0; // float32
                pub const M_F_SPEED_MAX: usize = 0x1C4; // float32
                pub const M_B_IGNORE_DT: usize = 0x1C8; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_velocity_random {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_F_SPEED_MIN: usize = 0x1C8; // CPerParticleFloatInput
                pub const M_F_SPEED_MAX: usize = 0x320; // CPerParticleFloatInput
                pub const M_LOCAL_COORDINATE_SYSTEM_SPEED_MIN: usize = 0x478; // CPerParticleVecInput
                pub const M_LOCAL_COORDINATE_SYSTEM_SPEED_MAX: usize = 0xAD0; // CPerParticleVecInput
                pub const M_B_IGNORE_DT: usize = 0x1128; // bool
                pub const M_RANDOMNESS_PARAMETERS: usize = 0x112C; // CRandomNumberGeneratorParameters
            }
            // Parent: c_particle_function_initializer
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_initial_velocity_noise {
                pub const M_VEC_ABS_VAL: usize = 0x1C0; // 
                pub const M_VEC_ABS_VAL_INV: usize = 0x1CC; // 
                pub const M_VEC_OFFSET_LOC: usize = 0x1D8; // CPerParticleVecInput
                pub const M_FL_OFFSET: usize = 0x830; // CPerParticleFloatInput
                pub const M_VEC_OUTPUT_MIN: usize = 0x988; // CPerParticleVecInput
                pub const M_VEC_OUTPUT_MAX: usize = 0xFE0; // CPerParticleVecInput
                pub const M_FL_NOISE_SCALE: usize = 0x1638; // CPerParticleFloatInput
                pub const M_FL_NOISE_SCALE_LOC: usize = 0x1790; // CPerParticleFloatInput
                pub const M_TRANSFORM_INPUT: usize = 0x18E8; // CParticleTransformInput
                pub const M_B_IGNORE_DT: usize = 0x1950; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_initial_velocity_from_hitbox {
                pub const M_FL_VELOCITY_MIN: usize = 0x1C0; // float32
                pub const M_FL_VELOCITY_MAX: usize = 0x1C4; // float32
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C8; // int32
                pub const M_HITBOX_SET_NAME: usize = 0x1CC; // char[128]
                pub const M_B_USE_BONES: usize = 0x24C; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_velocity_radial_random {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_F_SPEED_MIN: usize = 0x1C4; // float32
                pub const M_F_SPEED_MAX: usize = 0x1C8; // float32
                pub const M_VEC_LOCAL_COORDINATE_SYSTEM_SPEED_SCALE: usize = 0x1CC; // 
                pub const M_B_IGNORE_DELTA: usize = 0x1D9; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_life_time {
                pub const M_F_LIFETIME_MIN: usize = 0x1C0; // float32
                pub const M_F_LIFETIME_MAX: usize = 0x1C4; // float32
                pub const M_F_LIFETIME_RAND_EXPONENT: usize = 0x1C8; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_scalar {
                pub const M_FL_MIN: usize = 0x1C0; // float32
                pub const M_FL_MAX: usize = 0x1C4; // float32
                pub const M_FL_EXPONENT: usize = 0x1C8; // float32
                pub const M_N_FIELD_OUTPUT: usize = 0x1CC; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_vector {
                pub const M_VEC_MIN: usize = 0x1C0; // 
                pub const M_VEC_MAX: usize = 0x1CC; // 
                pub const M_N_FIELD_OUTPUT: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const M_RANDOMNESS_PARAMETERS: usize = 0x1DC; // CRandomNumberGeneratorParameters
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_vector_component {
                pub const M_FL_MIN: usize = 0x1C0; // float32
                pub const M_FL_MAX: usize = 0x1C4; // float32
                pub const M_N_FIELD_OUTPUT: usize = 0x1C8; // ParticleAttributeIndex_t
                pub const M_N_COMPONENT: usize = 0x1CC; // int32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_add_vector_to_vector {
                pub const M_VEC_SCALE: usize = 0x1C0; // 
                pub const M_N_FIELD_OUTPUT: usize = 0x1CC; // ParticleAttributeIndex_t
                pub const M_N_FIELD_INPUT: usize = 0x1D0; // ParticleAttributeIndex_t
                pub const M_V_OFFSET_MIN: usize = 0x1D4; // 
                pub const M_V_OFFSET_MAX: usize = 0x1E0; // 
                pub const M_RANDOMNESS_PARAMETERS: usize = 0x1EC; // CRandomNumberGeneratorParameters
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_alpha_window_threshold {
                pub const M_FL_MIN: usize = 0x1C0; // float32
                pub const M_FL_MAX: usize = 0x1C4; // float32
                pub const M_FL_EXPONENT: usize = 0x1C8; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_radius {
                pub const M_FL_RADIUS_MIN: usize = 0x1C0; // float32
                pub const M_FL_RADIUS_MAX: usize = 0x1C4; // float32
                pub const M_FL_RADIUS_RAND_EXPONENT: usize = 0x1C8; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_alpha {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_ALPHA_MIN: usize = 0x1C4; // int32
                pub const M_N_ALPHA_MAX: usize = 0x1C8; // int32
                pub const M_FL_ALPHA_RAND_EXPONENT: usize = 0x1D4; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_general_random_rotation {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_FL_DEGREES: usize = 0x1C4; // float32
                pub const M_FL_DEGREES_MIN: usize = 0x1C8; // float32
                pub const M_FL_DEGREES_MAX: usize = 0x1CC; // float32
                pub const M_FL_ROTATION_RAND_EXPONENT: usize = 0x1D0; // float32
                pub const M_B_RANDOMLY_FLIP_DIRECTION: usize = 0x1D4; // bool
            }
            // Parent: c_general_random_rotation
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_rotation {
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_orient2_d_rel_to_cp {
                pub const M_N_CP: usize = 0x1C0; // int32
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_FL_ROT_OFFSET: usize = 0x1C8; // float32
            }
            // Parent: c_general_random_rotation
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_rotation_speed {
            }
            // Parent: c_general_random_rotation
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_yaw {
            }
            // Parent: c_particle_function_initializer
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_color {
                pub const M_COLOR_MIN: usize = 0x1DC; // Color
                pub const M_COLOR_MAX: usize = 0x1E0; // Color
                pub const M_TINT_MIN: usize = 0x1E4; // Color
                pub const M_TINT_MAX: usize = 0x1E8; // Color
                pub const M_FL_TINT_PERC: usize = 0x1EC; // float32
                pub const M_FL_UPDATE_THRESHOLD: usize = 0x1F0; // float32
                pub const M_N_TINT_CP: usize = 0x1F4; // int32
                pub const M_N_FIELD_OUTPUT: usize = 0x1F8; // ParticleAttributeIndex_t
                pub const M_N_TINT_BLEND_MODE: usize = 0x1FC; // ParticleColorBlendMode_t
                pub const M_FL_LIGHT_AMPLIFICATION: usize = 0x200; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_color_lit_per_particle {
                pub const M_COLOR_MIN: usize = 0x1D8; // Color
                pub const M_COLOR_MAX: usize = 0x1DC; // Color
                pub const M_TINT_MIN: usize = 0x1E0; // Color
                pub const M_TINT_MAX: usize = 0x1E4; // Color
                pub const M_FL_TINT_PERC: usize = 0x1E8; // float32
                pub const M_N_TINT_BLEND_MODE: usize = 0x1EC; // ParticleColorBlendMode_t
                pub const M_FL_LIGHT_AMPLIFICATION: usize = 0x1F0; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_trail_length {
                pub const M_FL_MIN_LENGTH: usize = 0x1C0; // float32
                pub const M_FL_MAX_LENGTH: usize = 0x1C4; // float32
                pub const M_FL_LENGTH_RAND_EXPONENT: usize = 0x1C8; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_sequence {
                pub const M_N_SEQUENCE_MIN: usize = 0x1C0; // int32
                pub const M_N_SEQUENCE_MAX: usize = 0x1C4; // int32
                pub const M_B_SHUFFLE: usize = 0x1C8; // bool
                pub const M_B_LINEAR: usize = 0x1C9; // bool
                pub const M_WEIGHTED_LIST: usize = 0x1D0; // CUtlVector< SequenceWeightedList_t >
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_sequence_from_cp {
                pub const M_B_KILL_UNUSED: usize = 0x1C0; // bool
                pub const M_B_RADIUS_SCALE: usize = 0x1C1; // bool
                pub const M_N_CP: usize = 0x1C4; // int32
                pub const M_VEC_OFFSET: usize = 0x1C8; // 
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_model_sequence {
                pub const M_ACTIVITY_NAME: usize = 0x1C0; // char[256]
                pub const M_SEQUENCE_NAME: usize = 0x2C0; // char[256]
                pub const M_H_MODEL: usize = 0x3C0; // CStrongHandle< InfoForResourceTypeCModel >
            }
            // Parent: c_particle_function_initializer
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_scale_velocity {
                pub const M_VEC_SCALE: usize = 0x1C0; // CParticleCollectionVecInput
            }
            // Parent: c_particle_function_initializer
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_position_warp {
                pub const M_VEC_WARP_MIN: usize = 0x1C0; // CParticleCollectionVecInput
                pub const M_VEC_WARP_MAX: usize = 0x818; // CParticleCollectionVecInput
                pub const M_N_SCALE_CONTROL_POINT_NUMBER: usize = 0xE70; // int32
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0xE74; // int32
                pub const M_N_RADIUS_COMPONENT: usize = 0xE78; // int32
                pub const M_FL_WARP_TIME: usize = 0xE7C; // float32
                pub const M_FL_WARP_START_TIME: usize = 0xE80; // float32
                pub const M_FL_PREV_POS_SCALE: usize = 0xE84; // float32
                pub const M_B_INVERT_WARP: usize = 0xE88; // bool
                pub const M_B_USE_COUNT: usize = 0xE89; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_position_warp_scalar {
                pub const M_VEC_WARP_MIN: usize = 0x1C0; // 
                pub const M_VEC_WARP_MAX: usize = 0x1CC; // 
                pub const M_INPUT_VALUE: usize = 0x1D8; // CPerParticleFloatInput
                pub const M_FL_PREV_POS_SCALE: usize = 0x330; // float32
                pub const M_N_SCALE_CONTROL_POINT_NUMBER: usize = 0x334; // int32
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x338; // int32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_creation_noise {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_B_ABS_VAL: usize = 0x1C4; // bool
                pub const M_B_ABS_VAL_INV: usize = 0x1C5; // bool
                pub const M_FL_OFFSET: usize = 0x1C8; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1CC; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D0; // float32
                pub const M_FL_NOISE_SCALE: usize = 0x1D4; // float32
                pub const M_FL_NOISE_SCALE_LOC: usize = 0x1D8; // float32
                pub const M_VEC_OFFSET_LOC: usize = 0x1DC; // 
                pub const M_FL_WORLD_TIME_SCALE: usize = 0x1E8; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_along_path {
                pub const M_F_MAX_DISTANCE: usize = 0x1C0; // float32
                pub const M_PATH_PARAMS: usize = 0x1D0; // CPathParameters
                pub const M_B_USE_RANDOM_C_PS: usize = 0x210; // bool
                pub const M_V_END_OFFSET: usize = 0x214; // 
                pub const M_B_SAVE_OFFSET: usize = 0x220; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_move_between_points {
                pub const M_FL_SPEED_MIN: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_FL_SPEED_MAX: usize = 0x318; // CPerParticleFloatInput
                pub const M_FL_END_SPREAD: usize = 0x470; // CPerParticleFloatInput
                pub const M_FL_START_OFFSET: usize = 0x5C8; // CPerParticleFloatInput
                pub const M_FL_END_OFFSET: usize = 0x720; // CPerParticleFloatInput
                pub const M_N_END_CONTROL_POINT_NUMBER: usize = 0x878; // int32
                pub const M_B_TRAIL_BIAS: usize = 0x87C; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_scalar {
                pub const M_N_FIELD_INPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1C8; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1CC; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D0; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D4; // float32
                pub const M_FL_START_TIME: usize = 0x1D8; // float32
                pub const M_FL_END_TIME: usize = 0x1DC; // float32
                pub const M_N_SET_METHOD: usize = 0x1E0; // ParticleSetMethod_t
                pub const M_B_ACTIVE_RANGE: usize = 0x1E4; // bool
                pub const M_FL_REMAP_BIAS: usize = 0x1E8; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_particle_count_to_scalar {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_INPUT_MIN: usize = 0x1C4; // int32
                pub const M_N_INPUT_MAX: usize = 0x1C8; // int32
                pub const M_N_SCALE_CONTROL_POINT: usize = 0x1CC; // int32
                pub const M_N_SCALE_CONTROL_POINT_FIELD: usize = 0x1D0; // int32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D4; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D8; // float32
                pub const M_N_SET_METHOD: usize = 0x1DC; // ParticleSetMethod_t
                pub const M_B_ACTIVE_RANGE: usize = 0x1E0; // bool
                pub const M_B_INVERT: usize = 0x1E1; // bool
                pub const M_B_WRAP: usize = 0x1E2; // bool
                pub const M_FL_REMAP_BIAS: usize = 0x1E4; // float32
            }
            // Parent: c_init_remap_particle_count_to_scalar
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_particle_count_to_named_model_element_scalar {
                pub const M_H_MODEL: usize = 0x1F0; // CStrongHandle< InfoForResourceTypeCModel >
                pub const M_OUTPUT_MIN_NAME: usize = 0x1F8; // CUtlString
                pub const M_OUTPUT_MAX_NAME: usize = 0x200; // CUtlString
                pub const M_B_MODEL_FROM_RENDERER: usize = 0x208; // bool
            }
            // Parent: c_init_remap_particle_count_to_named_model_element_scalar
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_particle_count_to_named_model_sequence_scalar {
            }
            // Parent: c_init_remap_particle_count_to_named_model_element_scalar
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_particle_count_to_named_model_body_part_scalar {
            }
            // Parent: c_init_remap_particle_count_to_named_model_element_scalar
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_particle_count_to_named_model_mesh_group_scalar {
            }
            // Parent: c_particle_function_initializer
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_inherit_velocity {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_FL_VELOCITY_SCALE: usize = 0x1C4; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_velocity_from_cp {
                pub const M_VELOCITY_INPUT: usize = 0x1C0; // CParticleCollectionVecInput
                pub const M_TRANSFORM_INPUT: usize = 0x818; // CParticleTransformInput
                pub const M_FL_VELOCITY_SCALE: usize = 0x880; // float32
                pub const M_B_DIRECTION_ONLY: usize = 0x884; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_age_noise {
                pub const M_B_ABS_VAL: usize = 0x1C0; // bool
                pub const M_B_ABS_VAL_INV: usize = 0x1C1; // bool
                pub const M_FL_OFFSET: usize = 0x1C4; // float32
                pub const M_FL_AGE_MIN: usize = 0x1C8; // float32
                pub const M_FL_AGE_MAX: usize = 0x1CC; // float32
                pub const M_FL_NOISE_SCALE: usize = 0x1D0; // float32
                pub const M_FL_NOISE_SCALE_LOC: usize = 0x1D4; // float32
                pub const M_VEC_OFFSET_LOC: usize = 0x1D8; // 
            }
            // Parent: c_particle_function_initializer
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_sequence_life_time {
                pub const M_FL_FRAMERATE: usize = 0x1C0; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_scalar_to_vector {
                pub const M_N_FIELD_INPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1C8; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1CC; // float32
                pub const M_VEC_OUTPUT_MIN: usize = 0x1D0; // 
                pub const M_VEC_OUTPUT_MAX: usize = 0x1DC; // 
                pub const M_FL_START_TIME: usize = 0x1E8; // float32
                pub const M_FL_END_TIME: usize = 0x1EC; // float32
                pub const M_N_SET_METHOD: usize = 0x1F0; // ParticleSetMethod_t
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1F4; // int32
                pub const M_B_LOCAL_COORDS: usize = 0x1F8; // bool
                pub const M_FL_REMAP_BIAS: usize = 0x1FC; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_offset_vector_to_vector {
                pub const M_N_FIELD_INPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_VEC_OUTPUT_MIN: usize = 0x1C8; // 
                pub const M_VEC_OUTPUT_MAX: usize = 0x1D4; // 
                pub const M_RANDOMNESS_PARAMETERS: usize = 0x1E0; // CRandomNumberGeneratorParameters
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            pub mod c_init_create_sequential_path_v2 {
                pub const M_F_MAX_DISTANCE: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_FL_NUM_TO_ASSIGN: usize = 0x318; // CParticleCollectionFloatInput
                pub const M_B_LOOP: usize = 0x470; // bool
                pub const M_B_CP_PAIRS: usize = 0x471; // bool
                pub const M_B_SAVE_OFFSET: usize = 0x472; // bool
                pub const M_PATH_PARAMS: usize = 0x480; // CPathParameters
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            pub mod c_init_create_sequential_path {
                pub const M_F_MAX_DISTANCE: usize = 0x1C0; // float32
                pub const M_FL_NUM_TO_ASSIGN: usize = 0x1C4; // float32
                pub const M_B_LOOP: usize = 0x1C8; // bool
                pub const M_B_CP_PAIRS: usize = 0x1C9; // bool
                pub const M_B_SAVE_OFFSET: usize = 0x1CA; // bool
                pub const M_PATH_PARAMS: usize = 0x1D0; // CPathParameters
            }
            // Parent: c_particle_function_initializer
            // Fields count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_initial_repulsion_velocity {
                pub const M_COLLISION_GROUP_NAME: usize = 0x1C0; // char[128]
                pub const M_N_TRACE_SET: usize = 0x240; // ParticleTraceSet_t
                pub const M_VEC_OUTPUT_MIN: usize = 0x244; // 
                pub const M_VEC_OUTPUT_MAX: usize = 0x250; // 
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x25C; // int32
                pub const M_B_PER_PARTICLE: usize = 0x260; // bool
                pub const M_B_TRANSLATE: usize = 0x261; // bool
                pub const M_B_PROPORTIONAL: usize = 0x262; // bool
                pub const M_FL_TRACE_LENGTH: usize = 0x264; // float32
                pub const M_B_PER_PARTICLE_TR: usize = 0x268; // bool
                pub const M_B_INHERIT: usize = 0x269; // bool
                pub const M_N_CHILD_CP: usize = 0x26C; // int32
                pub const M_N_CHILD_GROUP_ID: usize = 0x270; // int32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_yaw_flip {
                pub const M_FL_PERCENT: usize = 0x1C0; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_second_sequence {
                pub const M_N_SEQUENCE_MIN: usize = 0x1C0; // int32
                pub const M_N_SEQUENCE_MAX: usize = 0x1C4; // int32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_c_pto_scalar {
                pub const M_N_CP_INPUT: usize = 0x1C0; // int32
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_N_FIELD: usize = 0x1C8; // int32
                pub const M_FL_INPUT_MIN: usize = 0x1CC; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1D0; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D4; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D8; // float32
                pub const M_FL_START_TIME: usize = 0x1DC; // float32
                pub const M_FL_END_TIME: usize = 0x1E0; // float32
                pub const M_N_SET_METHOD: usize = 0x1E4; // ParticleSetMethod_t
                pub const M_FL_REMAP_BIAS: usize = 0x1E8; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_transform_to_vector {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_V_INPUT_MIN: usize = 0x1C4; // 
                pub const M_V_INPUT_MAX: usize = 0x1D0; // 
                pub const M_V_OUTPUT_MIN: usize = 0x1DC; // 
                pub const M_V_OUTPUT_MAX: usize = 0x1E8; // 
                pub const M_TRANSFORM_INPUT: usize = 0x1F8; // CParticleTransformInput
                pub const M_LOCAL_SPACE_TRANSFORM: usize = 0x260; // CParticleTransformInput
                pub const M_FL_START_TIME: usize = 0x2C8; // float32
                pub const M_FL_END_TIME: usize = 0x2CC; // float32
                pub const M_N_SET_METHOD: usize = 0x2D0; // ParticleSetMethod_t
                pub const M_B_OFFSET: usize = 0x2D4; // bool
                pub const M_B_ACCELERATE: usize = 0x2D5; // bool
                pub const M_FL_REMAP_BIAS: usize = 0x2D8; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_chaotic_attractor {
                pub const M_FL_A_PARM: usize = 0x1C0; // float32
                pub const M_FL_B_PARM: usize = 0x1C4; // float32
                pub const M_FL_C_PARM: usize = 0x1C8; // float32
                pub const M_FL_D_PARM: usize = 0x1CC; // float32
                pub const M_FL_SCALE: usize = 0x1D0; // float32
                pub const M_FL_SPEED_MIN: usize = 0x1D4; // float32
                pub const M_FL_SPEED_MAX: usize = 0x1D8; // float32
                pub const M_N_BASE_CP: usize = 0x1DC; // int32
                pub const M_B_UNIFORM_SPEED: usize = 0x1E0; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_from_parent_particles {
                pub const M_FL_VELOCITY_SCALE: usize = 0x1C0; // float32
                pub const M_FL_INCREMENT: usize = 0x1C4; // float32
                pub const M_B_RANDOM_DISTRIBUTION: usize = 0x1C8; // bool
                pub const M_N_RANDOM_SEED: usize = 0x1CC; // int32
                pub const M_B_SUB_FRAME: usize = 0x1D0; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_inherit_from_parent_particles {
                pub const M_FL_SCALE: usize = 0x1C0; // float32
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_N_INCREMENT: usize = 0x1C8; // int32
                pub const M_B_RANDOM_DISTRIBUTION: usize = 0x1CC; // bool
                pub const M_N_RANDOM_SEED: usize = 0x1D0; // int32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_from_c_ps {
                pub const M_N_INCREMENT: usize = 0x1C0; // int32
                pub const M_N_MIN_CP: usize = 0x1C4; // int32
                pub const M_N_MAX_CP: usize = 0x1C8; // int32
                pub const M_N_DYNAMIC_CP_COUNT: usize = 0x1D0; // CParticleCollectionFloatInput
            }
            // Parent: c_particle_function_initializer
            // Fields count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_distance_to_cp_init {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1C8; // CPerParticleFloatInput
                pub const M_FL_INPUT_MAX: usize = 0x320; // CPerParticleFloatInput
                pub const M_FL_OUTPUT_MIN: usize = 0x478; // CPerParticleFloatInput
                pub const M_FL_OUTPUT_MAX: usize = 0x5D0; // CPerParticleFloatInput
                pub const M_N_START_CP: usize = 0x728; // int32
                pub const M_B_LOS: usize = 0x72C; // bool
                pub const M_COLLISION_GROUP_NAME: usize = 0x72D; // char[128]
                pub const M_N_TRACE_SET: usize = 0x7B0; // ParticleTraceSet_t
                pub const M_FL_MAX_TRACE_LENGTH: usize = 0x7B8; // CPerParticleFloatInput
                pub const M_FL_LOS_SCALE: usize = 0x910; // float32
                pub const M_N_SET_METHOD: usize = 0x914; // ParticleSetMethod_t
                pub const M_B_ACTIVE_RANGE: usize = 0x918; // bool
                pub const M_VEC_DISTANCE_SCALE: usize = 0x91C; // 
                pub const M_FL_REMAP_BIAS: usize = 0x928; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_lifespan_from_velocity {
                pub const M_VEC_COMPONENT_SCALE: usize = 0x1C0; // 
                pub const M_FL_TRACE_OFFSET: usize = 0x1CC; // float32
                pub const M_FL_MAX_TRACE_LENGTH: usize = 0x1D0; // float32
                pub const M_FL_TRACE_TOLERANCE: usize = 0x1D4; // float32
                pub const M_N_MAX_PLANES: usize = 0x1D8; // int32
                pub const M_COLLISION_GROUP_NAME: usize = 0x1E0; // char[128]
                pub const M_N_TRACE_SET: usize = 0x260; // ParticleTraceSet_t
                pub const M_B_INCLUDE_WATER: usize = 0x270; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_from_plane_cache {
                pub const M_VEC_OFFSET_MIN: usize = 0x1C0; // 
                pub const M_VEC_OFFSET_MAX: usize = 0x1CC; // 
                pub const M_B_USE_NORMAL: usize = 0x1D9; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_model_cull {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_B_BOUND_BOX: usize = 0x1C4; // bool
                pub const M_B_CULL_OUTSIDE: usize = 0x1C5; // bool
                pub const M_B_USE_BONES: usize = 0x1C6; // bool
                pub const M_HITBOX_SET_NAME: usize = 0x1C7; // char[128]
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_distance_cull {
                pub const M_N_CONTROL_POINT: usize = 0x1C0; // int32
                pub const M_FL_DISTANCE: usize = 0x1C8; // CParticleCollectionFloatInput
                pub const M_B_CULL_INSIDE: usize = 0x320; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_plane_cull {
                pub const M_N_CONTROL_POINT: usize = 0x1C0; // int32
                pub const M_FL_DISTANCE: usize = 0x1C8; // CParticleCollectionFloatInput
                pub const M_B_CULL_INSIDE: usize = 0x320; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_distance_to_neighbor_cull {
                pub const M_FL_DISTANCE: usize = 0x1C0; // CPerParticleFloatInput
            }
            // Parent: c_particle_function_initializer
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_rt_env_cull {
                pub const M_VEC_TEST_DIR: usize = 0x1C0; // 
                pub const M_VEC_TEST_NORMAL: usize = 0x1CC; // 
                pub const M_B_USE_VELOCITY: usize = 0x1D8; // bool
                pub const M_B_CULL_ON_MISS: usize = 0x1D9; // bool
                pub const M_B_LIFE_ADJUST: usize = 0x1DA; // bool
                pub const M_RT_ENV_NAME: usize = 0x1DB; // char[128]
                pub const M_N_RT_ENV_CP: usize = 0x25C; // int32
                pub const M_N_COMPONENT: usize = 0x260; // int32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_normal_align_to_cp {
                pub const M_TRANSFORM_INPUT: usize = 0x1C0; // CParticleTransformInput
                pub const M_N_CONTROL_POINT_AXIS: usize = 0x228; // ParticleControlPointAxis_t
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_normal_offset {
                pub const M_OFFSET_MIN: usize = 0x1C0; // 
                pub const M_OFFSET_MAX: usize = 0x1CC; // 
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1D8; // int32
                pub const M_B_LOCAL_COORDS: usize = 0x1DC; // bool
                pub const M_B_NORMALIZE: usize = 0x1DD; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_speed_to_scalar {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C4; // int32
                pub const M_FL_START_TIME: usize = 0x1C8; // float32
                pub const M_FL_END_TIME: usize = 0x1CC; // float32
                pub const M_FL_INPUT_MIN: usize = 0x1D0; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1D4; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D8; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1DC; // float32
                pub const M_N_SET_METHOD: usize = 0x1E0; // ParticleSetMethod_t
                pub const M_B_PER_PARTICLE: usize = 0x1E4; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_init_from_cp_snapshot {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_N_ATTRIBUTE_TO_READ: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_N_ATTRIBUTE_TO_WRITE: usize = 0x1C8; // ParticleAttributeIndex_t
                pub const M_N_LOCAL_SPACE_CP: usize = 0x1CC; // int32
                pub const M_B_RANDOM: usize = 0x1D0; // bool
                pub const M_B_REVERSE: usize = 0x1D1; // bool
                pub const M_N_SNAP_SHOT_INCREMENT: usize = 0x1D8; // CParticleCollectionFloatInput
                pub const M_N_MANUAL_SNAPSHOT_INDEX: usize = 0x330; // CPerParticleFloatInput
                pub const M_N_RANDOM_SEED: usize = 0x488; // int32
                pub const M_B_LOCAL_SPACE_ANGLES: usize = 0x48C; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 19
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_init_skinned_position_from_cp_snapshot {
                pub const M_N_SNAPSHOT_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C4; // int32
                pub const M_B_RANDOM: usize = 0x1C8; // bool
                pub const M_N_RANDOM_SEED: usize = 0x1CC; // int32
                pub const M_B_RIGID: usize = 0x1D0; // bool
                pub const M_B_SET_NORMAL: usize = 0x1D1; // bool
                pub const M_B_IGNORE_DT: usize = 0x1D2; // bool
                pub const M_FL_MIN_NORMAL_VELOCITY: usize = 0x1D4; // float32
                pub const M_FL_MAX_NORMAL_VELOCITY: usize = 0x1D8; // float32
                pub const M_N_INDEX_TYPE: usize = 0x1DC; // SnapshotIndexType_t
                pub const M_FL_READ_INDEX: usize = 0x1E0; // CPerParticleFloatInput
                pub const M_FL_INCREMENT: usize = 0x338; // float32
                pub const M_N_FULL_LOOP_INCREMENT: usize = 0x33C; // int32
                pub const M_N_SNAP_SHOT_START_POINT: usize = 0x340; // int32
                pub const M_FL_BONE_VELOCITY: usize = 0x344; // float32
                pub const M_FL_BONE_VELOCITY_MAX: usize = 0x348; // float32
                pub const M_B_COPY_COLOR: usize = 0x34C; // bool
                pub const M_B_COPY_ALPHA: usize = 0x34D; // bool
                pub const M_B_SET_RADIUS: usize = 0x34E; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_init_from_parent_killed {
                pub const M_N_ATTRIBUTE_TO_COPY: usize = 0x1C0; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_init_from_vector_field_snapshot {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_N_LOCAL_SPACE_CP: usize = 0x1C4; // int32
                pub const M_N_WEIGHT_UPDATE_CP: usize = 0x1C8; // int32
                pub const M_B_USE_VERTICAL_VELOCITY: usize = 0x1CC; // bool
                pub const M_VEC_SCALE: usize = 0x1D0; // CPerParticleVecInput
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_initial_direction_to_transform_to_vector {
                pub const M_TRANSFORM_INPUT: usize = 0x1C0; // CParticleTransformInput
                pub const M_N_FIELD_OUTPUT: usize = 0x228; // ParticleAttributeIndex_t
                pub const M_FL_SCALE: usize = 0x22C; // float32
                pub const M_FL_OFFSET_ROT: usize = 0x230; // float32
                pub const M_VEC_OFFSET_AXIS: usize = 0x234; // 
                pub const M_B_NORMALIZE: usize = 0x240; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_initial_transform_direction_to_rotation {
                pub const M_TRANSFORM_INPUT: usize = 0x1C0; // CParticleTransformInput
                pub const M_N_FIELD_OUTPUT: usize = 0x228; // ParticleAttributeIndex_t
                pub const M_FL_OFFSET_ROT: usize = 0x22C; // float32
                pub const M_N_COMPONENT: usize = 0x230; // int32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_q_angles_to_rotation {
                pub const M_TRANSFORM_INPUT: usize = 0x1C0; // CParticleTransformInput
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_transform_orientation_to_rotations {
                pub const M_TRANSFORM_INPUT: usize = 0x1C0; // CParticleTransformInput
                pub const M_VEC_ROTATION: usize = 0x228; // 
                pub const M_B_USE_QUAT: usize = 0x234; // bool
                pub const M_B_WRITE_NORMAL: usize = 0x235; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_set_rigid_attachment {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_N_FIELD_INPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x1C8; // ParticleAttributeIndex_t
                pub const M_B_LOCAL_SPACE: usize = 0x1CC; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_initial_visibility_scalar {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1C8; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1CC; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D0; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D4; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_radius_from_cp_object {
                pub const M_N_CONTROL_POINT: usize = 0x1C0; // int32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_initial_sequence_from_model {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT_ANIM: usize = 0x1C8; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1CC; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1D0; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D4; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D8; // float32
                pub const M_N_SET_METHOD: usize = 0x1DC; // ParticleSetMethod_t
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_global_scale {
                pub const M_FL_SCALE: usize = 0x1C0; // float32
                pub const M_N_SCALE_CONTROL_POINT_NUMBER: usize = 0x1C4; // int32
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1C8; // int32
                pub const M_B_SCALE_RADIUS: usize = 0x1CC; // bool
                pub const M_B_SCALE_POSITION: usize = 0x1CD; // bool
                pub const M_B_SCALE_VELOCITY: usize = 0x1CE; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_point_list {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_POINT_LIST: usize = 0x1C8; // CUtlVector< PointDefinition_t >
                pub const M_B_PLACE_ALONG_PATH: usize = 0x1E0; // bool
                pub const M_B_CLOSED_LOOP: usize = 0x1E1; // bool
                pub const M_N_NUM_POINTS_ALONG_PATH: usize = 0x1E4; // int32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_named_model_element {
                pub const M_H_MODEL: usize = 0x1C0; // CStrongHandle< InfoForResourceTypeCModel >
                pub const M_NAMES: usize = 0x1C8; // CUtlVector< CUtlString >
                pub const M_B_SHUFFLE: usize = 0x1E0; // bool
                pub const M_B_LINEAR: usize = 0x1E1; // bool
                pub const M_B_MODEL_FROM_RENDERER: usize = 0x1E2; // bool
                pub const M_N_FIELD_OUTPUT: usize = 0x1E4; // ParticleAttributeIndex_t
            }
            // Parent: c_init_random_named_model_element
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_named_model_sequence {
            }
            // Parent: c_init_random_named_model_element
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_named_model_body_part {
            }
            // Parent: c_init_random_named_model_element
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_random_named_model_mesh_group {
            }
            // Parent: c_particle_function_initializer
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_named_model_element_to_scalar {
                pub const M_H_MODEL: usize = 0x1C0; // CStrongHandle< InfoForResourceTypeCModel >
                pub const M_NAMES: usize = 0x1C8; // CUtlVector< CUtlString >
                pub const M_VALUES: usize = 0x1E0; // CUtlVector< float32 >
                pub const M_N_FIELD_INPUT: usize = 0x1F8; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x1FC; // ParticleAttributeIndex_t
                pub const M_N_SET_METHOD: usize = 0x200; // ParticleSetMethod_t
                pub const M_B_MODEL_FROM_RENDERER: usize = 0x204; // bool
            }
            // Parent: c_init_remap_named_model_element_to_scalar
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_named_model_sequence_to_scalar {
            }
            // Parent: c_init_remap_named_model_element_to_scalar
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_named_model_body_part_to_scalar {
            }
            // Parent: c_init_remap_named_model_element_to_scalar
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_remap_named_model_mesh_group_to_scalar {
            }
            // Parent: c_particle_function_initializer
            // Fields count: 18
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_status_effect {
                pub const M_N_DETAIL2_COMBO: usize = 0x1C0; // Detail2Combo_t
                pub const M_FL_DETAIL2_ROTATION: usize = 0x1C4; // float32
                pub const M_FL_DETAIL2_SCALE: usize = 0x1C8; // float32
                pub const M_FL_DETAIL2_BLEND_FACTOR: usize = 0x1CC; // float32
                pub const M_FL_COLOR_WARP_INTENSITY: usize = 0x1D0; // float32
                pub const M_FL_DIFFUSE_WARP_BLEND_TO_FULL: usize = 0x1D4; // float32
                pub const M_FL_ENV_MAP_INTENSITY: usize = 0x1D8; // float32
                pub const M_FL_AMBIENT_SCALE: usize = 0x1DC; // float32
                pub const M_SPECULAR_COLOR: usize = 0x1E0; // Color
                pub const M_FL_SPECULAR_SCALE: usize = 0x1E4; // float32
                pub const M_FL_SPECULAR_EXPONENT: usize = 0x1E8; // float32
                pub const M_FL_SPECULAR_EXPONENT_BLEND_TO_FULL: usize = 0x1EC; // float32
                pub const M_FL_SPECULAR_BLEND_TO_FULL: usize = 0x1F0; // float32
                pub const M_RIM_LIGHT_COLOR: usize = 0x1F4; // Color
                pub const M_FL_RIM_LIGHT_SCALE: usize = 0x1F8; // float32
                pub const M_FL_REFLECTIONS_TINT_BY_BASE_BLEND_TO_NONE: usize = 0x1FC; // float32
                pub const M_FL_METALNESS_BLEND_TO_FULL: usize = 0x200; // float32
                pub const M_FL_SELF_ILLUM_BLEND_TO_FULL: usize = 0x204; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 19
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_status_effect_citadel {
                pub const M_FL_SFX_COLOR_WARP_AMOUNT: usize = 0x1C0; // float32
                pub const M_FL_SFX_NORMAL_AMOUNT: usize = 0x1C4; // float32
                pub const M_FL_SFX_METALNESS_AMOUNT: usize = 0x1C8; // float32
                pub const M_FL_SFX_ROUGHNESS_AMOUNT: usize = 0x1CC; // float32
                pub const M_FL_SFX_SELF_ILLUM_AMOUNT: usize = 0x1D0; // float32
                pub const M_FL_SFXS_SCALE: usize = 0x1D4; // float32
                pub const M_FL_SFXS_SCROLL_X: usize = 0x1D8; // float32
                pub const M_FL_SFXS_SCROLL_Y: usize = 0x1DC; // float32
                pub const M_FL_SFXS_SCROLL_Z: usize = 0x1E0; // float32
                pub const M_FL_SFXS_OFFSET_X: usize = 0x1E4; // float32
                pub const M_FL_SFXS_OFFSET_Y: usize = 0x1E8; // float32
                pub const M_FL_SFXS_OFFSET_Z: usize = 0x1EC; // float32
                pub const M_N_DETAIL_COMBO: usize = 0x1F0; // DetailCombo_t
                pub const M_FL_SFXS_DETAIL_AMOUNT: usize = 0x1F4; // float32
                pub const M_FL_SFXS_DETAIL_SCALE: usize = 0x1F8; // float32
                pub const M_FL_SFXS_DETAIL_SCROLL_X: usize = 0x1FC; // float32
                pub const M_FL_SFXS_DETAIL_SCROLL_Y: usize = 0x200; // float32
                pub const M_FL_SFXS_DETAIL_SCROLL_Z: usize = 0x204; // float32
                pub const M_FL_SFXS_USE_MODEL_U_VS: usize = 0x208; // float32
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_create_particle_impulse {
                pub const M_INPUT_RADIUS: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_INPUT_MAGNITUDE: usize = 0x318; // CPerParticleFloatInput
                pub const M_N_FALLOFF_FUNCTION: usize = 0x470; // ParticleFalloffFunction_t
                pub const M_INPUT_FALLOFF_EXP: usize = 0x478; // CPerParticleFloatInput
                pub const M_N_IMPULSE_TYPE: usize = 0x5D0; // ParticleImpulseType_t
            }
            // Parent: c_particle_function_initializer
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_quantize_float {
                pub const M_INPUT_VALUE: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_N_OUTPUT_FIELD: usize = 0x318; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_set_attribute_to_scalar_expression {
                pub const M_N_EXPRESSION: usize = 0x1C0; // ScalarExpressionType_t
                pub const M_FL_INPUT1: usize = 0x1C8; // CPerParticleFloatInput
                pub const M_FL_INPUT2: usize = 0x320; // CPerParticleFloatInput
                pub const M_FL_OUTPUT_REMAP: usize = 0x478; // CParticleRemapFloatInput
                pub const M_N_OUTPUT_FIELD: usize = 0x5D0; // ParticleAttributeIndex_t
                pub const M_N_SET_METHOD: usize = 0x5D4; // ParticleSetMethod_t
            }
            // Parent: c_particle_function_initializer
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_set_vector_attribute_to_vector_expression {
                pub const M_N_EXPRESSION: usize = 0x1C0; // VectorExpressionType_t
                pub const M_V_INPUT1: usize = 0x1C8; // CPerParticleVecInput
                pub const M_V_INPUT2: usize = 0x820; // CPerParticleVecInput
                pub const M_N_OUTPUT_FIELD: usize = 0xE78; // ParticleAttributeIndex_t
                pub const M_N_SET_METHOD: usize = 0xE7C; // ParticleSetMethod_t
                pub const M_B_NORMALIZED_OUTPUT: usize = 0xE80; // bool
            }
            // Parent: c_particle_function_initializer
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_init_float_collection {
                pub const M_INPUT_VALUE: usize = 0x1C0; // CParticleCollectionFloatInput
                pub const M_N_OUTPUT_FIELD: usize = 0x318; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_initializer
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_init_float {
                pub const M_INPUT_VALUE: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_N_OUTPUT_FIELD: usize = 0x318; // ParticleAttributeIndex_t
                pub const M_N_SET_METHOD: usize = 0x31C; // ParticleSetMethod_t
                pub const M_INPUT_STRENGTH: usize = 0x320; // CPerParticleFloatInput
            }
            // Parent: c_particle_function_initializer
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_init_vec_collection {
                pub const M_INPUT_VALUE: usize = 0x1C0; // CParticleCollectionVecInput
                pub const M_N_OUTPUT_FIELD: usize = 0x818; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_initializer
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_init_init_vec {
                pub const M_INPUT_VALUE: usize = 0x1C0; // CPerParticleVecInput
                pub const M_N_OUTPUT_FIELD: usize = 0x818; // ParticleAttributeIndex_t
                pub const M_N_SET_METHOD: usize = 0x81C; // ParticleSetMethod_t
                pub const M_B_NORMALIZED_OUTPUT: usize = 0x820; // bool
                pub const M_B_WRITE_PREVIOUS_POSITION: usize = 0x821; // bool
            }
            // Parent: c_particle_function_emitter
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_instantaneous_emitter {
                pub const M_N_PARTICLES_TO_EMIT: usize = 0x1C0; // CParticleCollectionFloatInput
                pub const M_FL_START_TIME: usize = 0x318; // CParticleCollectionFloatInput
                pub const M_FL_INIT_FROM_KILLED_PARENT_PARTICLES: usize = 0x470; // float32
                pub const M_FL_PARENT_PARTICLE_SCALE: usize = 0x478; // CParticleCollectionFloatInput
                pub const M_N_MAX_EMITTED_PER_FRAME: usize = 0x5D0; // int32
                pub const M_N_SNAPSHOT_CONTROL_POINT: usize = 0x5D4; // int32
            }
            // Parent: c_particle_function_emitter
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_continuous_emitter {
                pub const M_FL_EMISSION_DURATION: usize = 0x1C0; // CParticleCollectionFloatInput
                pub const M_FL_START_TIME: usize = 0x318; // CParticleCollectionFloatInput
                pub const M_FL_EMIT_RATE: usize = 0x470; // CParticleCollectionFloatInput
                pub const M_FL_EMISSION_SCALE: usize = 0x5C8; // float32
                pub const M_FL_SCALE_PER_PARENT_PARTICLE: usize = 0x5CC; // float32
                pub const M_B_INIT_FROM_KILLED_PARENT_PARTICLES: usize = 0x5D0; // bool
                pub const M_N_SNAPSHOT_CONTROL_POINT: usize = 0x5D4; // int32
                pub const M_N_LIMIT_PER_UPDATE: usize = 0x5D8; // int32
                pub const M_B_FORCE_EMIT_ON_FIRST_UPDATE: usize = 0x5DC; // bool
                pub const M_B_FORCE_EMIT_ON_LAST_UPDATE: usize = 0x5DD; // bool
            }
            // Parent: c_particle_function_emitter
            // Fields count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_noise_emitter {
                pub const M_FL_EMISSION_DURATION: usize = 0x1C0; // float32
                pub const M_FL_START_TIME: usize = 0x1C4; // float32
                pub const M_FL_EMISSION_SCALE: usize = 0x1C8; // float32
                pub const M_N_SCALE_CONTROL_POINT: usize = 0x1CC; // int32
                pub const M_N_SCALE_CONTROL_POINT_FIELD: usize = 0x1D0; // int32
                pub const M_N_WORLD_NOISE_POINT: usize = 0x1D4; // int32
                pub const M_B_ABS_VAL: usize = 0x1D8; // bool
                pub const M_B_ABS_VAL_INV: usize = 0x1D9; // bool
                pub const M_FL_OFFSET: usize = 0x1DC; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1E0; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1E4; // float32
                pub const M_FL_NOISE_SCALE: usize = 0x1E8; // float32
                pub const M_FL_WORLD_NOISE_SCALE: usize = 0x1EC; // float32
                pub const M_VEC_OFFSET_LOC: usize = 0x1F0; // 
                pub const M_FL_WORLD_TIME_SCALE: usize = 0x1FC; // float32
            }
            // Parent: c_particle_function_emitter
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_maintain_emitter {
                pub const M_N_PARTICLES_TO_MAINTAIN: usize = 0x1C0; // CParticleCollectionFloatInput
                pub const M_FL_START_TIME: usize = 0x318; // float32
                pub const M_FL_EMISSION_DURATION: usize = 0x320; // CParticleCollectionFloatInput
                pub const M_FL_EMISSION_RATE: usize = 0x478; // float32
                pub const M_N_SNAPSHOT_CONTROL_POINT: usize = 0x47C; // int32
                pub const M_B_EMIT_INSTANTANEOUSLY: usize = 0x480; // bool
                pub const M_B_FINAL_EMIT_ON_STOP: usize = 0x481; // bool
                pub const M_FL_SCALE: usize = 0x488; // CParticleCollectionFloatInput
            }
            // Parent: c_particle_function_force
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_random_force {
                pub const M_MIN_FORCE: usize = 0x1D0; // 
                pub const M_MAX_FORCE: usize = 0x1DC; // 
            }
            // Parent: c_particle_function_force
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_cp_velocity_force {
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1D0; // int32
                pub const M_FL_SCALE: usize = 0x1D8; // CPerParticleFloatInput
            }
            // Parent: c_particle_function_force
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_parent_vortices {
                pub const M_FL_FORCE_SCALE: usize = 0x1D0; // float32
                pub const M_VEC_TWIST_AXIS: usize = 0x1D4; // 
                pub const M_B_FLIP_BASED_ON_YAW: usize = 0x1E0; // bool
            }
            // Parent: c_particle_function_force
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_twist_around_axis {
                pub const M_F_FORCE_AMOUNT: usize = 0x1D0; // float32
                pub const M_TWIST_AXIS: usize = 0x1D4; // 
                pub const M_B_LOCAL_SPACE: usize = 0x1E0; // bool
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1E4; // int32
            }
            // Parent: c_particle_function_force
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_attract_to_control_point {
                pub const M_VEC_COMPONENT_SCALE: usize = 0x1D0; // 
                pub const M_F_FORCE_AMOUNT: usize = 0x1E0; // CPerParticleFloatInput
                pub const M_F_FALLOFF_POWER: usize = 0x338; // float32
                pub const M_TRANSFORM_INPUT: usize = 0x340; // CParticleTransformInput
                pub const M_F_FORCE_AMOUNT_MIN: usize = 0x3A8; // CPerParticleFloatInput
                pub const M_B_APPLY_MIN_FORCE: usize = 0x500; // bool
            }
            // Parent: c_particle_function_force
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_force_based_on_distance_to_plane {
                pub const M_FL_MIN_DIST: usize = 0x1D0; // float32
                pub const M_VEC_FORCE_AT_MIN_DIST: usize = 0x1D4; // 
                pub const M_FL_MAX_DIST: usize = 0x1E0; // float32
                pub const M_VEC_FORCE_AT_MAX_DIST: usize = 0x1E4; // 
                pub const M_VEC_PLANE_NORMAL: usize = 0x1F0; // 
                pub const M_N_CONTROL_POINT_NUMBER: usize = 0x1FC; // int32
                pub const M_FL_EXPONENT: usize = 0x200; // float32
            }
            // Parent: c_particle_function_force
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_time_varying_force {
                pub const M_FL_START_LERP_TIME: usize = 0x1D0; // float32
                pub const M_STARTING_FORCE: usize = 0x1D4; // 
                pub const M_FL_END_LERP_TIME: usize = 0x1E0; // float32
                pub const M_ENDING_FORCE: usize = 0x1E4; // 
            }
            // Parent: c_particle_function_force
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_turbulence_force {
                pub const M_FL_NOISE_COORD_SCALE0: usize = 0x1D0; // float32
                pub const M_FL_NOISE_COORD_SCALE1: usize = 0x1D4; // float32
                pub const M_FL_NOISE_COORD_SCALE2: usize = 0x1D8; // float32
                pub const M_FL_NOISE_COORD_SCALE3: usize = 0x1DC; // float32
                pub const M_VEC_NOISE_AMOUNT0: usize = 0x1E0; // 
                pub const M_VEC_NOISE_AMOUNT1: usize = 0x1EC; // 
                pub const M_VEC_NOISE_AMOUNT2: usize = 0x1F8; // 
                pub const M_VEC_NOISE_AMOUNT3: usize = 0x204; // 
            }
            // Parent: c_particle_function_force
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_curl_noise_force {
                pub const M_N_NOISE_TYPE: usize = 0x1D0; // ParticleDirectionNoiseType_t
                pub const M_VEC_NOISE_FREQ: usize = 0x1D8; // CPerParticleVecInput
                pub const M_VEC_NOISE_SCALE: usize = 0x830; // CPerParticleVecInput
                pub const M_VEC_OFFSET: usize = 0xE88; // CPerParticleVecInput
                pub const M_VEC_OFFSET_RATE: usize = 0x14E0; // CPerParticleVecInput
                pub const M_FL_WORLEY_SEED: usize = 0x1B38; // CPerParticleFloatInput
                pub const M_FL_WORLEY_JITTER: usize = 0x1C90; // CPerParticleFloatInput
            }
            // Parent: c_particle_function_force
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_per_particle_force {
                pub const M_FL_FORCE_SCALE: usize = 0x1D0; // CPerParticleFloatInput
                pub const M_V_FORCE: usize = 0x328; // CPerParticleVecInput
                pub const M_N_CP: usize = 0x980; // int32
            }
            // Parent: c_particle_function_force
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_wind_force {
                pub const M_V_FORCE: usize = 0x1D0; // 
            }
            // Parent: c_particle_function_force
            // Fields count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_external_wind_force {
                pub const M_VEC_SAMPLE_POSITION: usize = 0x1D0; // CPerParticleVecInput
                pub const M_VEC_SCALE: usize = 0x828; // CPerParticleVecInput
                pub const M_B_SAMPLE_WIND: usize = 0xE80; // bool
                pub const M_B_SAMPLE_WATER: usize = 0xE81; // bool
                pub const M_B_DAMPEN_NEAR_WATER_PLANE: usize = 0xE82; // bool
                pub const M_B_SAMPLE_GRAVITY: usize = 0xE83; // bool
                pub const M_VEC_GRAVITY_FORCE: usize = 0xE88; // CPerParticleVecInput
                pub const M_B_USE_BASIC_MOVEMENT_GRAVITY: usize = 0x14E0; // bool
                pub const M_FL_LOCAL_GRAVITY_SCALE: usize = 0x14E8; // CPerParticleFloatInput
                pub const M_FL_LOCAL_BUOYANCY_SCALE: usize = 0x1640; // CPerParticleFloatInput
                pub const M_VEC_BUOYANCY_FORCE: usize = 0x1798; // CPerParticleVecInput
            }
            // Parent: c_particle_function_force
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_external_game_impulse_force {
                pub const M_FL_FORCE_SCALE: usize = 0x1D0; // CPerParticleFloatInput
                pub const M_B_ROPES: usize = 0x328; // bool
                pub const M_B_ROPES_Z_ONLY: usize = 0x329; // bool
                pub const M_B_EXPLOSIONS: usize = 0x32A; // bool
                pub const M_B_PARTICLES: usize = 0x32B; // bool
            }
            // Parent: c_particle_function_force
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_local_acceleration_force {
                pub const M_N_CP: usize = 0x1D0; // int32
                pub const M_N_SCALE_CP: usize = 0x1D4; // int32
                pub const M_VEC_ACCEL: usize = 0x1D8; // CParticleCollectionVecInput
            }
            // Parent: c_particle_function_force
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_density_force {
                pub const M_FL_RADIUS_SCALE: usize = 0x1D0; // float32
                pub const M_FL_FORCE_SCALE: usize = 0x1D4; // float32
                pub const M_FL_TARGET_DENSITY: usize = 0x1D8; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_basic_movement {
                pub const M_GRAVITY: usize = 0x1C0; // CParticleCollectionVecInput
                pub const M_F_DRAG: usize = 0x818; // CParticleCollectionFloatInput
                pub const M_N_MAX_CONSTRAINT_PASSES: usize = 0x970; // int32
            }
            // Parent: c_particle_function_operator
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_fade_and_kill {
                pub const M_FL_START_FADE_IN_TIME: usize = 0x1C0; // float32
                pub const M_FL_END_FADE_IN_TIME: usize = 0x1C4; // float32
                pub const M_FL_START_FADE_OUT_TIME: usize = 0x1C8; // float32
                pub const M_FL_END_FADE_OUT_TIME: usize = 0x1CC; // float32
                pub const M_FL_START_ALPHA: usize = 0x1D0; // float32
                pub const M_FL_END_ALPHA: usize = 0x1D4; // float32
                pub const M_B_FORCE_PRESERVE_PARTICLE_ORDER: usize = 0x1D8; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_fade_and_kill_for_tracers {
                pub const M_FL_START_FADE_IN_TIME: usize = 0x1C0; // float32
                pub const M_FL_END_FADE_IN_TIME: usize = 0x1C4; // float32
                pub const M_FL_START_FADE_OUT_TIME: usize = 0x1C8; // float32
                pub const M_FL_END_FADE_OUT_TIME: usize = 0x1CC; // float32
                pub const M_FL_START_ALPHA: usize = 0x1D0; // float32
                pub const M_FL_END_ALPHA: usize = 0x1D4; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_fade_in {
                pub const M_FL_FADE_IN_TIME_MIN: usize = 0x1C0; // float32
                pub const M_FL_FADE_IN_TIME_MAX: usize = 0x1C4; // float32
                pub const M_FL_FADE_IN_TIME_EXP: usize = 0x1C8; // float32
                pub const M_B_PROPORTIONAL: usize = 0x1CC; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_fade_out {
                pub const M_FL_FADE_OUT_TIME_MIN: usize = 0x1C0; // float32
                pub const M_FL_FADE_OUT_TIME_MAX: usize = 0x1C4; // float32
                pub const M_FL_FADE_OUT_TIME_EXP: usize = 0x1C8; // float32
                pub const M_FL_FADE_BIAS: usize = 0x1CC; // float32
                pub const M_B_PROPORTIONAL: usize = 0x200; // bool
                pub const M_B_EASE_IN_AND_OUT: usize = 0x201; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_fade_in_simple {
                pub const M_FL_FADE_IN_TIME: usize = 0x1C0; // float32
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_operator
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_fade_out_simple {
                pub const M_FL_FADE_OUT_TIME: usize = 0x1C0; // float32
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_operator
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_clamp_scalar {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_FL_OUTPUT_MIN: usize = 0x1C8; // CPerParticleFloatInput
                pub const M_FL_OUTPUT_MAX: usize = 0x320; // CPerParticleFloatInput
            }
            // Parent: c_particle_function_operator
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_clamp_vector {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_VEC_OUTPUT_MIN: usize = 0x1C8; // CPerParticleVecInput
                pub const M_VEC_OUTPUT_MAX: usize = 0x820; // CPerParticleVecInput
            }
            // Parent: c_particle_function_operator
            // Fields count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_oscillate_scalar {
                pub const M_RATE_MIN: usize = 0x1C0; // float32
                pub const M_RATE_MAX: usize = 0x1C4; // float32
                pub const M_FREQUENCY_MIN: usize = 0x1C8; // float32
                pub const M_FREQUENCY_MAX: usize = 0x1CC; // float32
                pub const M_N_FIELD: usize = 0x1D0; // ParticleAttributeIndex_t
                pub const M_B_PROPORTIONAL: usize = 0x1D4; // bool
                pub const M_B_PROPORTIONAL_OP: usize = 0x1D5; // bool
                pub const M_FL_START_TIME_MIN: usize = 0x1D8; // float32
                pub const M_FL_START_TIME_MAX: usize = 0x1DC; // float32
                pub const M_FL_END_TIME_MIN: usize = 0x1E0; // float32
                pub const M_FL_END_TIME_MAX: usize = 0x1E4; // float32
                pub const M_FL_OSC_MULT: usize = 0x1E8; // float32
                pub const M_FL_OSC_ADD: usize = 0x1EC; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_oscillate_scalar_simple {
                pub const M_RATE: usize = 0x1C0; // float32
                pub const M_FREQUENCY: usize = 0x1C4; // float32
                pub const M_N_FIELD: usize = 0x1C8; // ParticleAttributeIndex_t
                pub const M_FL_OSC_MULT: usize = 0x1CC; // float32
                pub const M_FL_OSC_ADD: usize = 0x1D0; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_oscillate_vector {
                pub const M_RATE_MIN: usize = 0x1C0; // 
                pub const M_RATE_MAX: usize = 0x1CC; // 
                pub const M_FREQUENCY_MIN: usize = 0x1D8; // 
                pub const M_FREQUENCY_MAX: usize = 0x1E4; // 
                pub const M_N_FIELD: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const M_B_PROPORTIONAL: usize = 0x1F4; // bool
                pub const M_B_PROPORTIONAL_OP: usize = 0x1F5; // bool
                pub const M_B_OFFSET: usize = 0x1F6; // bool
                pub const M_FL_START_TIME_MIN: usize = 0x1F8; // float32
                pub const M_FL_START_TIME_MAX: usize = 0x1FC; // float32
                pub const M_FL_END_TIME_MIN: usize = 0x200; // float32
                pub const M_FL_END_TIME_MAX: usize = 0x204; // float32
                pub const M_FL_OSC_MULT: usize = 0x208; // CPerParticleFloatInput
                pub const M_FL_OSC_ADD: usize = 0x360; // CPerParticleFloatInput
                pub const M_FL_RATE_SCALE: usize = 0x4B8; // CPerParticleFloatInput
            }
            // Parent: c_particle_function_operator
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_oscillate_vector_simple {
                pub const M_RATE: usize = 0x1C0; // 
                pub const M_FREQUENCY: usize = 0x1CC; // 
                pub const M_N_FIELD: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const M_FL_OSC_MULT: usize = 0x1DC; // float32
                pub const M_FL_OSC_ADD: usize = 0x1E0; // float32
                pub const M_B_OFFSET: usize = 0x1E4; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_difference_previous_particle {
                pub const M_N_FIELD_INPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1C8; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1CC; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D0; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D4; // float32
                pub const M_N_SET_METHOD: usize = 0x1D8; // ParticleSetMethod_t
                pub const M_B_ACTIVE_RANGE: usize = 0x1DC; // bool
                pub const M_B_SET_PREVIOUS_PARTICLE: usize = 0x1DD; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_point_vector_at_next_particle {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_FL_INTERPOLATION: usize = 0x1C8; // CPerParticleFloatInput
            }
            // Parent: c_particle_function_operator
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_scalar {
                pub const M_N_FIELD_INPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1C8; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1CC; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D0; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D4; // float32
                pub const M_B_OLD_CODE: usize = 0x1D8; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_density_to_vector {
                pub const M_FL_RADIUS_SCALE: usize = 0x1C0; // float32
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_FL_DENSITY_MIN: usize = 0x1C8; // float32
                pub const M_FL_DENSITY_MAX: usize = 0x1CC; // float32
                pub const M_VEC_OUTPUT_MIN: usize = 0x1D0; // 
                pub const M_VEC_OUTPUT_MAX: usize = 0x1DC; // 
                pub const M_B_USE_PARENT_DENSITY: usize = 0x1E8; // bool
                pub const M_N_VOXEL_GRID_RESOLUTION: usize = 0x1EC; // int32
            }
            // Parent: c_particle_function_operator
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_diffusion {
                pub const M_FL_RADIUS_SCALE: usize = 0x1C0; // float32
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_N_VOXEL_GRID_RESOLUTION: usize = 0x1C8; // int32
            }
            // Parent: c_particle_function_operator
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_scalar_end_cap {
                pub const M_N_FIELD_INPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1C8; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1CC; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D0; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D4; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_reinitialize_scalar_end_cap {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_FL_OUTPUT_MIN: usize = 0x1C4; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1C8; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_scalar_once_timed {
                pub const M_B_PROPORTIONAL: usize = 0x1C0; // bool
                pub const M_N_FIELD_INPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x1C8; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1CC; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1D0; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D4; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D8; // float32
                pub const M_FL_REMAP_TIME: usize = 0x1DC; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_particle_count_on_scalar_end_cap {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_INPUT_MIN: usize = 0x1C4; // int32
                pub const M_N_INPUT_MAX: usize = 0x1C8; // int32
                pub const M_FL_OUTPUT_MIN: usize = 0x1CC; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D0; // float32
                pub const M_B_BACKWARDS: usize = 0x1D4; // bool
                pub const M_N_SET_METHOD: usize = 0x1D8; // ParticleSetMethod_t
            }
            // Parent: c_particle_function_operator
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_particle_count_to_scalar {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_INPUT_MIN: usize = 0x1C8; // CParticleCollectionFloatInput
                pub const M_N_INPUT_MAX: usize = 0x320; // CParticleCollectionFloatInput
                pub const M_FL_OUTPUT_MIN: usize = 0x478; // CParticleCollectionFloatInput
                pub const M_FL_OUTPUT_MAX: usize = 0x5D0; // CParticleCollectionFloatInput
                pub const M_B_ACTIVE_RANGE: usize = 0x728; // bool
                pub const M_N_SET_METHOD: usize = 0x72C; // ParticleSetMethod_t
            }
            // Parent: c_particle_function_operator
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_visibility_scalar {
                pub const M_N_FIELD_INPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1C8; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1CC; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1D0; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D4; // float32
                pub const M_FL_RADIUS_SCALE: usize = 0x1D8; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_transform_visibility_to_scalar {
                pub const M_N_SET_METHOD: usize = 0x1C0; // ParticleSetMethod_t
                pub const M_TRANSFORM_INPUT: usize = 0x1C8; // CParticleTransformInput
                pub const M_N_FIELD_OUTPUT: usize = 0x230; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x234; // float32
                pub const M_FL_INPUT_MAX: usize = 0x238; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x23C; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x240; // float32
                pub const M_FL_RADIUS: usize = 0x244; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_transform_visibility_to_vector {
                pub const M_N_SET_METHOD: usize = 0x1C0; // ParticleSetMethod_t
                pub const M_TRANSFORM_INPUT: usize = 0x1C8; // CParticleTransformInput
                pub const M_N_FIELD_OUTPUT: usize = 0x230; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x234; // float32
                pub const M_FL_INPUT_MAX: usize = 0x238; // float32
                pub const M_VEC_OUTPUT_MIN: usize = 0x23C; // 
                pub const M_VEC_OUTPUT_MAX: usize = 0x248; // 
                pub const M_FL_RADIUS: usize = 0x254; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_lerp_scalar {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_FL_OUTPUT: usize = 0x1C8; // CPerParticleFloatInput
                pub const M_FL_START_TIME: usize = 0x320; // float32
                pub const M_FL_END_TIME: usize = 0x324; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_lerp_end_cap_scalar {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_FL_OUTPUT: usize = 0x1C4; // float32
                pub const M_FL_LERP_TIME: usize = 0x1C8; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_lerp_end_cap_vector {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_VEC_OUTPUT: usize = 0x1C4; // 
                pub const M_FL_LERP_TIME: usize = 0x1D0; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_lerp_vector {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_VEC_OUTPUT: usize = 0x1C4; // 
                pub const M_FL_START_TIME: usize = 0x1D0; // float32
                pub const M_FL_END_TIME: usize = 0x1D4; // float32
                pub const M_N_SET_METHOD: usize = 0x1D8; // ParticleSetMethod_t
            }
            // Parent: c_particle_function_operator
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_lerp_to_other_attribute {
                pub const M_FL_INTERPOLATION: usize = 0x1C0; // CPerParticleFloatInput
                pub const M_N_FIELD_INPUT_FROM: usize = 0x318; // ParticleAttributeIndex_t
                pub const M_N_FIELD_INPUT: usize = 0x31C; // ParticleAttributeIndex_t
                pub const M_N_FIELD_OUTPUT: usize = 0x320; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_operator
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_speed {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1C4; // float32
                pub const M_FL_INPUT_MAX: usize = 0x1C8; // float32
                pub const M_FL_OUTPUT_MIN: usize = 0x1CC; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1D0; // float32
                pub const M_N_SET_METHOD: usize = 0x1D4; // ParticleSetMethod_t
                pub const M_B_IGNORE_DELTA: usize = 0x1D8; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_remap_vectorto_cp {
                pub const M_N_OUT_CONTROL_POINT_NUMBER: usize = 0x1C0; // int32
                pub const M_N_FIELD_INPUT: usize = 0x1C4; // ParticleAttributeIndex_t
                pub const M_N_PARTICLE_NUMBER: usize = 0x1C8; // int32
            }
            // Parent: c_particle_function_operator
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_ramp_scalar_linear {
                pub const M_RATE_MIN: usize = 0x1C0; // float32
                pub const M_RATE_MAX: usize = 0x1C4; // float32
                pub const M_FL_START_TIME_MIN: usize = 0x1C8; // float32
                pub const M_FL_START_TIME_MAX: usize = 0x1CC; // float32
                pub const M_FL_END_TIME_MIN: usize = 0x1D0; // float32
                pub const M_FL_END_TIME_MAX: usize = 0x1D4; // float32
                pub const M_N_FIELD: usize = 0x200; // ParticleAttributeIndex_t
                pub const M_B_PROPORTIONAL_OP: usize = 0x204; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_ramp_scalar_spline {
                pub const M_RATE_MIN: usize = 0x1C0; // float32
                pub const M_RATE_MAX: usize = 0x1C4; // float32
                pub const M_FL_START_TIME_MIN: usize = 0x1C8; // float32
                pub const M_FL_START_TIME_MAX: usize = 0x1CC; // float32
                pub const M_FL_END_TIME_MIN: usize = 0x1D0; // float32
                pub const M_FL_END_TIME_MAX: usize = 0x1D4; // float32
                pub const M_FL_BIAS: usize = 0x1D8; // float32
                pub const M_N_FIELD: usize = 0x200; // ParticleAttributeIndex_t
                pub const M_B_PROPORTIONAL_OP: usize = 0x204; // bool
                pub const M_B_EASE_OUT: usize = 0x205; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_ramp_scalar_linear_simple {
                pub const M_RATE: usize = 0x1C0; // float32
                pub const M_FL_START_TIME: usize = 0x1C4; // float32
                pub const M_FL_END_TIME: usize = 0x1C8; // float32
                pub const M_N_FIELD: usize = 0x1F0; // ParticleAttributeIndex_t
            }
            // Parent: c_particle_function_operator
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_ramp_scalar_spline_simple {
                pub const M_RATE: usize = 0x1C0; // float32
                pub const M_FL_START_TIME: usize = 0x1C4; // float32
                pub const M_FL_END_TIME: usize = 0x1C8; // float32
                pub const M_N_FIELD: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const M_B_EASE_OUT: usize = 0x1F4; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_chladni_wave {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_FL_INPUT_MIN: usize = 0x1C8; // CPerParticleFloatInput
                pub const M_FL_INPUT_MAX: usize = 0x320; // CPerParticleFloatInput
                pub const M_FL_OUTPUT_MIN: usize = 0x478; // CPerParticleFloatInput
                pub const M_FL_OUTPUT_MAX: usize = 0x5D0; // CPerParticleFloatInput
                pub const M_VEC_WAVE_LENGTH: usize = 0x728; // CPerParticleVecInput
                pub const M_VEC_HARMONICS: usize = 0xD80; // CPerParticleVecInput
                pub const M_N_SET_METHOD: usize = 0x13D8; // ParticleSetMethod_t
                pub const M_N_LOCAL_SPACE_CONTROL_POINT: usize = 0x13DC; // int32
                pub const M_B3_D: usize = 0x13E0; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_noise {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_FL_OUTPUT_MIN: usize = 0x1C4; // float32
                pub const M_FL_OUTPUT_MAX: usize = 0x1C8; // float32
                pub const M_FL4_NOISE_SCALE: usize = 0x1CC; // float32
                pub const M_B_ADDITIVE: usize = 0x1D0; // bool
                pub const M_FL_NOISE_ANIMATION_TIME_SCALE: usize = 0x1D4; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_vector_noise {
                pub const M_N_FIELD_OUTPUT: usize = 0x1C0; // ParticleAttributeIndex_t
                pub const M_VEC_OUTPUT_MIN: usize = 0x1C4; // 
                pub const M_VEC_OUTPUT_MAX: usize = 0x1D0; // 
                pub const M_FL4_NOISE_SCALE: usize = 0x1DC; // float32
                pub const M_B_ADDITIVE: usize = 0x1E0; // bool
                pub const M_B_OFFSET: usize = 0x1E1; // bool
                pub const M_FL_NOISE_ANIMATION_TIME_SCALE: usize = 0x1E4; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_decay {
                pub const M_B_ROPE_DECAY: usize = 0x1C0; // bool
                pub const M_B_FORCE_PRESERVE_PARTICLE_ORDER: usize = 0x1C1; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_decay_offscreen {
                pub const M_FL_OFFSCREEN_TIME: usize = 0x1C0; // CParticleCollectionFloatInput
            }
            // Parent: c_particle_function_operator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_end_cap_timed_freeze {
                pub const M_FL_FREEZE_TIME: usize = 0x1C0; // CParticleCollectionFloatInput
            }
            // Parent: c_particle_function_operator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_end_cap_timed_decay {
                pub const M_FL_DECAY_TIME: usize = 0x1C0; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_end_cap_decay {
            }
            // Parent: c_particle_function_operator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_velocity_decay {
                pub const M_FL_MIN_VELOCITY: usize = 0x1C0; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_alpha_decay {
                pub const M_FL_MIN_ALPHA: usize = 0x1C0; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_radius_decay {
                pub const M_FL_MIN_RADIUS: usize = 0x1C0; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_decay_maintain_count {
                pub const M_N_PARTICLES_TO_MAINTAIN: usize = 0x1C0; // int32
                pub const M_FL_DECAY_DELAY: usize = 0x1C4; // float32
                pub const M_N_SNAPSHOT_CONTROL_POINT: usize = 0x1C8; // int32
                pub const M_B_LIFESPAN_DECAY: usize = 0x1CC; // bool
                pub const M_FL_SCALE: usize = 0x1D0; // CParticleCollectionFloatInput
                pub const M_B_KILL_NEWEST: usize = 0x328; // bool
            }
            // Parent: c_particle_function_operator
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_decay_clamp_count {
                pub const M_N_COUNT: usize = 0x1C0; // CParticleCollectionFloatInput
            }
            // Parent: c_particle_function_operator
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_cull {
                pub const M_FL_CULL_PERC: usize = 0x1C0; // float32
                pub const M_FL_CULL_START: usize = 0x1C4; // float32
                pub const M_FL_CULL_END: usize = 0x1C8; // float32
                pub const M_FL_CULL_EXP: usize = 0x1CC; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_general_spin {
                pub const M_N_SPIN_RATE_DEGREES: usize = 0x1C0; // int32
                pub const M_N_SPIN_RATE_MIN_DEGREES: usize = 0x1C4; // int32
                pub const M_F_SPIN_RATE_STOP_TIME: usize = 0x1CC; // float32
            }
            // Parent: c_particle_function_operator
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_spin_update_base {
            }
            // Parent: c_general_spin
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_spin {
            }
            // Parent: c_spin_update_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_spin_update {
            }
            // Parent: c_general_spin
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_spin_yaw {
            }
            // Parent: c_particle_function_operator
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_op_interpolate_radius {
                pub const M_FL_START_TIME: usize = 0x1C0; // float32
                pub const M_FL_END_TIME: usize = 0x1C4; // float32
                pub const M_FL_START_SCALE: usize = 0x1C8; // float32
                pub const M_FL_END_SCALE: usize = 0x1CC; // float32
                pub const M_B_EASE_IN_AND_OUT: usize = 0x1D0; // bool
                pub const M_FL_BIAS: usize = 0x1D4; // float32
            }
        }
        // Module: pulse_system.dll
        // Classes count: 51
        // Enums count: 7
        pub mod pulse_system {
            // Alignment: 2
            // Members count: 61
            #[repr(i16)]
            pub enum PulseInstructionCodeT {
                Invalid = 0,
                ImmediateHalt = 1,
                ReturnVoid = 2,
                ReturnValue = 3,
                Nop = 4,
                Jump = 5,
                JumpCond = 6,
                ChunkLeap = 7,
                ChunkLeapCond = 8,
                PulseCallSync = 9,
                PulseCallAsyncFire = 10,
                CellInvoke = 11,
                LibraryInvoke = 12,
                SetVar = 13,
                GetVar = 14,
                GetConst = 15,
                SetRegisterDomainValue = 16,
                Copy = 17,
                Not = 18,
                Negate = 19,
                Add = 20,
                Sub = 21,
                Mul = 22,
                Div = 23,
                Mod = 24,
                Lt = 25,
                Lte = 26,
                Eq = 27,
                Ne = 28,
                And = 29,
                Or = 30,
                ConvertValue = 31,
                LastSerializedCode = 32,
                NegateInt = 33,
                NegateFloat = 34,
                AddInt = 35,
                AddFloat = 36,
                AddString = 37,
                SubInt = 38,
                SubFloat = 39,
                MulInt = 40,
                MulFloat = 41,
                DivInt = 42,
                DivFloat = 43,
                ModInt = 44,
                ModFloat = 45,
                LtInt = 46,
                LtFloat = 47,
                LteInt = 48,
                LteFloat = 49,
                EqBool = 50,
                EqInt = 51,
                EqFloat = 52,
                EqString = 53,
                EqEntityName = 54,
                NeBool = 55,
                NeInt = 56,
                NeFloat = 57,
                NeString = 58,
                NeEntityName = 59,
                GetConstInlineStorage = 60
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum PulseMethodCallModeT {
                SyncWaitForCompletion = 0,
                AsyncFireAndForget = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum PulseCursorExecResultT {
                Succeeded = 0,
                Canceled = 1,
                Failed = 2
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum PulseCursorCancelPriorityT {
                None = 0,
                CancelOnSucceeded = 1,
                SoftCancel = 2,
                HardCancel = 3
            }
            // Alignment: 4
            // Members count: 19
            #[repr(i32)]
            pub enum PulseValueTypeT {
                PvalInvalid = -1,
                PvalBool = 0,
                PvalInt = 1,
                PvalFloat = 2,
                PvalString = 3,
                PvalVec3 = 4,
                PvalTransform = 5,
                PvalColorRgb = 6,
                PvalEhandle = 7,
                PvalResource = 8,
                PvalSndevtGuid = 9,
                PvalEntityName = 10,
                PvalSchemaPtr = 11,
                PvalTypesafeInt = 12,
                PvalCursorFlow = 13,
                PvalAny = 14,
                PvalSchemaEnum = 15,
                PvalPanoramaPanelHandle = 16,
                PvalCount = 17
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum PulseTestEnumColorT {
                Black = 0,
                White = 1,
                Red = 2,
                Green = 3,
                Blue = 4
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum PulseTestEnumShapeT {
                Circle = 100,
                Square = 200,
                Triangle = 300
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            // MPropertyDescription
            pub mod c_pulse_cursor_funcs {
            }
            // Parent: None
            // Fields count: 1
            pub mod c_pulse_cell_wait_for_cursors_with_tag_base_cursor_state_t {
                pub const M_TAG_NAME: usize = 0x0; // CUtlSymbolLarge
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            // MPropertyDescription
            pub mod c_pulse_test_funcs_library_a {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod fake_entity_t_api {
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_register_info {
                pub const M_N_REG: usize = 0x0; // PulseRuntimeRegisterIndex_t
                pub const M_TYPE: usize = 0x8; // CPulseValueFullType
                pub const M_ORIGIN_NAME: usize = 0x18; // CKV3MemberNameWithStorage
                pub const M_N_WRITTEN_BY_INSTRUCTION: usize = 0x50; // int32
                pub const M_N_LAST_READ_BY_INSTRUCTION: usize = 0x54; // int32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_chunk {
                pub const M_INSTRUCTIONS: usize = 0x0; // CUtlLeanVector< PGDInstruction_t >
                pub const M_REGISTERS: usize = 0x10; // CUtlLeanVector< CPulse_RegisterInfo >
                pub const M_INSTRUCTION_EDITOR_I_DS: usize = 0x20; // CUtlLeanVector< PulseDocNodeID_t >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_variable {
                pub const M_NAME: usize = 0x0; // CUtlSymbolLarge
                pub const M_DESCRIPTION: usize = 0x8; // CUtlString
                pub const M_TYPE: usize = 0x10; // CPulseValueFullType
                pub const M_DEFAULT_VALUE: usize = 0x20; // KeyValues3
                pub const M_B_IS_PUBLIC: usize = 0x32; // bool
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_constant {
                pub const M_TYPE: usize = 0x0; // CPulseValueFullType
                pub const M_VALUE: usize = 0x10; // KeyValues3
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_public_output {
                pub const M_NAME: usize = 0x0; // CUtlSymbolLarge
                pub const M_DESCRIPTION: usize = 0x8; // CUtlString
                pub const M_PARAM_TYPE: usize = 0x10; // CPulseValueFullType
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_output_connection {
                pub const M_SOURCE_OUTPUT: usize = 0x0; // CUtlSymbolLarge
                pub const M_TARGET_ENTITY: usize = 0x8; // CUtlSymbolLarge
                pub const M_TARGET_INPUT: usize = 0x10; // CUtlSymbolLarge
                pub const M_PARAM: usize = 0x18; // CUtlSymbolLarge
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_invoke_binding {
                pub const M_REGISTER_MAP: usize = 0x0; // PulseRegisterMap_t
                pub const M_FUNC_NAME: usize = 0x20; // CUtlSymbolLarge
                pub const M_N_CELL_INDEX: usize = 0x28; // PulseRuntimeCellIndex_t
                pub const M_N_SRC_CHUNK: usize = 0x2C; // PulseRuntimeChunkIndex_t
                pub const M_N_SRC_INSTRUCTION: usize = 0x30; // int32
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_call_info {
                pub const M_PORT_NAME: usize = 0x0; // CUtlSymbolLarge
                pub const M_N_EDITOR_NODE_ID: usize = 0x8; // PulseDocNodeID_t
                pub const M_REGISTER_MAP: usize = 0x10; // PulseRegisterMap_t
                pub const M_CALL_METHOD_ID: usize = 0x30; // PulseDocNodeID_t
                pub const M_N_SRC_CHUNK: usize = 0x34; // PulseRuntimeChunkIndex_t
                pub const M_N_SRC_INSTRUCTION: usize = 0x38; // int32
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_graph_def {
                pub const M_DOMAIN_IDENTIFIER: usize = 0x8; // CUtlSymbolLarge
                pub const M_PARENT_MAP_NAME: usize = 0x10; // CUtlSymbolLarge
                pub const M_CHUNKS: usize = 0x18; // CUtlVector< CPulse_Chunk* >
                pub const M_CELLS: usize = 0x30; // CUtlVector< CPulseCell_Base* >
                pub const M_VARS: usize = 0x48; // CUtlVector< CPulse_Variable >
                pub const M_PUBLIC_OUTPUTS: usize = 0x60; // CUtlVector< CPulse_PublicOutput >
                pub const M_INVOKE_BINDINGS: usize = 0x78; // CUtlVector< CPulse_InvokeBinding* >
                pub const M_CALL_INFOS: usize = 0x90; // CUtlVector< CPulse_CallInfo* >
                pub const M_CONSTANTS: usize = 0xA8; // CUtlVector< CPulse_Constant >
                pub const M_OUTPUT_CONNECTIONS: usize = 0xC0; // CUtlVector< CPulse_OutputConnection* >
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            // MPropertyDescription
            pub mod c_pulse_mathlib {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            // MPropertyDescription
            pub mod c_pulse_test_script_lib {
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod pulse_register_map_t {
                pub const M_INPARAMS: usize = 0x0; // KeyValues3
                pub const M_OUTPARAMS: usize = 0x10; // KeyValues3
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod pgd_instruction_t {
                pub const M_N_CODE: usize = 0x0; // PulseInstructionCode_t
                pub const M_N_VAR: usize = 0x4; // PulseRuntimeVarIndex_t
                pub const M_N_REG0: usize = 0x8; // PulseRuntimeRegisterIndex_t
                pub const M_N_REG1: usize = 0xA; // PulseRuntimeRegisterIndex_t
                pub const M_N_REG2: usize = 0xC; // PulseRuntimeRegisterIndex_t
                pub const M_N_INVOKE_BINDING_INDEX: usize = 0x10; // PulseRuntimeInvokeIndex_t
                pub const M_N_CHUNK: usize = 0x14; // PulseRuntimeChunkIndex_t
                pub const M_N_DEST_INSTRUCTION: usize = 0x18; // int32
                pub const M_N_CALL_INFO_INDEX: usize = 0x1C; // PulseRuntimeCallInfoIndex_t
                pub const M_N_CONST_IDX: usize = 0x20; // PulseRuntimeConstantIndex_t
                pub const M_DOMAIN_VALUE: usize = 0x28; // CBufferString
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_outflow_connection {
                pub const M_SOURCE_OUTFLOW_NAME: usize = 0x0; // CUtlSymbolLarge
                pub const M_N_DEST_CHUNK: usize = 0x8; // PulseRuntimeChunkIndex_t
                pub const M_N_INSTRUCTION: usize = 0xC; // int32
            }
            // Parent: c_pulse_outflow_connection
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_resume_point {
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_base {
                pub const M_N_EDITOR_NODE_ID: usize = 0x8; // PulseDocNodeID_t
            }
            // Parent: c_pulse_cell_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_base_flow {
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_inflow_base_entrypoint {
                pub const M_ENTRY_CHUNK: usize = 0x48; // PulseRuntimeChunkIndex_t
                pub const M_REGISTER_MAP: usize = 0x50; // PulseRegisterMap_t
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_runtime_method_arg {
                pub const M_NAME: usize = 0x0; // CKV3MemberNameWithStorage
                pub const M_DESCRIPTION: usize = 0x38; // CUtlString
                pub const M_TYPE: usize = 0x40; // CPulseValueFullType
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_base_yielding_inflow {
            }
            // Parent: c_pulse_cell_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_base_value {
            }
            // Parent: c_pulse_cell_base_yielding_inflow
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            pub mod c_pulse_cell_inflow_wait {
                pub const M_WAKE_RESUME: usize = 0x48; // CPulse_ResumePoint
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_outflow_cycle_ordered_instance_state_t {
                pub const M_N_NEXT_INDEX: usize = 0x0; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_outflow_cycle_shuffled_instance_state_t {
                pub const M_SHUFFLE: usize = 0x0; // CUtlVectorFixedGrowable< uint8 >
                pub const M_N_NEXT_SHUFFLE: usize = 0x20; // int32
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod c_pulse_cell_outflow_test_random_yes_no {
                pub const M_YES: usize = 0x48; // CPulse_OutflowConnection
                pub const M_NO: usize = 0x58; // CPulse_OutflowConnection
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod c_pulse_cell_outflow_test_explicit_yes_no {
                pub const M_YES: usize = 0x48; // CPulse_OutflowConnection
                pub const M_NO: usize = 0x58; // CPulse_OutflowConnection
            }
            // Parent: c_pulse_cell_base_value
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod c_pulse_cell_value_test_value50 {
            }
            // Parent: c_pulse_cell_base_value
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            pub mod c_pulse_cell_value_random_int {
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_timeline_timeline_event_t {
                pub const M_FL_TIME_FROM_PREVIOUS: usize = 0x0; // float32
                pub const M_B_PAUSE_FOR_PREVIOUS_EVENTS: usize = 0x4; // bool
                pub const M_B_CALL_MODE_SYNC: usize = 0x5; // bool
                pub const M_EVENT_OUTFLOW: usize = 0x8; // CPulse_OutflowConnection
            }
            // Parent: None
            // Fields count: 7
            pub mod fake_entity_t {
                pub const M_N_HANDLE: usize = 0x0; // PulseTestEHandle_t
                pub const M_NAME: usize = 0x8; // CUtlString
                pub const M_CLASS: usize = 0x10; // CUtlString
                pub const M_B_DESTROYED: usize = 0x18; // bool
                pub const M_P_ASSOCIATED_GRAPH_INSTANCE: usize = 0x20; // CPulseGraphInstance_TestDomain*
                pub const M_B_FUNC_WAS_CALLED: usize = 0x28; // bool
                pub const M_F_VALUE: usize = 0x2C; // float32
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod c_pulse_graph_instance_test_domain {
                pub const M_B_IS_RUNNING_UNIT_TESTS: usize = 0xD8; // bool
                pub const M_B_EXPLICIT_TIME_STEPPING: usize = 0xD9; // bool
                pub const M_B_EXPECTING_TO_DESTROY_WITH_YIELDED_CURSORS: usize = 0xDA; // bool
                pub const M_N_NEXT_VALIDATE_INDEX: usize = 0xDC; // int32
                pub const M_TRACEPOINTS: usize = 0xE0; // CUtlVector< CUtlString >
                pub const M_B_TEST_YES_OR_NO_PATH: usize = 0xF8; // bool
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            pub mod c_pulse_cell_step_test_domain_tracepoint {
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            pub mod c_pulse_cell_step_test_domain_create_fake_entity {
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            pub mod c_pulse_cell_step_test_domain_destroy_fake_entity {
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            // MPulseEditorHeaderText
            pub mod c_pulse_cell_step_test_domain_ent_fire {
                pub const M_INPUT: usize = 0x48; // CUtlString
            }
            // Parent: c_pulse_cell_base_value
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            pub mod c_pulse_cell_val_test_domain_get_entity_name {
            }
            // Parent: c_pulse_cell_base_value
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            pub mod c_pulse_cell_val_test_domain_find_entity_by_name {
            }
            // Parent: c_pulse_cell_base_yielding_inflow
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            pub mod c_pulse_cell_test_wait_with_cursor_state {
                pub const M_WAKE_RESUME: usize = 0x48; // CPulse_ResumePoint
                pub const M_WAKE_CANCEL: usize = 0x58; // CPulse_ResumePoint
                pub const M_WAKE_FAIL: usize = 0x68; // CPulse_ResumePoint
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_test_wait_with_cursor_state_cursor_state_t {
                pub const FL_WAIT_VALUE: usize = 0x0; // float32
                pub const B_FAIL_ON_CANCEL: usize = 0x4; // bool
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            // MPropertyDescription
            pub mod c_pulse_test_funcs_derived_domain {
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            pub mod c_pulse_cell_test_no_inflow {
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            pub mod c_pulse_cell_test_multi_inflow_with_default {
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            pub mod c_pulse_cell_test_multi_inflow_no_default {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            pub mod c_pulse_graph_instance_turtle_graphics {
            }
            // Parent: c_pulse_cell_base_yielding_inflow
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_wait_for_cursors_with_tag_base {
                pub const M_N_CURSORS_ALLOWED_TO_WAIT: usize = 0x48; // int32
                pub const M_WAIT_COMPLETE: usize = 0x50; // CPulse_ResumePoint
            }
            // Parent: c_pulse_cell_wait_for_cursors_with_tag_base
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            pub mod c_pulse_cell_wait_for_cursors_with_tag {
                pub const M_B_TAG_SELF_WHEN_COMPLETE: usize = 0x60; // bool
                pub const M_N_DESIRED_KILL_PRIORITY: usize = 0x64; // PulseCursorCancelPriority_t
            }
            // Parent: c_pulse_cell_wait_for_cursors_with_tag_base
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPulseEditorHeaderIcon
            pub mod c_pulse_cell_cursor_queue {
                pub const M_N_CURSORS_ALLOWED_TO_RUN_PARALLEL: usize = 0x60; // int32
            }
        }
        // Module: rendersystemdx11.dll
        // Classes count: 3
        // Enums count: 5
        pub mod rendersystemdx11 {
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum RenderSlotTypeT {
                RenderSlotInvalid = -1,
                RenderSlotPerVertex = 0,
                RenderSlotPerInstance = 1
            }
            // Alignment: 4
            // Members count: 12
            #[repr(i32)]
            pub enum RenderBufferFlagsT {
                RenderBufferUsageVertexBuffer = 1,
                RenderBufferUsageIndexBuffer = 2,
                RenderBufferUsageShaderResource = 4,
                RenderBufferUsageUnorderedAccess = 8,
                RenderBufferByteaddressBuffer = 16,
                RenderBufferStructuredBuffer = 32,
                RenderBufferAppendConsumeBuffer = 64,
                RenderBufferUavCounter = 128,
                RenderBufferUavDrawIndirectArgs = 256,
                RenderBufferAccelerationStructure = 512,
                RenderBufferShaderBindingTable = 1024,
                RenderBufferPerFrameWriteOnce = 2048
            }
            // Alignment: 4
            // Members count: 13
            #[repr(i32)]
            pub enum RenderPrimitiveTypeT {
                RenderPrimPoints = 0,
                RenderPrimLines = 1,
                RenderPrimLinesWithAdjacency = 2,
                RenderPrimLineStrip = 3,
                RenderPrimLineStripWithAdjacency = 4,
                RenderPrimTriangles = 5,
                RenderPrimTrianglesWithAdjacency = 6,
                RenderPrimTriangleStrip = 7,
                RenderPrimTriangleStripWithAdjacency = 8,
                RenderPrimInstancedQuads = 9,
                RenderPrimHeterogenous = 10,
                RenderPrimComputeShader = 11,
                RenderPrimTypeCount = 12
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum InputLayoutVariationT {
                InputLayoutVariationDefault = 0,
                InputLayoutVariationStream1Instanceid = 1,
                InputLayoutVariationStream1InstanceidMorphVertId = 2,
                InputLayoutVariationMax = 3
            }
            // Alignment: 1
            // Members count: 8
            #[repr(i8)]
            pub enum RenderMultisampleTypeT {
                RenderMultisampleInvalid = -1,
                RenderMultisampleNone = 0,
                RenderMultisample2x = 1,
                RenderMultisample4x = 2,
                RenderMultisample6x = 3,
                RenderMultisample8x = 4,
                RenderMultisample16x = 5,
                RenderMultisampleTypeCount = 6
            }
            // Parent: None
            // Fields count: 7
            pub mod render_input_layout_field_t {
                pub const M_P_SEMANTIC_NAME: usize = 0x0; // uint8[32]
                pub const M_N_SEMANTIC_INDEX: usize = 0x20; // int32
                pub const M_FORMAT: usize = 0x24; // uint32
                pub const M_N_OFFSET: usize = 0x28; // int32
                pub const M_N_SLOT: usize = 0x2C; // int32
                pub const M_N_SLOT_TYPE: usize = 0x30; // RenderSlotType_t
                pub const M_N_INSTANCE_STEP_RATE: usize = 0x34; // int32
            }
            // Parent: None
            // Fields count: 4
            pub mod vs_input_signature_element_t {
                pub const M_P_NAME: usize = 0x0; // char[64]
                pub const M_P_SEMANTIC: usize = 0x40; // char[64]
                pub const M_P_D3D_SEMANTIC_NAME: usize = 0x80; // char[64]
                pub const M_N_D3D_SEMANTIC_INDEX: usize = 0xC0; // int32
            }
            // Parent: None
            // Fields count: 1
            pub mod vs_input_signature_t {
                pub const M_ELEMS: usize = 0x0; // CUtlVector< VsInputSignatureElement_t >
            }
        }
        // Module: resourcesystem.dll
        // Classes count: 57
        // Enums count: 2
        pub mod resourcesystem {
            // Alignment: 1
            // Members count: 2
            #[repr(i8)]
            pub enum FuseVariableAccessT {
                Writable = 0,
                ReadOnly = 1
            }
            // Alignment: 1
            // Members count: 9
            #[repr(i8)]
            pub enum FuseVariableTypeT {
                Invalid = 0,
                Bool = 1,
                Int8 = 2,
                Int16 = 3,
                Int32 = 4,
                Uint8 = 5,
                Uint16 = 6,
                Uint32 = 7,
                Float32 = 8
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod test_resource_t {
                pub const M_NAME: usize = 0x0; // CUtlString
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_test_resource_t {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_procedural_test_resource_t {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_manifest_test_resource_t {
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod manifest_test_resource_t {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_CHILD: usize = 0x8; // CStrongHandle< InfoForResourceTypeManifestTestResource_t >
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_cv_data_resource {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_i_pulse_graph_def {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_type_script_resource {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_java_script_resource {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_i_particle_system_definition {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_i_particle_snapshot {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_anim_data {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_animation_group {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_sequence_group_data {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_i_material2 {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_morph_set_data {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_render_mesh {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_model {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_texture_base {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_i_vector_graphic {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_cv_sound_event_script_list {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_cv_sound_stack_script_list {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_voice_container_base {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_cv_mix_list_resource {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_phys_aggregate_data {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_cv_phys_x_surface_properties_list {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_entity_lump {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_world_node {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_world_t {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_voxel_visibility {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_post_processing_resource {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_panorama_style {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_panorama_layout {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_panorama_dynamic_images {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_dota_item_definition_resource {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_cdota_patch_notes_list {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_cdota_novels_list {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_i_anim_graph_model_binding {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_choreo_scene_file_data {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_cdac_game_defs_data {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_composite_material_kit {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_gc_exportable_external_data {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_smart_prop {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_ccsgo_econ_item {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_response_rules_list {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_nm_skeleton {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_nm_clip {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_nm_graph {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_nm_graph_data_set {
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod constant_info_t {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_NAME_TOKEN: usize = 0x8; // CUtlStringToken
                pub const M_FL_VALUE: usize = 0xC; // float32
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod variable_info_t {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_NAME_TOKEN: usize = 0x8; // CUtlStringToken
                pub const M_N_INDEX: usize = 0xC; // FuseVariableIndex_t
                pub const M_N_NUM_COMPONENTS: usize = 0xE; // uint8
                pub const M_E_VAR_TYPE: usize = 0xF; // FuseVariableType_t
                pub const M_E_ACCESS: usize = 0x10; // FuseVariableAccess_t
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod function_info_t {
                pub const M_NAME: usize = 0x8; // CUtlString
                pub const M_NAME_TOKEN: usize = 0x10; // CUtlStringToken
                pub const M_N_PARAM_COUNT: usize = 0x14; // int32
                pub const M_N_INDEX: usize = 0x18; // FuseFunctionIndex_t
                pub const M_B_IS_PURE: usize = 0x1A; // bool
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_fuse_program {
                pub const M_PROGRAM_BUFFER: usize = 0x0; // CUtlVector< uint8 >
                pub const M_VARIABLES_READ: usize = 0x18; // CUtlVector< FuseVariableIndex_t >
                pub const M_VARIABLES_WRITTEN: usize = 0x30; // CUtlVector< FuseVariableIndex_t >
                pub const M_N_MAX_TEMP_VARS_USED: usize = 0x48; // int32
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_fuse_symbol_table {
                pub const M_CONSTANTS: usize = 0x0; // CUtlVector< ConstantInfo_t >
                pub const M_VARIABLES: usize = 0x18; // CUtlVector< VariableInfo_t >
                pub const M_FUNCTIONS: usize = 0x30; // CUtlVector< FunctionInfo_t >
                pub const M_CONSTANT_MAP: usize = 0x48; // CUtlHashtable< CUtlStringToken, int32 >
                pub const M_VARIABLE_MAP: usize = 0x68; // CUtlHashtable< CUtlStringToken, int32 >
                pub const M_FUNCTION_MAP: usize = 0x88; // CUtlHashtable< CUtlStringToken, int32 >
            }
            // Parent: None
            // Fields count: 2
            pub mod aabb_t {
                pub const M_V_MIN_BOUNDS: usize = 0x0; // 
                pub const M_V_MAX_BOUNDS: usize = 0xC; // 
            }
            // Parent: None
            // Fields count: 2
            pub mod packed_aabb_t {
                pub const M_N_PACKED_MIN: usize = 0x0; // uint32
                pub const M_N_PACKED_MAX: usize = 0x4; // uint32
            }
            // Parent: None
            // Fields count: 4
            pub mod four_quaternions {
                pub const X: usize = 0x0; // fltx4
                pub const Y: usize = 0x10; // fltx4
                pub const Z: usize = 0x20; // fltx4
                pub const W: usize = 0x30; // fltx4
            }
        }
        // Module: scenefilecache.dll
        // Classes count: 0
        // Enums count: 0
        pub mod scenefilecache {
        }
        // Module: scenesystem.dll
        // Classes count: 9
        // Enums count: 1
        pub mod scenesystem {
            // Alignment: 1
            // Members count: 4
            #[repr(i8)]
            pub enum DisableShadowsT {
                KDisableShadowsNone = 0,
                KDisableShadowsAll = 1,
                KDisableShadowsBaked = 2,
                KDisableShadowsRealtime = 3
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cssds_end_frame_view_info {
                pub const M_N_VIEW_ID: usize = 0x0; // uint64
                pub const M_VIEW_NAME: usize = 0x8; // CUtlString
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cssds_msg_end_frame {
                pub const M_VIEWS: usize = 0x0; // CUtlVector< CSSDSEndFrameViewInfo >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod scene_view_id_t {
                pub const M_N_VIEW_ID: usize = 0x0; // uint64
                pub const M_N_FRAME_COUNT: usize = 0x8; // uint64
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cssds_msg_view_render {
                pub const M_VIEW_ID: usize = 0x0; // SceneViewId_t
                pub const M_VIEW_NAME: usize = 0x10; // CUtlString
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cssds_msg_layer_base {
                pub const M_VIEW_ID: usize = 0x0; // SceneViewId_t
                pub const M_VIEW_NAME: usize = 0x10; // CUtlString
                pub const M_N_LAYER_INDEX: usize = 0x18; // int32
                pub const M_N_LAYER_ID: usize = 0x20; // uint64
                pub const M_LAYER_NAME: usize = 0x28; // CUtlString
                pub const M_DISPLAY_TEXT: usize = 0x30; // CUtlString
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cssds_msg_view_target {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_TEXTURE_ID: usize = 0x8; // uint64
                pub const M_N_WIDTH: usize = 0x10; // int32
                pub const M_N_HEIGHT: usize = 0x14; // int32
                pub const M_N_REQUESTED_WIDTH: usize = 0x18; // int32
                pub const M_N_REQUESTED_HEIGHT: usize = 0x1C; // int32
                pub const M_N_NUM_MIP_LEVELS: usize = 0x20; // int32
                pub const M_N_DEPTH: usize = 0x24; // int32
                pub const M_N_MULTISAMPLE_NUM_SAMPLES: usize = 0x28; // int32
                pub const M_N_FORMAT: usize = 0x2C; // int32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cssds_msg_view_target_list {
                pub const M_VIEW_ID: usize = 0x0; // SceneViewId_t
                pub const M_VIEW_NAME: usize = 0x10; // CUtlString
                pub const M_TARGETS: usize = 0x18; // CUtlVector< CSSDSMsg_ViewTarget >
            }
            // Parent: cssds_msg_layer_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cssds_msg_pre_layer {
            }
            // Parent: cssds_msg_layer_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cssds_msg_post_layer {
            }
        }
        // Module: schemasystem.dll
        // Classes count: 6
        // Enums count: 2
        pub mod schemasystem {
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ThreeStateT {
                TrsFalse = 0,
                TrsTrue = 1,
                TrsNone = 2
            }
            // Alignment: 1
            // Members count: 81
            #[repr(i8)]
            pub enum FieldtypeT {
                FieldVoid = 0,
                FieldFloat32 = 1,
                FieldString = 2,
                FieldVector = 3,
                FieldQuaternion = 4,
                FieldInt32 = 5,
                FieldBoolean = 6,
                FieldInt16 = 7,
                FieldCharacter = 8,
                FieldColor32 = 9,
                FieldEmbedded = 10,
                FieldCustom = 11,
                FieldClassptr = 12,
                FieldEhandle = 13,
                FieldPositionVector = 14,
                FieldTime = 15,
                FieldTick = 16,
                FieldSoundname = 17,
                FieldInput = 18,
                FieldFunction = 19,
                FieldVmatrix = 20,
                FieldVmatrixWorldspace = 21,
                FieldMatrix3x4Worldspace = 22,
                FieldInterval = 23,
                FieldUnused = 24,
                FieldVector2d = 25,
                FieldInt64 = 26,
                FieldVector4d = 27,
                FieldResource = 28,
                FieldTypeunknown = 29,
                FieldCstring = 30,
                FieldHscript = 31,
                FieldVariant = 32,
                FieldUint64 = 33,
                FieldFloat64 = 34,
                FieldPositiveintegerOrNull = 35,
                FieldHscriptNewInstance = 36,
                FieldUint32 = 37,
                FieldUtlstringtoken = 38,
                FieldQangle = 39,
                FieldNetworkOriginCellQuantizedVector = 40,
                FieldHmaterial = 41,
                FieldHmodel = 42,
                FieldNetworkQuantizedVector = 43,
                FieldNetworkQuantizedFloat = 44,
                FieldDirectionVectorWorldspace = 45,
                FieldQangleWorldspace = 46,
                FieldQuaternionWorldspace = 47,
                FieldHscriptLightbinding = 48,
                FieldV8Value = 49,
                FieldV8Object = 50,
                FieldV8Array = 51,
                FieldV8CallbackInfo = 52,
                FieldUtlstring = 53,
                FieldNetworkOriginCellQuantizedPositionVector = 54,
                FieldHrendertexture = 55,
                FieldHparticlesystemdefinition = 56,
                FieldUint8 = 57,
                FieldUint16 = 58,
                FieldCtransform = 59,
                FieldCtransformWorldspace = 60,
                FieldHpostprocessing = 61,
                FieldMatrix3x4 = 62,
                FieldShim = 63,
                FieldCmotiontransform = 64,
                FieldCmotiontransformWorldspace = 65,
                FieldAttachmentHandle = 66,
                FieldAmmoIndex = 67,
                FieldConditionId = 68,
                FieldAiScheduleBits = 69,
                FieldModifierHandle = 70,
                FieldRotationVector = 71,
                FieldRotationVectorWorldspace = 72,
                FieldHvdata = 73,
                FieldScale32 = 74,
                FieldStringAndToken = 75,
                FieldEngineTime = 76,
                FieldEngineTick = 77,
                FieldWorldGroupId = 78,
                FieldGlobalsymbol = 79,
                FieldTypecount = 80
            }
            // Parent: None
            // Fields count: 22
            pub mod c_schema_system_internal_registration {
                pub const M_VECTOR2_D: usize = 0x0; // Vector2D
                pub const M_VECTOR: usize = 0x8; // 
                pub const M_VECTOR_ALIGNED: usize = 0x20; // VectorAligned
                pub const M_QUATERNION: usize = 0x30; // Quaternion
                pub const M_Q_ANGLE: usize = 0x40; // QAngle
                pub const M_ROTATION_VECTOR: usize = 0x4C; // RotationVector
                pub const M_RADIAN_EULER: usize = 0x58; // RadianEuler
                pub const M_DEGREE_EULER: usize = 0x64; // DegreeEuler
                pub const M_QUATERNION_STORAGE: usize = 0x70; // QuaternionStorage
                pub const M_MATRIX3X4_T: usize = 0x80; // matrix3x4_t
                pub const M_MATRIX3X4A_T: usize = 0xB0; // matrix3x4a_t
                pub const M_COLOR: usize = 0xE0; // Color
                pub const M_VECTOR4_D: usize = 0xE4; // Vector4D
                pub const M_C_TRANSFORM: usize = 0x100; // CTransform
                pub const M_P_KEY_VALUES: usize = 0x120; // KeyValues*
                pub const M_C_UTL_BINARY_BLOCK: usize = 0x128; // CUtlBinaryBlock
                pub const M_C_UTL_STRING: usize = 0x140; // CUtlString
                pub const M_C_UTL_SYMBOL: usize = 0x148; // CUtlSymbol
                pub const M_STRING_TOKEN: usize = 0x14C; // CUtlStringToken
                pub const M_STRING_TOKEN_WITH_STORAGE: usize = 0x150; // CUtlStringTokenWithStorage
                pub const M_RESOURCE_TYPES: usize = 0x168; // CResourceArray< CResourcePointer< CResourceString > >
                pub const M_KV3: usize = 0x170; // KeyValues3
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_c_resource_manifest_internal {
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_example_schema_v_data_monomorphic {
                pub const M_N_EXAMPLE1: usize = 0x0; // int32
                pub const M_N_EXAMPLE2: usize = 0x4; // int32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_example_schema_v_data_polymorphic_base {
                pub const M_N_BASE: usize = 0x8; // int32
            }
            // Parent: c_example_schema_v_data_polymorphic_base
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_example_schema_v_data_polymorphic_derived_a {
                pub const M_N_DERIVED_A: usize = 0x10; // int32
            }
            // Parent: c_example_schema_v_data_polymorphic_base
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_example_schema_v_data_polymorphic_derived_b {
                pub const M_N_DERIVED_B: usize = 0x10; // int32
            }
        }
        // Module: server.dll
        // Classes count: 233
        // Enums count: 128
        pub mod server {
            // Alignment: 4
            // Members count: 19
            #[repr(i32)]
            pub enum NavAttributeEnum {
                NavMeshAvoid = 128,
                NavMeshStairs = 4096,
                NavMeshNonZup = 32768,
                NavMeshShortHeight = 65536,
                NavMeshCrouch = 65536,
                NavMeshJump = 2,
                NavMeshPrecise = 4,
                NavMeshNoJump = 8,
                NavMeshStop = 16,
                NavMeshRun = 32,
                NavMeshWalk = 64,
                NavMeshTransient = 256,
                NavMeshDontHide = 512,
                NavMeshStand = 1024,
                NavMeshNoHostages = 2048,
                NavMeshNoMerge = 8192,
                NavMeshObstacleTop = 16384,
                NavAttrFirstGameIndex = 19,
                NavAttrLastIndex = 31
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum NavDirType {
                North = 0,
                East = 1,
                South = 2,
                West = 3,
                NumNavDirTypeDirections = 4
            }
            // Alignment: 1
            // Members count: 3
            #[repr(i8)]
            pub enum SequenceFinishNotifyStateT {
                EDoNotNotify = 0,
                ENotifyWhenFinished = 1,
                ENotifyTriggered = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum PointTemplateOwnerSpawnGroupTypeT {
                InsertIntoPointTemplateSpawnGroup = 0,
                InsertIntoCurrentlyActiveSpawnGroup = 1,
                InsertIntoNewlyCreatedSpawnGroup = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum PointTemplateClientOnlyEntityBehaviorT {
                CreateForCurrentlyConnectedClientsOnly = 0,
                CreateForClientsWhoConnectLater = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum PerformanceModeT {
                PmNormal = 0,
                PmNoGibs = 1,
                PmFullGibs = 2,
                PmReducedGibs = 3
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum AmmoPositionT {
                AmmoPositionInvalid = -1,
                AmmoPositionPrimary = 0,
                AmmoPositionSecondary = 1,
                AmmoPositionCount = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ChatIgnoreTypeT {
                ChatIgnoreNone = 0,
                ChatIgnoreAll = 1,
                ChatIgnoreTeam = 2
            }
            // Alignment: 1
            // Members count: 3
            #[repr(i8)]
            pub enum FixAngleSetT {
                None = 0,
                Absolute = 1,
                Relative = 2
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum CommandExecModeT {
                ExecManual = 0,
                ExecLevelstart = 1,
                ExecPeriodic = 2,
                ExecModesCount = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum CommandEntitySpecTypeT {
                SpecSearch = 0,
                SpecTypesCount = 1
            }
            // Alignment: 4
            // Members count: 43
            #[repr(i32)]
            pub enum GameAnimEventIndexT {
                AeEmpty = 0,
                AeClPlaysound = 1,
                AeClPlaysoundAttachment = 2,
                AeClPlaysoundPosition = 3,
                AeSvPlaysound = 4,
                AeClStopsound = 5,
                AeClPlaysoundLooping = 6,
                AeClCreateParticleEffect = 7,
                AeClStopParticleEffect = 8,
                AeClCreateParticleEffectCfg = 9,
                AeSvCreateParticleEffectCfg = 10,
                AeSvStopParticleEffect = 11,
                AeFootstep = 12,
                AeRagdoll = 13,
                AeClStopRagdollControl = 14,
                AeClEnableBodygroup = 15,
                AeClDisableBodygroup = 16,
                AeBodygroupSetValue = 17,
                AeClBodygroupSetValueCmodelWpn = 18,
                AeWeaponPerformAttack = 19,
                AeFireInput = 20,
                AeClClothAttr = 21,
                AeClClothGroundOffset = 22,
                AeClClothStiffen = 23,
                AeClClothEffect = 24,
                AeClCreateAnimScopeProp = 25,
                AePulseGraph = 26,
                AePulseGraphLookat = 27,
                AePulseGraphAimat = 28,
                AePulseGraphIklockleftarm = 29,
                AePulseGraphIklockrightarm = 30,
                AeClWeaponTransitionIntoHand = 31,
                AeClBodygroupSetToClip = 32,
                AeClBodygroupSetToNextclip = 33,
                AeSvShowSilencer = 34,
                AeSvAttachSilencerComplete = 35,
                AeSvHideSilencer = 36,
                AeSvDetachSilencerComplete = 37,
                AeClEjectMag = 38,
                AeWpnCompleteReload = 39,
                AeWpnHealthshotInject = 40,
                AeClC4ScreenText = 41,
                AeGrenadeThrowComplete = 42
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum ObserverModeT {
                ObsModeNone = 0,
                ObsModeFixed = 1,
                ObsModeInEye = 2,
                ObsModeChase = 3,
                ObsModeRoaming = 4,
                ObsModeDirected = 5,
                NumObserverModes = 6
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ObserverInterpStateT {
                ObserverInterpNone = 0,
                ObserverInterpTraveling = 1,
                ObserverInterpSettling = 2
            }
            // Alignment: 4
            // Members count: 27
            #[repr(i32)]
            pub enum RumbleEffectT {
                RumbleInvalid = -1,
                RumbleStopAll = 0,
                RumblePistol = 1,
                Rumble357 = 2,
                RumbleSmg1 = 3,
                RumbleAr2 = 4,
                RumbleShotgunSingle = 5,
                RumbleShotgunDouble = 6,
                RumbleAr2AltFire = 7,
                RumbleRpgMissile = 8,
                RumbleCrowbarSwing = 9,
                RumbleAirboatGun = 10,
                RumbleJeepEngineLoop = 11,
                RumbleFlatLeft = 12,
                RumbleFlatRight = 13,
                RumbleFlatBoth = 14,
                RumbleDmgLow = 15,
                RumbleDmgMed = 16,
                RumbleDmgHigh = 17,
                RumbleFallLong = 18,
                RumbleFallShort = 19,
                RumblePhyscannonOpen = 20,
                RumblePhyscannonPunt = 21,
                RumblePhyscannonLow = 22,
                RumblePhyscannonMedium = 23,
                RumblePhyscannonHigh = 24,
                NumRumbleEffects = 25
            }
            // Alignment: 4
            // Members count: 25
            #[repr(i32)]
            pub enum WeaponSoundT {
                WeaponSoundEmpty = 0,
                WeaponSoundSecondaryEmpty = 1,
                WeaponSoundSingle = 2,
                WeaponSoundSecondaryAttack = 3,
                WeaponSoundMeleeMiss = 4,
                WeaponSoundMeleeHit = 5,
                WeaponSoundMeleeHitWorld = 6,
                WeaponSoundMeleeHitPlayer = 7,
                WeaponSoundMeleeHitNpc = 8,
                WeaponSoundSpecial1 = 9,
                WeaponSoundSpecial2 = 10,
                WeaponSoundSpecial3 = 11,
                WeaponSoundNearlyempty = 12,
                WeaponSoundImpact = 13,
                WeaponSoundReflect = 14,
                WeaponSoundSecondaryImpact = 15,
                WeaponSoundSecondaryReflect = 16,
                WeaponSoundReload = 17,
                WeaponSoundSingleAccurate = 18,
                WeaponSoundZoomIn = 19,
                WeaponSoundZoomOut = 20,
                WeaponSoundMousePressed = 21,
                WeaponSoundDrop = 22,
                WeaponSoundRadioUse = 23,
                WeaponSoundNumTypes = 24
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum AmmoFlagsT {
                AmmoForceDropIfCarried = 1,
                AmmoReserveStaysWithWeapon = 2,
                AmmoFlagMax = 2
            }
            // Alignment: 4
            // Members count: 15
            #[repr(i32)]
            pub enum TakeDamageFlagsT {
                DflagNone = 0,
                DflagSuppressHealthChanges = 1,
                DflagSuppressPhysicsForce = 2,
                DflagSuppressEffects = 4,
                DflagPreventDeath = 8,
                DflagForceDeath = 16,
                DflagAlwaysGib = 32,
                DflagNeverGib = 64,
                DflagRemoveNoRagdoll = 128,
                DflagSuppressDamageModification = 256,
                DflagAlwaysFireDamageEvents = 512,
                DflagRadiusDmg = 1024,
                DmgLastdflag = 1024,
                DflagIgnoreArmor = 2048,
                DflagSuppressUtilremove = 4096
            }
            // Alignment: 4
            // Members count: 24
            #[repr(i32)]
            pub enum DamageTypesT {
                DmgGeneric = 0,
                DmgCrush = 1,
                DmgBullet = 2,
                DmgSlash = 4,
                DmgBurn = 8,
                DmgVehicle = 16,
                DmgFall = 32,
                DmgBlast = 64,
                DmgClub = 128,
                DmgShock = 256,
                DmgSonic = 512,
                DmgEnergybeam = 1024,
                DmgDrown = 16384,
                DmgPoison = 32768,
                DmgRadiation = 65536,
                DmgDrownrecover = 131072,
                DmgAcid = 262144,
                DmgPhysgun = 1048576,
                DmgDissolve = 2097152,
                DmgBlastSurface = 4194304,
                DmgBuckshot = 16777216,
                DmgLastgenericflag = 16777216,
                DmgHeadshot = 33554432,
                DmgDangerzone = 67108864
            }
            // Alignment: 4
            // Members count: 17
            #[repr(i32)]
            pub enum BaseExplosionTypesT {
                ExplosionTypeDefault = 0,
                ExplosionTypeGrenade = 1,
                ExplosionTypeMolotov = 2,
                ExplosionTypeFireworks = 3,
                ExplosionTypeGascan = 4,
                ExplosionTypeGascylinder = 5,
                ExplosionTypeExplosivebarrel = 6,
                ExplosionTypeElectrical = 7,
                ExplosionTypeEmp = 8,
                ExplosionTypeShrapnel = 9,
                ExplosionTypeSmokegrenade = 10,
                ExplosionTypeFlashbang = 11,
                ExplosionTypeTripmine = 12,
                ExplosionTypeIce = 13,
                ExplosionTypeNone = 14,
                ExplosionTypeCustom = 15,
                ExplosionTypeCount = 16
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum HierarchyTypeT {
                HierarchyNone = 0,
                HierarchyBoneMerge = 1,
                HierarchyAttachment = 2,
                HierarchyAbsorigin = 3,
                HierarchyBone = 4,
                HierarchyTypeCount = 5
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum CanPlaySequenceT {
                CannotPlay = 0,
                CanPlayNow = 1,
                CanPlayEnqueued = 2
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum ScriptedMoveToT {
                CineMovetoWait = 0,
                CineMovetoWalk = 1,
                CineMovetoRun = 2,
                CineMovetoCustom = 3,
                CineMovetoTeleport = 4,
                CineMovetoWaitFacing = 5
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ScriptedOnDeathT {
                SsOndeathNotApplicable = -1,
                SsOndeathUndefined = 0,
                SsOndeathRagdoll = 1,
                SsOndeathAnimatedDeath = 2
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum IChoreoServicesScriptStateT {
                ScriptPlaying = 0,
                ScriptWait = 1,
                ScriptPostIdle = 2,
                ScriptCleanup = 3,
                ScriptWalkToMark = 4,
                ScriptRunToMark = 5,
                ScriptCustomMoveToMark = 6
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum IChoreoServicesChoreoStateT {
                StatePreScript = 0,
                StateWaitForScript = 1,
                StateWalkToMark = 2,
                StateSynchronizeScript = 3,
                StatePlayScript = 4,
                StatePlayScriptPostIdle = 5,
                StatePlayScriptPostIdleDone = 6
            }
            // Alignment: 8
            // Members count: 21
            #[repr(i64)]
            pub enum InputBitMaskT {
                InNone = 0,
                InAll = -1,
                InAttack = 1,
                InJump = 2,
                InDuck = 4,
                InForward = 8,
                InBack = 16,
                InUse = 32,
                InTurnleft = 128,
                InTurnright = 256,
                InMoveleft = 512,
                InMoveright = 1024,
                InAttack2 = 2048,
                InReload = 8192,
                InSpeed = 65536,
                InJoyautosprint = 131072,
                InFirstModSpecificBit = 4294967296,
                InUseorreload = 4294967296,
                InScore = 8589934592,
                InZoom = 17179869184,
                InLookAtWeapon = 34359738368
            }
            // Alignment: 4
            // Members count: 9
            #[repr(i32)]
            pub enum EInButtonState {
                InButtonUp = 0,
                InButtonDown = 1,
                InButtonDownUp = 2,
                InButtonUpDown = 3,
                InButtonUpDownUp = 4,
                InButtonDownUpDown = 5,
                InButtonDownUpDownUp = 6,
                InButtonUpDownUpDown = 7,
                InButtonStateCount = 8
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum ShakeCommandT {
                ShakeStart = 0,
                ShakeStop = 1,
                ShakeAmplitude = 2,
                ShakeFrequency = 3,
                ShakeStartRumbleonly = 4,
                ShakeStartNorumble = 5
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum TimelineCompressionT {
                TimelineCompressionSum = 0,
                TimelineCompressionCountPerInterval = 1,
                TimelineCompressionAverage = 2,
                TimelineCompressionAverageBlend = 3,
                TimelineCompressionTotal = 4
            }
            // Alignment: 8
            // Members count: 39
            #[repr(i64)]
            pub enum DebugOverlayBitsT {
                OverlayTextBit = 1,
                OverlayNameBit = 2,
                OverlayBboxBit = 4,
                OverlayPivotBit = 8,
                OverlayMessageBit = 16,
                OverlayAbsboxBit = 32,
                OverlayRboxBit = 64,
                OverlayShowBlockslos = 128,
                OverlayAttachmentsBit = 256,
                OverlayInterpolatedAttachmentsBit = 512,
                OverlayInterpolatedPivotBit = 1024,
                OverlaySkeletonBit = 2048,
                OverlayInterpolatedSkeletonBit = 4096,
                OverlayTriggerBoundsBit = 8192,
                OverlayHitboxBit = 16384,
                OverlayInterpolatedHitboxBit = 32768,
                OverlayAutoaimBit = 65536,
                OverlayNpcSelectedBit = 131072,
                OverlayJointInfoBit = 262144,
                OverlayNpcRouteBit = 524288,
                OverlayVisibilityTracesBit = 1048576,
                OverlayNpcEnemiesBit = 4194304,
                OverlayNpcConditionsBit = 8388608,
                OverlayNpcCombatBit = 16777216,
                OverlayNpcTaskBit = 33554432,
                OverlayNpcBodylocations = 67108864,
                OverlayNpcViewconeBit = 134217728,
                OverlayNpcKillBit = 268435456,
                OverlayWcChangeEntity = 536870912,
                OverlayBuddhaMode = 1073741824,
                OverlayNpcSteeringRegulations = 2147483648,
                OverlayNpcTaskTextBit = 4294967296,
                OverlayPropDebug = 8589934592,
                OverlayNpcRelationBit = 17179869184,
                OverlayViewoffset = 34359738368,
                OverlayVcollideWireframeBit = 68719476736,
                OverlayActornameBit = 274877906944,
                OverlayNpcConditionsTextBit = 549755813888,
                OverlayNpcAbilityRangeDebugBit = 1099511627776
            }
            // Alignment: 1
            // Members count: 14
            #[repr(i8)]
            pub enum MoveTypeT {
                MovetypeNone = 0,
                MovetypeObsolete = 1,
                MovetypeWalk = 2,
                MovetypeFly = 3,
                MovetypeFlygravity = 4,
                MovetypeVphysics = 5,
                MovetypePush = 6,
                MovetypeNoclip = 7,
                MovetypeObserver = 8,
                MovetypeLadder = 9,
                MovetypeCustom = 10,
                MovetypeLast = 11,
                MovetypeInvalid = 11,
                MovetypeMaxBits = 5
            }
            // Alignment: 1
            // Members count: 6
            #[repr(i8)]
            pub enum MoveCollideT {
                MovecollideDefault = 0,
                MovecollideFlyBounce = 1,
                MovecollideFlyCustom = 2,
                MovecollideFlySlide = 3,
                MovecollideCount = 4,
                MovecollideMaxBits = 3
            }
            // Alignment: 1
            // Members count: 9
            #[repr(i8)]
            pub enum SolidTypeT {
                SolidNone = 0,
                SolidBsp = 1,
                SolidBbox = 2,
                SolidObb = 3,
                SolidSphere = 4,
                SolidPoint = 5,
                SolidVphysics = 6,
                SolidCapsule = 7,
                SolidLast = 8
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum BrushSoliditiesE {
                BrushsolidToggle = 0,
                BrushsolidNever = 1,
                BrushsolidAlways = 2
            }
            // Alignment: 1
            // Members count: 13
            #[repr(i8)]
            pub enum RenderModeT {
                KRenderNormal = 0,
                KRenderTransColor = 1,
                KRenderTransTexture = 2,
                KRenderGlow = 3,
                KRenderTransAlpha = 4,
                KRenderTransAdd = 5,
                KRenderEnvironmental = 6,
                KRenderTransAddFrameBlend = 7,
                KRenderTransAlphaAdd = 8,
                KRenderWorldGlow = 9,
                KRenderNone = 10,
                KRenderDevVisualizer = 11,
                KRenderModeCount = 12
            }
            // Alignment: 1
            // Members count: 20
            #[repr(i8)]
            pub enum RenderFxT {
                KRenderFxNone = 0,
                KRenderFxPulseSlow = 1,
                KRenderFxPulseFast = 2,
                KRenderFxPulseSlowWide = 3,
                KRenderFxPulseFastWide = 4,
                KRenderFxFadeSlow = 5,
                KRenderFxFadeFast = 6,
                KRenderFxSolidSlow = 7,
                KRenderFxSolidFast = 8,
                KRenderFxStrobeSlow = 9,
                KRenderFxStrobeFast = 10,
                KRenderFxStrobeFaster = 11,
                KRenderFxFlickerSlow = 12,
                KRenderFxFlickerFast = 13,
                KRenderFxNoDissipation = 14,
                KRenderFxFadeOut = 15,
                KRenderFxFadeIn = 16,
                KRenderFxPulseFastWider = 17,
                KRenderFxGlowShell = 18,
                KRenderFxMax = 19
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum CrrResponseResponseEnumT {
                MaxResponseName = 192,
                MaxRuleName = 128
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum LessonPanelLayoutFileTypesT {
                LayoutHandDefault = 0,
                LayoutWorldDefault = 1,
                LayoutCustom = 2
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum TouchT {
                TouchNone = 0,
                TouchPlayerOnly = 1,
                TouchNpcOnly = 2,
                TouchPlayerOrNpc = 3,
                TouchPlayerOrNpcOrPhysicsprop = 4
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ScriptedMoveTypeT {
                ScriptedMovetypeNone = 0,
                ScriptedMovetypeToWithDuration = 1,
                ScriptedMovetypeToWithMovespeed = 2,
                ScriptedMovetypeSweepToAtMovementSpeed = 3
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum ForcedCrouchStateT {
                ForcedcrouchNone = 0,
                ForcedcrouchCrouched = 1,
                ForcedcrouchUncrouched = 2
            }
            // Alignment: 4
            // Members count: 12
            #[repr(i32)]
            pub enum HullT {
                HullHuman = 0,
                HullSmallCentered = 1,
                HullWideHuman = 2,
                HullTiny = 3,
                HullMedium = 4,
                HullTinyCentered = 5,
                HullLarge = 6,
                HullLargeCentered = 7,
                HullMediumTall = 8,
                HullSmall = 9,
                NumHulls = 10,
                HullNone = 11
            }
            // Alignment: 4
            // Members count: 1
            #[repr(i32)]
            pub enum NavpropertiesT {
                NavIgnore = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum CLogicBranchListLogicBranchListenerLastStateT {
                LogicBranchListenerNotInit = 0,
                LogicBranchListenerAllTrue = 1,
                LogicBranchListenerAllFalse = 2,
                LogicBranchListenerMixed = 3
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum EntFinderMethodT {
                EntFindMethodNearest = 0,
                EntFindMethodFarthest = 1,
                EntFindMethodRandom = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ValueRemapperInputTypeT {
                InputTypePlayerShootPosition = 0,
                InputTypePlayerShootPositionAroundAxis = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ValueRemapperOutputTypeT {
                OutputTypeAnimationCycle = 0,
                OutputTypeRotationX = 1,
                OutputTypeRotationY = 2,
                OutputTypeRotationZ = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ValueRemapperHapticsTypeT {
                HaticsTypeDefault = 0,
                HaticsTypeNone = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum ValueRemapperMomentumTypeT {
                MomentumTypeNone = 0,
                MomentumTypeFriction = 1,
                MomentumTypeSpringTowardSnapValue = 2,
                MomentumTypeSpringAwayFromSnapValue = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ValueRemapperRatchetTypeT {
                RatchetTypeAbsolute = 0,
                RatchetTypeEachEngage = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum PointWorldTextJustifyHorizontalT {
                PointWorldTextJustifyHorizontalLeft = 0,
                PointWorldTextJustifyHorizontalCenter = 1,
                PointWorldTextJustifyHorizontalRight = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum PointWorldTextJustifyVerticalT {
                PointWorldTextJustifyVerticalBottom = 0,
                PointWorldTextJustifyVerticalCenter = 1,
                PointWorldTextJustifyVerticalTop = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum PointWorldTextReorientModeT {
                PointWorldTextReorientNone = 0,
                PointWorldTextReorientAroundUp = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum DoorCheckE {
                DoorCheckForward = 0,
                DoorCheckBackward = 1,
                DoorCheckFull = 2
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum PropDoorRotatingSpawnPosT {
                DoorSpawnClosed = 0,
                DoorSpawnOpenForward = 1,
                DoorSpawnOpenBack = 2,
                DoorSpawnAjar = 3
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum PropDoorRotatingOpenDirectionE {
                DoorRotatingOpenBothWays = 0,
                DoorRotatingOpenForward = 1,
                DoorRotatingOpenBackward = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum SceneOnPlayerDeathT {
                SceneOnplayerdeathDoNothing = 0,
                SceneOnplayerdeathCancel = 1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ScriptedConflictResponseT {
                SsConflictEnqueue = 0,
                SsConflictInterrupt = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum TrainCode {
                TrainSafe = 0,
                TrainBlocking = 1,
                TrainFollowing = 2
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum NpcFollowFormationT {
                Default = -1,
                CloseCircle = 0,
                WideCircle = 1,
                MediumCircle = 5,
                Sidekick = 6
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum NpcLookTypeT {
                Chest = 0,
                Head = 1,
                Eyes = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum SoundEventStartTypeT {
                SoundeventStartPlayer = 0,
                SoundeventStartWorld = 1,
                SoundeventStartEntity = 2
            }
            // Alignment: 4
            // Members count: 8
            #[repr(i32)]
            pub enum ToggleState {
                TsAtTop = 0,
                TsAtBottom = 1,
                TsGoingUp = 2,
                TsGoingDown = 3,
                DoorOpen = 0,
                DoorClosed = 1,
                DoorOpening = 2,
                DoorClosing = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum FuncDoorSpawnPosT {
                FuncDoorSpawnClosed = 0,
                FuncDoorSpawnOpen = 1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum FilterT {
                FilterAnd = 0,
                FilterOr = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum Explosions {
                ExpRandom = 0,
                ExpDirected = 1,
                ExpUsePrecise = 2
            }
            // Alignment: 4
            // Members count: 12
            #[repr(i32)]
            pub enum Materials {
                MatGlass = 0,
                MatWood = 1,
                MatMetal = 2,
                MatFlesh = 3,
                MatCinderBlock = 4,
                MatCeilingTile = 5,
                MatComputer = 6,
                MatUnbreakableGlass = 7,
                MatRocks = 8,
                MatWeb = 9,
                MatNone = 10,
                MatLastMaterial = 11
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum EOverrideBlockLosT {
                BlockLosDefault = 0,
                BlockLosForceFalse = 1,
                BlockLosForceTrue = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum MoveLinearAuthoredPosT {
                MovelinearAuthoredAtStartPosition = 0,
                MovelinearAuthoredAtOpenPosition = 1,
                MovelinearAuthoredAtClosedPosition = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum TrackOrientationTypeT {
                TrackOrientationFixed = 0,
                TrackOrientationFacePath = 1,
                TrackOrientationFacePathAngles = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum SimpleConstraintSoundProfileSimpleConstraintsSoundProfileKeypointsT {
                KMinThreshold = 0,
                KMinFull = 1,
                KHighwater = 2
            }
            // Alignment: 4
            // Members count: 23
            #[repr(i32)]
            pub enum SoundFlagsT {
                SoundNone = 0,
                SoundCombat = 1,
                SoundWorld = 2,
                SoundPlayer = 4,
                SoundDanger = 8,
                SoundBulletImpact = 16,
                SoundThumper = 32,
                SoundPhysicsDanger = 64,
                SoundMoveAway = 128,
                SoundPlayerVehicle = 256,
                SoundGlassBreak = 512,
                SoundPhysicsObject = 1024,
                SoundContextGunfire = 1048576,
                SoundContextCombineOnly = 2097152,
                SoundContextReactToSource = 4194304,
                SoundContextExplosion = 8388608,
                SoundContextExcludeCombine = 16777216,
                SoundContextDangerApproach = 33554432,
                SoundContextAlliesOnly = 67108864,
                SoundContextPanicNpcs = 134217728,
                AllContexts = -1048576,
                AllScents = 0,
                AllSounds = 1048575
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum TrainVelocityTypeT {
                TrainVelocityInstantaneous = 0,
                TrainVelocityLinearBlend = 1,
                TrainVelocityEaseInEaseOut = 2
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum TrainOrientationTypeT {
                TrainOrientationFixed = 0,
                TrainOrientationAtPathTracks = 1,
                TrainOrientationLinearBlend = 2,
                TrainOrientationEaseInEaseOut = 3
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum BeamTypeT {
                BeamInvalid = 0,
                BeamPoints = 1,
                BeamEntpoint = 2,
                BeamEnts = 3,
                BeamHose = 4,
                BeamSpline = 5,
                BeamLaser = 6
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum BeamClipStyleT {
                KNoclip = 0,
                KGeoclip = 1,
                KModelclip = 2,
                KBeamclipstyleNumbits = 2
            }
            // Alignment: 1
            // Members count: 10
            #[repr(i8)]
            pub enum SurroundingBoundsTypeT {
                UseObbCollisionBounds = 0,
                UseBestCollisionBounds = 1,
                UseHitboxes = 2,
                UseSpecifiedBounds = 3,
                UseGameCode = 4,
                UseRotationExpandedBounds = 5,
                UseRotationExpandedOrientedBounds = 6,
                UseCollisionBoundsNeverVphysics = 7,
                UseRotationExpandedSequenceBounds = 8,
                SurroundingTypeBitCount = 3
            }
            // Alignment: 1
            // Members count: 2
            #[repr(i8)]
            pub enum ShatterPanelMode {
                ShatterGlass = 0,
                ShatterDrywall = 1
            }
            // Alignment: 1
            // Members count: 5
            #[repr(i8)]
            pub enum ShatterDamageCause {
                ShatterdamageBullet = 0,
                ShatterdamageMelee = 1,
                ShatterdamageThrown = 2,
                ShatterdamageScript = 3,
                ShatterdamageExplosive = 4
            }
            // Alignment: 1
            // Members count: 5
            #[repr(i8)]
            pub enum ShatterGlassStressType {
                ShatterglassBlunt = 0,
                ShatterglassBallistic = 1,
                ShatterglassPulse = 2,
                ShatterdrywallChunks = 3,
                ShatterglassExplosive = 4
            }
            // Alignment: 1
            // Members count: 3
            #[repr(i8)]
            pub enum OnFrame {
                OnframeUnknown = 0,
                OnframeTrue = 1,
                OnframeFalse = 2
            }
            // Alignment: 1
            // Members count: 2
            #[repr(i8)]
            pub enum ShardSolidT {
                ShardSolid = 0,
                ShardDebris = 1
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum AnimLoopModeT {
                AnimLoopModeInvalid = -1,
                AnimLoopModeNotLooping = 0,
                AnimLoopModeLooping = 1,
                AnimLoopModeUseSequenceSettings = 2,
                AnimLoopModeCount = 3
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum EntitySubclassScopeT {
                SubclassScopeNone = -1,
                SubclassScopePrecipitation = 0,
                SubclassScopePlayerWeapons = 1,
                SubclassScopeCount = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum SubclassVDataChangeTypeT {
                SubclassVdataCreated = 0,
                SubclassVdataSubclassChanged = 1,
                SubclassVdataReloaded = 2
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum PlayerConnectedState {
                PlayerNeverConnected = -1,
                PlayerConnected = 0,
                PlayerConnecting = 1,
                PlayerReconnecting = 2,
                PlayerDisconnecting = 3,
                PlayerDisconnected = 4,
                PlayerReserved = 5
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum WeaponSwitchReasonT {
                EDrawn = 0,
                EEquipped = 1,
                EUserInitiatedSwitchToLast = 2
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum WeaponAttackTypeT {
                EInvalid = -1,
                EPrimary = 0,
                ESecondary = 1,
                ECount = 2
            }
            // Alignment: 4
            // Members count: 35
            #[repr(i32)]
            pub enum VoteCreateFailedT {
                VoteFailedGeneric = 0,
                VoteFailedTransitioningPlayers = 1,
                VoteFailedRateExceeded = 2,
                VoteFailedYesMustExceedNo = 3,
                VoteFailedQuorumFailure = 4,
                VoteFailedIssueDisabled = 5,
                VoteFailedMapNotFound = 6,
                VoteFailedMapNameRequired = 7,
                VoteFailedFailedRecently = 8,
                VoteFailedTeamCantCall = 9,
                VoteFailedWaitingforplayers = 10,
                VoteFailedPlayernotfound = 11,
                VoteFailedCannotKickAdmin = 12,
                VoteFailedScrambleInProgress = 13,
                VoteFailedSpectator = 14,
                VoteFailedFailedRecentKick = 15,
                VoteFailedFailedRecentChangemap = 16,
                VoteFailedFailedRecentSwapteams = 17,
                VoteFailedFailedRecentScrambleteams = 18,
                VoteFailedFailedRecentRestart = 19,
                VoteFailedSwapInProgress = 20,
                VoteFailedDisabled = 21,
                VoteFailedNextlevelSet = 22,
                VoteFailedTooEarlySurrender = 23,
                VoteFailedMatchPaused = 24,
                VoteFailedMatchNotPaused = 25,
                VoteFailedNotInWarmup = 26,
                VoteFailedNot10Players = 27,
                VoteFailedTimeoutActive = 28,
                VoteFailedTimeoutInactive = 29,
                VoteFailedTimeoutExhausted = 30,
                VoteFailedCantRoundEnd = 31,
                VoteFailedRematch = 32,
                VoteFailedContinue = 33,
                VoteFailedMax = 34
            }
            // Alignment: 1
            // Members count: 9
            #[repr(i8)]
            pub enum ItemFlagTypesT {
                ItemFlagNone = 0,
                ItemFlagCanSelectWithoutAmmo = 1,
                ItemFlagNoautoreload = 2,
                ItemFlagNoautoswitchempty = 4,
                ItemFlagLimitinworld = 8,
                ItemFlagExhaustible = 16,
                ItemFlagDohitlocationdmg = 32,
                ItemFlagNoammopickups = 64,
                ItemFlagNoitempickup = 128
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum EntityDisolveTypeT {
                EntityDissolveInvalid = -1,
                EntityDissolveNormal = 0,
                EntityDissolveElectrical = 1,
                EntityDissolveElectricalLight = 2,
                EntityDissolveCore = 3
            }
            // Alignment: 4
            // Members count: 14
            #[repr(i32)]
            pub enum HitGroupT {
                HitgroupInvalid = -1,
                HitgroupGeneric = 0,
                HitgroupHead = 1,
                HitgroupChest = 2,
                HitgroupStomach = 3,
                HitgroupLeftarm = 4,
                HitgroupRightarm = 5,
                HitgroupLeftleg = 6,
                HitgroupRightleg = 7,
                HitgroupNeck = 8,
                HitgroupUnused = 9,
                HitgroupGear = 10,
                HitgroupSpecial = 11,
                HitgroupCount = 12
            }
            // Alignment: 1
            // Members count: 7
            #[repr(i8)]
            pub enum WaterLevelT {
                WlNotInWater = 0,
                WlFeet = 1,
                WlKnees = 2,
                WlWaist = 3,
                WlChest = 4,
                WlFullyUnderwater = 5,
                WlCount = 6
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum DoorStateT {
                DoorStateClosed = 0,
                DoorStateOpening = 1,
                DoorStateOpen = 2,
                DoorStateClosing = 3,
                DoorStateAjar = 4
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ShadowTypeT {
                ShadowsNone = 0,
                ShadowsSimple = 1
            }
            // Alignment: 4
            // Members count: 10
            #[repr(i32)]
            pub enum ClassT {
                ClassNone = 0,
                ClassPlayer = 1,
                ClassPlayerAlly = 2,
                ClassBomb = 3,
                ClassFootContactShadow = 4,
                ClassWeapon = 5,
                ClassWaterSplasher = 6,
                ClassWeaponViewmodel = 7,
                ClassDoor = 8,
                NumClassifyClasses = 9
            }
            // Alignment: 4
            // Members count: 10
            #[repr(i32)]
            pub enum DispositionT {
                DEr = 0,
                DHt = 1,
                DFr = 2,
                DLi = 3,
                DNu = 4,
                DError = 0,
                DHate = 1,
                DFear = 2,
                DLike = 3,
                DNeutral = 4
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum LatchDirtyPermissionT {
                LatchDirtyDisallow = 0,
                LatchDirtyServerControlled = 1,
                LatchDirtyClientSimulated = 2,
                LatchDirtyPrediction = 3,
                LatchDirtyFramesimulate = 4,
                LatchDirtyParticleSimulate = 5
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum LifeStateT {
                LifeAlive = 0,
                LifeDying = 1,
                LifeDead = 2,
                LifeRespawnable = 3,
                LifeRespawning = 4
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum StanceTypeT {
                StanceCurrent = -1,
                StanceDefault = 0,
                StanceCrouching = 1,
                StanceProne = 2,
                NumStances = 3
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum ModifyDamageReturnT {
                ContinueToApplyDamage = 0,
                AbortDoNotApplyDamage = 1
            }
            // Alignment: 1
            // Members count: 3
            #[repr(i8)]
            pub enum BeginDeathLifeStateTransitionT {
                NoChangeInLifestate = 0,
                TransitionToLifestateDying = 1,
                TransitionToLifestateDead = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum WorldTextPanelHorizontalAlignT {
                WorldtextHorizontalAlignLeft = 0,
                WorldtextHorizontalAlignCenter = 1,
                WorldtextHorizontalAlignRight = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum WorldTextPanelVerticalAlignT {
                WorldtextVerticalAlignTop = 0,
                WorldtextVerticalAlignCenter = 1,
                WorldtextVerticalAlignBottom = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum WorldTextPanelOrientationT {
                WorldtextOrientationDefault = 0,
                WorldtextOrientationFaceuser = 1,
                WorldtextOrientationFaceuserUpright = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum AttributeprovidertypesT {
                ProviderGeneric = 0,
                ProviderWeapon = 1
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum SpawnPointCoopEnemyBotDefaultBehaviorT {
                DefendArea = 0,
                Hunt = 1,
                ChargeEnemy = 2,
                DefendInvestigate = 3
            }
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum MoveMountingAmountT {
                MoveMountNone = 0,
                MoveMountLow = 1,
                MoveMountHigh = 2,
                MoveMountMaxcount = 3
            }
            // Alignment: 4
            // Members count: 10
            #[repr(i32)]
            pub enum CsPlayerState {
                StateActive = 0,
                StateWelcome = 1,
                StatePickingteam = 2,
                StatePickingclass = 3,
                StateDeathAnim = 4,
                StateDeathWaitForKey = 5,
                StateObserverMode = 6,
                StateGungameRespawn = 7,
                StateDormant = 8,
                NumPlayerStates = 9
            }
            // Alignment: 4
            // Members count: 18
            #[repr(i32)]
            pub enum CsPlayerBlockingUseActionT {
                KCsPlayerBlockingUseActionNone = 0,
                KCsPlayerBlockingUseActionDefusingDefault = 1,
                KCsPlayerBlockingUseActionDefusingWithKit = 2,
                KCsPlayerBlockingUseActionHostageGrabbing = 3,
                KCsPlayerBlockingUseActionHostageDropping = 4,
                KCsPlayerBlockingUseActionOpeningSafe = 5,
                KCsPlayerBlockingUseActionEquippingParachute = 6,
                KCsPlayerBlockingUseActionEquippingHeavyArmor = 7,
                KCsPlayerBlockingUseActionEquippingContract = 8,
                KCsPlayerBlockingUseActionEquippingTabletUpgrade = 9,
                KCsPlayerBlockingUseActionTakingOffHeavyArmor = 10,
                KCsPlayerBlockingUseActionPayingToOpenDoor = 11,
                KCsPlayerBlockingUseActionCancelingSpawnRappelling = 12,
                KCsPlayerBlockingUseActionEquippingExoJump = 13,
                KCsPlayerBlockingUseActionPickingUpBumpMine = 14,
                KCsPlayerBlockingUseActionMapLongUseEntityPickup = 15,
                KCsPlayerBlockingUseActionMapLongUseEntityPlace = 16,
                KCsPlayerBlockingUseActionMaxCount = 17
            }
            // Alignment: 4
            // Members count: 8
            #[repr(i32)]
            pub enum GrenadeTypeT {
                GrenadeTypeExplosive = 0,
                GrenadeTypeFlash = 1,
                GrenadeTypeFire = 2,
                GrenadeTypeDecoy = 3,
                GrenadeTypeSmoke = 4,
                GrenadeTypeSensor = 5,
                GrenadeTypeSnowball = 6,
                GrenadeTypeTotal = 7
            }
            // Alignment: 4
            // Members count: 13
            #[repr(i32)]
            pub enum QuestProgressReason {
                QuestNoninitialized = 0,
                QuestOk = 1,
                QuestNotEnoughPlayers = 2,
                QuestWarmup = 3,
                QuestNotConnectedToSteam = 4,
                QuestNonofficialServer = 5,
                QuestNoEntitlement = 6,
                QuestNoQuest = 7,
                QuestPlayerIsBot = 8,
                QuestWrongMap = 9,
                QuestWrongMode = 10,
                QuestNotSyncedWithServer = 11,
                QuestReasonMax = 12
            }
            // Alignment: 4
            // Members count: 74
            #[repr(i32)]
            pub enum LoadoutSlotT {
                LoadoutSlotPromoted = -2,
                LoadoutSlotInvalid = -1,
                LoadoutSlotMelee = 0,
                LoadoutSlotC4 = 1,
                LoadoutSlotFirstAutoBuyWeapon = 0,
                LoadoutSlotLastAutoBuyWeapon = 1,
                LoadoutSlotSecondary0 = 2,
                LoadoutSlotSecondary1 = 3,
                LoadoutSlotSecondary2 = 4,
                LoadoutSlotSecondary3 = 5,
                LoadoutSlotSecondary4 = 6,
                LoadoutSlotSecondary5 = 7,
                LoadoutSlotSmg0 = 8,
                LoadoutSlotSmg1 = 9,
                LoadoutSlotSmg2 = 10,
                LoadoutSlotSmg3 = 11,
                LoadoutSlotSmg4 = 12,
                LoadoutSlotSmg5 = 13,
                LoadoutSlotRifle0 = 14,
                LoadoutSlotRifle1 = 15,
                LoadoutSlotRifle2 = 16,
                LoadoutSlotRifle3 = 17,
                LoadoutSlotRifle4 = 18,
                LoadoutSlotRifle5 = 19,
                LoadoutSlotHeavy0 = 20,
                LoadoutSlotHeavy1 = 21,
                LoadoutSlotHeavy2 = 22,
                LoadoutSlotHeavy3 = 23,
                LoadoutSlotHeavy4 = 24,
                LoadoutSlotHeavy5 = 25,
                LoadoutSlotFirstWheelWeapon = 2,
                LoadoutSlotLastWheelWeapon = 25,
                LoadoutSlotFirstPrimaryWeapon = 8,
                LoadoutSlotLastPrimaryWeapon = 25,
                LoadoutSlotFirstWheelGrenade = 26,
                LoadoutSlotGrenade0 = 26,
                LoadoutSlotGrenade1 = 27,
                LoadoutSlotGrenade2 = 28,
                LoadoutSlotGrenade3 = 29,
                LoadoutSlotGrenade4 = 30,
                LoadoutSlotGrenade5 = 31,
                LoadoutSlotLastWheelGrenade = 31,
                LoadoutSlotEquipment0 = 32,
                LoadoutSlotEquipment1 = 33,
                LoadoutSlotEquipment2 = 34,
                LoadoutSlotEquipment3 = 35,
                LoadoutSlotEquipment4 = 36,
                LoadoutSlotEquipment5 = 37,
                LoadoutSlotFirstWheelEquipment = 32,
                LoadoutSlotLastWheelEquipment = 37,
                LoadoutSlotClothingCustomplayer = 38,
                LoadoutSlotPet = 39,
                LoadoutSlotClothingFacemask = 40,
                LoadoutSlotClothingHands = 41,
                LoadoutSlotFirstCosmetic = 41,
                LoadoutSlotLastCosmetic = 41,
                LoadoutSlotClothingEyewear = 42,
                LoadoutSlotClothingHat = 43,
                LoadoutSlotClothingLowerbody = 44,
                LoadoutSlotClothingTorso = 45,
                LoadoutSlotClothingAppearance = 46,
                LoadoutSlotMisc0 = 47,
                LoadoutSlotMisc1 = 48,
                LoadoutSlotMisc2 = 49,
                LoadoutSlotMisc3 = 50,
                LoadoutSlotMisc4 = 51,
                LoadoutSlotMisc5 = 52,
                LoadoutSlotMisc6 = 53,
                LoadoutSlotMusickit = 54,
                LoadoutSlotFlair0 = 55,
                LoadoutSlotSpray0 = 56,
                LoadoutSlotFirstAllCharacter = 54,
                LoadoutSlotLastAllCharacter = 56,
                LoadoutSlotCount = 57
            }
            // Alignment: 1
            // Members count: 8
            #[repr(i8)]
            pub enum EKillTypesT {
                KillNone = 0,
                KillDefault = 1,
                KillHeadshot = 2,
                KillBlast = 3,
                KillBurn = 4,
                KillSlash = 5,
                KillShock = 6,
                KilltypeCount = 7
            }
            // Alignment: 4
            // Members count: 9
            #[repr(i32)]
            pub enum PreviewCharacterMode {
                Diorama = 0,
                MainMenu = 1,
                BuyMenu = 2,
                TeamSelect = 3,
                EndOfMatch = 4,
                InventoryInspect = 5,
                Walking = 6,
                TeamIntro = 7,
                WingmanIntro = 8
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum PreviewWeaponState {
                Dropped = 0,
                Holstered = 1,
                Deployed = 2,
                Planted = 3,
                Inspect = 4,
                Icon = 5
            }
            // Alignment: 4
            // Members count: 23
            #[repr(i32)]
            pub enum PreviewEomCelebration {
                Walkup = 0,
                Punching = 1,
                Swagger = 2,
                Dropdown = 3,
                Stretch = 4,
                SwatFemale = 5,
                MaskF = 6,
                Guerilla = 7,
                Guerilla02 = 8,
                Gendarmerie = 9,
                ScubaFemale = 10,
                ScubaMale = 11,
                AvaDefeat = 12,
                GendarmerieDefeat = 13,
                MaeDefeat = 14,
                RicksawDefeat = 15,
                ScubaFemaleDefeat = 16,
                ScubaMaleDefeat = 17,
                CrasswaterDefeat = 18,
                DarrylDefeat = 19,
                DoctorDefeat = 20,
                MuhlikDefeat = 21,
                VypaDefeat = 22
            }
            // Alignment: 4
            // Members count: 20
            #[repr(i32)]
            pub enum CsWeaponType {
                WeapontypeKnife = 0,
                WeapontypePistol = 1,
                WeapontypeSubmachinegun = 2,
                WeapontypeRifle = 3,
                WeapontypeShotgun = 4,
                WeapontypeSniperRifle = 5,
                WeapontypeMachinegun = 6,
                WeapontypeC4 = 7,
                WeapontypeTaser = 8,
                WeapontypeGrenade = 9,
                WeapontypeEquipment = 10,
                WeapontypeStackableitem = 11,
                WeapontypeFists = 12,
                WeapontypeBreachcharge = 13,
                WeapontypeBumpmine = 14,
                WeapontypeTablet = 15,
                WeapontypeMelee = 16,
                WeapontypeShield = 17,
                WeapontypeZoneRepulsor = 18,
                WeapontypeUnknown = 19
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum CsWeaponCategory {
                WeaponcategoryOther = 0,
                WeaponcategoryMelee = 1,
                WeaponcategorySecondary = 2,
                WeaponcategorySmg = 3,
                WeaponcategoryRifle = 4,
                WeaponcategoryHeavy = 5,
                WeaponcategoryCount = 6
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum CsWeaponSilencerType {
                WeaponsilencerNone = 0,
                WeaponsilencerDetachable = 1,
                WeaponsilencerIntegrated = 2
            }
            // Alignment: 4
            // Members count: 24
            #[repr(i32)]
            pub enum PlayerAnimEventT {
                PlayeranimeventFireGunPrimary = 0,
                PlayeranimeventFireGunSecondary = 1,
                PlayeranimeventGrenadePullPin = 2,
                PlayeranimeventThrowGrenade = 3,
                PlayeranimeventJump = 4,
                PlayeranimeventReload = 5,
                PlayeranimeventClearFiring = 6,
                PlayeranimeventDeploy = 7,
                PlayeranimeventSilencerState = 8,
                PlayeranimeventSilencerToggle = 9,
                PlayeranimeventThrowGrenadeUnderhand = 10,
                PlayeranimeventCatchWeapon = 11,
                PlayeranimeventLookatweaponRequest = 12,
                PlayeranimeventReloadCancelLookatweapon = 13,
                PlayeranimeventHaulback = 14,
                PlayeranimeventIdle = 15,
                PlayeranimeventStrikeHit = 16,
                PlayeranimeventStrikeMiss = 17,
                PlayeranimeventBackstab = 18,
                PlayeranimeventDryfire = 19,
                PlayeranimeventFidget = 20,
                PlayeranimeventRelease = 21,
                PlayeranimeventTaunt = 22,
                PlayeranimeventCount = 23
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum MedalRankT {
                MedalRankNone = 0,
                MedalRankBronze = 1,
                MedalRankSilver = 2,
                MedalRankGold = 3,
                MedalRankCount = 4
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum CsWeaponStateT {
                WeaponNotCarried = 0,
                WeaponIsCarriedByPlayer = 1,
                WeaponIsActive = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum CsWeaponMode {
                PrimaryMode = 0,
                SecondaryMode = 1,
                WeaponModeMax = 2
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum C4LightEffectT {
                ELightEffectNone = 0,
                ELightEffectDropped = 1,
                ELightEffectThirdPersonHeld = 2
            }
            // Alignment: 4
            // Members count: 17
            #[repr(i32)]
            pub enum GearSlotT {
                GearSlotInvalid = -1,
                GearSlotRifle = 0,
                GearSlotPistol = 1,
                GearSlotKnife = 2,
                GearSlotGrenades = 3,
                GearSlotC4 = 4,
                GearSlotReservedSlot6 = 5,
                GearSlotReservedSlot7 = 6,
                GearSlotReservedSlot8 = 7,
                GearSlotReservedSlot9 = 8,
                GearSlotReservedSlot10 = 9,
                GearSlotReservedSlot11 = 10,
                GearSlotBoosts = 11,
                GearSlotUtility = 12,
                GearSlotCount = 13,
                GearSlotFirst = 0,
                GearSlotLast = 12
            }
            // Alignment: 4
            // Members count: 7
            #[repr(i32)]
            pub enum ChickenActivity {
                Idle = 0,
                Walk = 1,
                Run = 2,
                Hop = 3,
                Jump = 4,
                Glide = 5,
                Land = 6
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod c_range_float {
                pub const M_P_VALUE: usize = 0x0; // float32[2]
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod c_range_int {
                pub const M_P_VALUE: usize = 0x0; // int32[2]
            }
            // Parent: None
            // Fields count: 2
            pub mod extent {
                pub const LO: usize = 0x0; // 
                pub const HI: usize = 0xC; // 
            }
            // Parent: None
            // Fields count: 2
            pub mod nav_gravity_t {
                pub const M_V_GRAVITY: usize = 0x0; // 
                pub const M_B_DEFAULT: usize = 0xC; // bool
            }
            // Parent: None
            // Fields count: 0
            pub mod c_nav_volume {
            }
            // Parent: c_nav_volume
            // Fields count: 1
            pub mod c_nav_volume_vector {
                pub const M_B_HAS_BEEN_PRE_FILTERED: usize = 0x78; // bool
            }
            // Parent: c_nav_volume_vector
            // Fields count: 0
            pub mod c_nav_volume_all {
            }
            // Parent: c_nav_volume
            // Fields count: 2
            pub mod c_nav_volume_sphere {
                pub const M_V_CENTER: usize = 0x70; // 
                pub const M_FL_RADIUS: usize = 0x7C; // float32
            }
            // Parent: c_nav_volume_sphere
            // Fields count: 1
            pub mod c_nav_volume_spherical_shell {
                pub const M_FL_RADIUS_INNER: usize = 0x80; // float32
            }
            // Parent: None
            // Fields count: 0
            pub mod c_entity_component {
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_nameStringableIndex (int32)
            pub mod c_entity_identity {
                pub const M_NAME_STRINGABLE_INDEX: usize = 0x14; // int32
                pub const M_NAME: usize = 0x18; // CUtlSymbolLarge
                pub const M_DESIGNER_NAME: usize = 0x20; // CUtlSymbolLarge
                pub const M_FLAGS: usize = 0x30; // uint32
                pub const M_WORLD_GROUP_ID: usize = 0x38; // WorldGroupId_t
                pub const M_F_DATA_OBJECT_TYPES: usize = 0x3C; // uint32
                pub const M_PATH_INDEX: usize = 0x40; // ChangeAccessorFieldPathIndex_t
                pub const M_P_PREV: usize = 0x58; // CEntityIdentity*
                pub const M_P_NEXT: usize = 0x60; // CEntityIdentity*
                pub const M_P_PREV_BY_CLASS: usize = 0x68; // CEntityIdentity*
                pub const M_P_NEXT_BY_CLASS: usize = 0x70; // CEntityIdentity*
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_pEntity (CEntityIdentity *)
            // NetworkVarNames: m_CScriptComponent (CScriptComponent::Storage_t)
            pub mod c_entity_instance {
                pub const M_ISZ_PRIVATE_V_SCRIPTS: usize = 0x8; // CUtlSymbolLarge
                pub const M_P_ENTITY: usize = 0x10; // CEntityIdentity*
                pub const M_C_SCRIPT_COMPONENT: usize = 0x28; // CScriptComponent*
                pub const M_B_VISIBLEIN_PVS: usize = 0x30; // bool
            }
            // Parent: c_entity_component
            // Fields count: 1
            pub mod c_script_component {
                pub const M_SCRIPT_CLASS_NAME: usize = 0x30; // CUtlSymbolLarge
            }
            // Parent: c_entity_component
            // Fields count: 2
            pub mod c_body_component {
                pub const M_P_SCENE_NODE: usize = 0x8; // CGameSceneNode*
                pub const M_P_CHAIN_ENTITY: usize = 0x20; // CNetworkVarChainer
            }
            // Parent: c_body_component
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_sceneNode (CGameSceneNode)
            pub mod c_body_component_point {
                pub const M_SCENE_NODE: usize = 0x50; // CGameSceneNode
                pub const M_P_CHAIN_ENTITY: usize = 0x1A0; // CNetworkVarChainer
            }
            // Parent: c_body_component
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_skeletonInstance (CSkeletonInstance)
            pub mod c_body_component_skeleton_instance {
                pub const M_SKELETON_INSTANCE: usize = 0x50; // CSkeletonInstance
                pub const M_P_CHAIN_ENTITY: usize = 0x430; // CNetworkVarChainer
            }
            // Parent: c_entity_component
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_bvDisabledHitGroups (uint32)
            pub mod c_hitbox_component {
                pub const M_BV_DISABLED_HIT_GROUPS: usize = 0x24; // uint32[1]
            }
            // Parent: c_entity_component
            // Fields count: 67
            //
            // Metadata:
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_SecondaryColor (Color)
            // NetworkVarNames: m_flBrightness (float)
            // NetworkVarNames: m_flBrightnessScale (float)
            // NetworkVarNames: m_flBrightnessMult (float)
            // NetworkVarNames: m_flRange (float)
            // NetworkVarNames: m_flFalloff (float)
            // NetworkVarNames: m_flAttenuation0 (float)
            // NetworkVarNames: m_flAttenuation1 (float)
            // NetworkVarNames: m_flAttenuation2 (float)
            // NetworkVarNames: m_flTheta (float)
            // NetworkVarNames: m_flPhi (float)
            // NetworkVarNames: m_hLightCookie (HRenderTextureStrong)
            // NetworkVarNames: m_nCascades (int)
            // NetworkVarNames: m_nCastShadows (int)
            // NetworkVarNames: m_nShadowWidth (int)
            // NetworkVarNames: m_nShadowHeight (int)
            // NetworkVarNames: m_bRenderDiffuse (bool)
            // NetworkVarNames: m_nRenderSpecular (int)
            // NetworkVarNames: m_bRenderTransmissive (bool)
            // NetworkVarNames: m_flOrthoLightWidth (float)
            // NetworkVarNames: m_flOrthoLightHeight (float)
            // NetworkVarNames: m_nStyle (int)
            // NetworkVarNames: m_Pattern (CUtlString)
            // NetworkVarNames: m_nCascadeRenderStaticObjects (int)
            // NetworkVarNames: m_flShadowCascadeCrossFade (float)
            // NetworkVarNames: m_flShadowCascadeDistanceFade (float)
            // NetworkVarNames: m_flShadowCascadeDistance0 (float)
            // NetworkVarNames: m_flShadowCascadeDistance1 (float)
            // NetworkVarNames: m_flShadowCascadeDistance2 (float)
            // NetworkVarNames: m_flShadowCascadeDistance3 (float)
            // NetworkVarNames: m_nShadowCascadeResolution0 (int)
            // NetworkVarNames: m_nShadowCascadeResolution1 (int)
            // NetworkVarNames: m_nShadowCascadeResolution2 (int)
            // NetworkVarNames: m_nShadowCascadeResolution3 (int)
            // NetworkVarNames: m_bUsesBakedShadowing (bool)
            // NetworkVarNames: m_nShadowPriority (int)
            // NetworkVarNames: m_nBakedShadowIndex (int)
            // NetworkVarNames: m_bRenderToCubemaps (bool)
            // NetworkVarNames: m_nDirectLight (int)
            // NetworkVarNames: m_nIndirectLight (int)
            // NetworkVarNames: m_flFadeMinDist (float)
            // NetworkVarNames: m_flFadeMaxDist (float)
            // NetworkVarNames: m_flShadowFadeMinDist (float)
            // NetworkVarNames: m_flShadowFadeMaxDist (float)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bFlicker (bool)
            // NetworkVarNames: m_bPrecomputedFieldsValid (bool)
            // NetworkVarNames: m_vPrecomputedBoundsMins (Vector)
            // NetworkVarNames: m_vPrecomputedBoundsMaxs (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent (Vector)
            // NetworkVarNames: m_flPrecomputedMaxRange (float)
            // NetworkVarNames: m_nFogLightingMode (int)
            // NetworkVarNames: m_flFogContributionStength (float)
            // NetworkVarNames: m_flNearClipPlane (float)
            // NetworkVarNames: m_SkyColor (Color)
            // NetworkVarNames: m_flSkyIntensity (float)
            // NetworkVarNames: m_SkyAmbientBounce (Color)
            // NetworkVarNames: m_bUseSecondaryColor (bool)
            // NetworkVarNames: m_bMixedShadows (bool)
            // NetworkVarNames: m_flLightStyleStartTime (GameTime_t)
            // NetworkVarNames: m_flCapsuleLength (float)
            // NetworkVarNames: m_flMinRoughness (float)
            pub mod c_light_component {
                pub const M_P_CHAIN_ENTITY: usize = 0x48; // CNetworkVarChainer
                pub const M_COLOR: usize = 0x85; // Color
                pub const M_SECONDARY_COLOR: usize = 0x89; // Color
                pub const M_FL_BRIGHTNESS: usize = 0x90; // float32
                pub const M_FL_BRIGHTNESS_SCALE: usize = 0x94; // float32
                pub const M_FL_BRIGHTNESS_MULT: usize = 0x98; // float32
                pub const M_FL_RANGE: usize = 0x9C; // float32
                pub const M_FL_FALLOFF: usize = 0xA0; // float32
                pub const M_FL_ATTENUATION0: usize = 0xA4; // float32
                pub const M_FL_ATTENUATION1: usize = 0xA8; // float32
                pub const M_FL_ATTENUATION2: usize = 0xAC; // float32
                pub const M_FL_THETA: usize = 0xB0; // float32
                pub const M_FL_PHI: usize = 0xB4; // float32
                pub const M_H_LIGHT_COOKIE: usize = 0xB8; // CStrongHandle< InfoForResourceTypeCTextureBase >
                pub const M_N_CASCADES: usize = 0xC0; // int32
                pub const M_N_CAST_SHADOWS: usize = 0xC4; // int32
                pub const M_N_SHADOW_WIDTH: usize = 0xC8; // int32
                pub const M_N_SHADOW_HEIGHT: usize = 0xCC; // int32
                pub const M_B_RENDER_DIFFUSE: usize = 0xD0; // bool
                pub const M_N_RENDER_SPECULAR: usize = 0xD4; // int32
                pub const M_B_RENDER_TRANSMISSIVE: usize = 0xD8; // bool
                pub const M_FL_ORTHO_LIGHT_WIDTH: usize = 0xDC; // float32
                pub const M_FL_ORTHO_LIGHT_HEIGHT: usize = 0xE0; // float32
                pub const M_N_STYLE: usize = 0xE4; // int32
                pub const M_PATTERN: usize = 0xE8; // CUtlString
                pub const M_N_CASCADE_RENDER_STATIC_OBJECTS: usize = 0xF0; // int32
                pub const M_FL_SHADOW_CASCADE_CROSS_FADE: usize = 0xF4; // float32
                pub const M_FL_SHADOW_CASCADE_DISTANCE_FADE: usize = 0xF8; // float32
                pub const M_FL_SHADOW_CASCADE_DISTANCE0: usize = 0xFC; // float32
                pub const M_FL_SHADOW_CASCADE_DISTANCE1: usize = 0x100; // float32
                pub const M_FL_SHADOW_CASCADE_DISTANCE2: usize = 0x104; // float32
                pub const M_FL_SHADOW_CASCADE_DISTANCE3: usize = 0x108; // float32
                pub const M_N_SHADOW_CASCADE_RESOLUTION0: usize = 0x10C; // int32
                pub const M_N_SHADOW_CASCADE_RESOLUTION1: usize = 0x110; // int32
                pub const M_N_SHADOW_CASCADE_RESOLUTION2: usize = 0x114; // int32
                pub const M_N_SHADOW_CASCADE_RESOLUTION3: usize = 0x118; // int32
                pub const M_B_USES_BAKED_SHADOWING: usize = 0x11C; // bool
                pub const M_N_SHADOW_PRIORITY: usize = 0x120; // int32
                pub const M_N_BAKED_SHADOW_INDEX: usize = 0x124; // int32
                pub const M_B_RENDER_TO_CUBEMAPS: usize = 0x128; // bool
                pub const M_N_DIRECT_LIGHT: usize = 0x12C; // int32
                pub const M_N_INDIRECT_LIGHT: usize = 0x130; // int32
                pub const M_FL_FADE_MIN_DIST: usize = 0x134; // float32
                pub const M_FL_FADE_MAX_DIST: usize = 0x138; // float32
                pub const M_FL_SHADOW_FADE_MIN_DIST: usize = 0x13C; // float32
                pub const M_FL_SHADOW_FADE_MAX_DIST: usize = 0x140; // float32
                pub const M_B_ENABLED: usize = 0x144; // bool
                pub const M_B_FLICKER: usize = 0x145; // bool
                pub const M_B_PRECOMPUTED_FIELDS_VALID: usize = 0x146; // bool
                pub const M_V_PRECOMPUTED_BOUNDS_MINS: usize = 0x148; // 
                pub const M_V_PRECOMPUTED_BOUNDS_MAXS: usize = 0x154; // 
                pub const M_V_PRECOMPUTED_OBB_ORIGIN: usize = 0x160; // 
                pub const M_V_PRECOMPUTED_OBB_ANGLES: usize = 0x16C; // QAngle
                pub const M_V_PRECOMPUTED_OBB_EXTENT: usize = 0x178; // 
                pub const M_FL_PRECOMPUTED_MAX_RANGE: usize = 0x184; // float32
                pub const M_N_FOG_LIGHTING_MODE: usize = 0x188; // int32
                pub const M_FL_FOG_CONTRIBUTION_STENGTH: usize = 0x18C; // float32
                pub const M_FL_NEAR_CLIP_PLANE: usize = 0x190; // float32
                pub const M_SKY_COLOR: usize = 0x194; // Color
                pub const M_FL_SKY_INTENSITY: usize = 0x198; // float32
                pub const M_SKY_AMBIENT_BOUNCE: usize = 0x19C; // Color
                pub const M_B_USE_SECONDARY_COLOR: usize = 0x1A0; // bool
                pub const M_B_MIXED_SHADOWS: usize = 0x1A1; // bool
                pub const M_FL_LIGHT_STYLE_START_TIME: usize = 0x1A4; // GameTime_t
                pub const M_FL_CAPSULE_LENGTH: usize = 0x1A8; // float32
                pub const M_FL_MIN_ROUGHNESS: usize = 0x1AC; // float32
                pub const M_B_PVS_MODIFY_ENTITY: usize = 0x1C0; // bool
            }
            // Parent: None
            // Fields count: 1
            pub mod c_network_transmit_component {
                pub const M_N_TRANSMIT_STATE_OWNED_COUNTER: usize = 0x16C; // uint8
            }
            // Parent: c_entity_component
            // Fields count: 5
            pub mod c_render_component {
                pub const M_P_CHAIN_ENTITY: usize = 0x10; // CNetworkVarChainer
                pub const M_B_IS_RENDERING_WITH_VIEW_MODELS: usize = 0x50; // bool
                pub const M_N_SPLITSCREEN_FLAGS: usize = 0x54; // uint32
                pub const M_B_ENABLE_RENDERING: usize = 0x60; // bool
                pub const M_B_INTERPOLATION_READY_TO_DRAW: usize = 0xB0; // bool
            }
            // Parent: None
            // Fields count: 0
            pub mod c_anim_event_listener_base {
            }
            // Parent: c_anim_event_listener_base
            // Fields count: 0
            pub mod c_anim_event_listener {
            }
            // Parent: c_anim_event_listener_base
            // Fields count: 0
            pub mod c_anim_event_queue_listener {
            }
            // Parent: None
            // Fields count: 1
            pub mod c_buoyancy_helper {
                pub const M_FL_FLUID_DENSITY: usize = 0x18; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod c_skill_float {
                pub const M_P_VALUE: usize = 0x0; // float32[4]
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod c_skill_int {
                pub const M_P_VALUE: usize = 0x0; // int32[4]
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_skill_damage {
                pub const M_FL_DAMAGE: usize = 0x0; // CSkillFloat
                pub const M_FL_PHYSICS_FORCE_DAMAGE: usize = 0x10; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod c_remap_float {
                pub const M_P_VALUE: usize = 0x0; // float32[4]
            }
            // Parent: None
            // Fields count: 2
            pub mod c_script_uniform_random_stream {
                pub const M_H_SCRIPT_SCOPE: usize = 0x8; // HSCRIPT
                pub const M_N_INITIAL_SEED: usize = 0x9C; // int32
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod c_base_player_controller_api {
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: nType (FixAngleSet_t)
            // NetworkVarNames: qAngle (QAngle)
            // NetworkVarNames: nIndex (uint32)
            pub mod view_angle_server_change_t {
                pub const N_TYPE: usize = 0x30; // FixAngleSet_t
                pub const Q_ANGLE: usize = 0x34; // QAngle
                pub const N_INDEX: usize = 0x40; // uint32
            }
            // Parent: None
            // Fields count: 2
            pub mod c_breakable_stage_helper {
                pub const M_N_CURRENT_STAGE: usize = 0x8; // int32
                pub const M_N_STAGE_COUNT: usize = 0xC; // int32
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod command_tool_command_t {
                pub const M_B_ENABLED: usize = 0x0; // bool
                pub const M_B_OPENED: usize = 0x1; // bool
                pub const M_INTERNAL_ID: usize = 0x4; // uint32
                pub const M_SHORT_NAME: usize = 0x8; // CUtlString
                pub const M_EXEC_MODE: usize = 0x10; // CommandExecMode_t
                pub const M_SPAWN_GROUP: usize = 0x18; // CUtlString
                pub const M_PERIODIC_EXEC_DELAY: usize = 0x20; // float32
                pub const M_SPEC_TYPE: usize = 0x24; // CommandEntitySpecType_t
                pub const M_ENTITY_SPEC: usize = 0x28; // CUtlString
                pub const M_COMMANDS: usize = 0x30; // CUtlString
                pub const M_SET_DEBUG_BITS: usize = 0x38; // DebugOverlayBits_t
                pub const M_CLEAR_DEBUG_BITS: usize = 0x40; // DebugOverlayBits_t
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod c_dynamic_prop_api {
            }
            // Parent: None
            // Fields count: 1
            pub mod c_player_pawn_component {
                pub const M_P_CHAIN_ENTITY: usize = 0x8; // CNetworkVarChainer
            }
            // Parent: None
            // Fields count: 1
            pub mod c_player_controller_component {
                pub const M_P_CHAIN_ENTITY: usize = 0x8; // CNetworkVarChainer
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_autoaim_services {
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: localSound (Vector)
            // NetworkVarNames: soundscapeIndex (int32)
            // NetworkVarNames: localBits (uint8)
            // NetworkVarNames: soundscapeEntityListIndex (int)
            // NetworkVarNames: soundEventHash (uint32)
            pub mod audioparams_t {
                pub const LOCAL_SOUND: usize = 0x8; // Vector[8]
                pub const SOUNDSCAPE_INDEX: usize = 0x68; // int32
                pub const LOCAL_BITS: usize = 0x6C; // uint8
                pub const SOUNDSCAPE_ENTITY_LIST_INDEX: usize = 0x70; // int32
                pub const SOUND_EVENT_HASH: usize = 0x74; // uint32
            }
            // Parent: c_player_pawn_component
            // Fields count: 12
            //
            // Metadata:
            // NetworkVarNames: m_vecCsViewPunchAngle (QAngle)
            // NetworkVarNames: m_nCsViewPunchAngleTick (GameTick_t)
            // NetworkVarNames: m_flCsViewPunchAngleTickRatio (float32)
            // NetworkVarNames: m_PlayerFog (fogplayerparams_t)
            // NetworkVarNames: m_hColorCorrectionCtrl (CHandle< CColorCorrection>)
            // NetworkVarNames: m_hViewEntity (CHandle< CBaseEntity>)
            // NetworkVarNames: m_hTonemapController (CHandle< CTonemapController2>)
            // NetworkVarNames: m_audio (audioparams_t)
            // NetworkVarNames: m_PostProcessingVolumes (CHandle<CPostProcessingVolume>)
            pub mod c_player_camera_services {
                pub const M_VEC_CS_VIEW_PUNCH_ANGLE: usize = 0x40; // QAngle
                pub const M_N_CS_VIEW_PUNCH_ANGLE_TICK: usize = 0x4C; // GameTick_t
                pub const M_FL_CS_VIEW_PUNCH_ANGLE_TICK_RATIO: usize = 0x50; // float32
                pub const M_PLAYER_FOG: usize = 0x58; // fogplayerparams_t
                pub const M_H_COLOR_CORRECTION_CTRL: usize = 0x98; // CHandle< CColorCorrection >
                pub const M_H_VIEW_ENTITY: usize = 0x9C; // CHandle< CBaseEntity >
                pub const M_H_TONEMAP_CONTROLLER: usize = 0xA0; // CHandle< CTonemapController2 >
                pub const M_AUDIO: usize = 0xA8; // audioparams_t
                pub const M_POST_PROCESSING_VOLUMES: usize = 0x120; // CNetworkUtlVectorBase< CHandle< CPostProcessingVolume > >
                pub const M_FL_OLD_PLAYER_Z: usize = 0x138; // float32
                pub const M_FL_OLD_PLAYER_VIEW_OFFSET_Z: usize = 0x13C; // float32
                pub const M_H_TRIGGER_SOUNDSCAPE_LIST: usize = 0x158; // CUtlVector< CHandle< CEnvSoundscapeTriggerable > >
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_flashlight_services {
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_item_services {
            }
            // Parent: c_player_pawn_component
            // Fields count: 15
            //
            // Metadata:
            // NetworkVarNames: m_nToggleButtonDownMask (ButtonBitMask_t)
            // NetworkVarNames: m_flMaxspeed (float32)
            // NetworkVarNames: m_arrForceSubtickMoveWhen (float32)
            pub mod c_player_movement_services {
                pub const M_N_IMPULSE: usize = 0x40; // int32
                pub const M_N_BUTTONS: usize = 0x48; // CInButtonState
                pub const M_N_QUEUED_BUTTON_DOWN_MASK: usize = 0x68; // uint64
                pub const M_N_QUEUED_BUTTON_CHANGE_MASK: usize = 0x70; // uint64
                pub const M_N_BUTTON_DOUBLE_PRESSED: usize = 0x78; // uint64
                pub const M_P_BUTTON_PRESSED_CMD_NUMBER: usize = 0x80; // uint32[64]
                pub const M_N_LAST_COMMAND_NUMBER_PROCESSED: usize = 0x180; // uint32
                pub const M_N_TOGGLE_BUTTON_DOWN_MASK: usize = 0x188; // uint64
                pub const M_FL_MAXSPEED: usize = 0x198; // float32
                pub const M_ARR_FORCE_SUBTICK_MOVE_WHEN: usize = 0x19C; // float32[4]
                pub const M_FL_FORWARD_MOVE: usize = 0x1AC; // float32
                pub const M_FL_LEFT_MOVE: usize = 0x1B0; // float32
                pub const M_FL_UP_MOVE: usize = 0x1B4; // float32
                pub const M_VEC_LAST_MOVEMENT_IMPULSES: usize = 0x1B8; // 
                pub const M_VEC_OLD_VIEW_ANGLES: usize = 0x1C4; // QAngle
            }
            // Parent: c_player_movement_services
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_flFallVelocity (float32)
            // NetworkVarNames: m_bInCrouch (bool)
            // NetworkVarNames: m_nCrouchState (uint32)
            // NetworkVarNames: m_flCrouchTransitionStartTime (GameTime_t)
            // NetworkVarNames: m_bDucked (bool)
            // NetworkVarNames: m_bDucking (bool)
            // NetworkVarNames: m_bInDuckJump (bool)
            pub mod c_player_movement_services_humanoid {
                pub const M_FL_STEP_SOUND_TIME: usize = 0x1D8; // float32
                pub const M_FL_FALL_VELOCITY: usize = 0x1DC; // float32
                pub const M_B_IN_CROUCH: usize = 0x1E0; // bool
                pub const M_N_CROUCH_STATE: usize = 0x1E4; // uint32
                pub const M_FL_CROUCH_TRANSITION_START_TIME: usize = 0x1E8; // GameTime_t
                pub const M_B_DUCKED: usize = 0x1EC; // bool
                pub const M_B_DUCKING: usize = 0x1ED; // bool
                pub const M_B_IN_DUCK_JUMP: usize = 0x1EE; // bool
                pub const M_GROUND_NORMAL: usize = 0x1F0; // 
                pub const M_FL_SURFACE_FRICTION: usize = 0x1FC; // float32
                pub const M_SURFACE_PROPS: usize = 0x200; // CUtlStringToken
                pub const M_N_STEPSIDE: usize = 0x210; // int32
                pub const M_I_TARGET_VOLUME: usize = 0x214; // int32
                pub const M_VEC_SMOOTHED_VELOCITY: usize = 0x218; // 
            }
            // Parent: c_player_pawn_component
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_iObserverMode (uint8)
            // NetworkVarNames: m_hObserverTarget (CHandle< CBaseEntity>)
            pub mod c_player_observer_services {
                pub const M_I_OBSERVER_MODE: usize = 0x40; // uint8
                pub const M_H_OBSERVER_TARGET: usize = 0x44; // CHandle< CBaseEntity >
                pub const M_I_OBSERVER_LAST_MODE: usize = 0x48; // ObserverMode_t
                pub const M_B_FORCED_OBSERVER_MODE: usize = 0x4C; // bool
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_use_services {
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_water_services {
            }
            // Parent: c_player_pawn_component
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_hMyWeapons (CHandle< CBasePlayerWeapon >)
            // NetworkVarNames: m_hActiveWeapon (CHandle< CBasePlayerWeapon>)
            // NetworkVarNames: m_hLastWeapon (CHandle< CBasePlayerWeapon>)
            // NetworkVarNames: m_iAmmo (uint16)
            pub mod c_player_weapon_services {
                pub const M_H_MY_WEAPONS: usize = 0x40; // CNetworkUtlVectorBase< CHandle< CBasePlayerWeapon > >
                pub const M_H_ACTIVE_WEAPON: usize = 0x58; // CHandle< CBasePlayerWeapon >
                pub const M_H_LAST_WEAPON: usize = 0x5C; // CHandle< CBasePlayerWeapon >
                pub const M_I_AMMO: usize = 0x60; // uint16[32]
                pub const M_B_PREVENT_WEAPON_PICKUP: usize = 0xA0; // bool
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod ammo_type_info_t {
                pub const M_N_MAX_CARRY: usize = 0x10; // int32
                pub const M_N_SPLASH_SIZE: usize = 0x1C; // CRangeInt
                pub const M_N_FLAGS: usize = 0x24; // AmmoFlags_t
                pub const M_FL_MASS: usize = 0x28; // float32
                pub const M_FL_SPEED: usize = 0x2C; // CRangeFloat
            }
            // Parent: None
            // Fields count: 0
            pub mod c_anim_graph_controller_base {
            }
            // Parent: c_body_component_skeleton_instance
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_animationController (CBaseAnimGraphController)
            pub mod c_body_component_base_anim_graph {
                pub const M_ANIMATION_CONTROLLER: usize = 0x460; // CBaseAnimGraphController
                pub const M_P_CHAIN_ENTITY: usize = 0x950; // CNetworkVarChainer
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_ID (CUtlStringToken)
            // NetworkVarNames: m_Values (Vector4D)
            pub mod entity_render_attribute_t {
                pub const M_ID: usize = 0x30; // CUtlStringToken
                pub const M_VALUES: usize = 0x34; // Vector4D
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_Handle (ModelConfigHandle_t)
            // NetworkVarNames: m_Name (string_t)
            // NetworkVarNames: m_AssociatedEntities (CHandle< CBaseModelEntity >)
            // NetworkVarNames: m_AssociatedEntityNames (string_t)
            pub mod active_model_config_t {
                pub const M_HANDLE: usize = 0x28; // ModelConfigHandle_t
                pub const M_NAME: usize = 0x30; // CUtlSymbolLarge
                pub const M_ASSOCIATED_ENTITIES: usize = 0x38; // CNetworkUtlVectorBase< CHandle< CBaseModelEntity > >
                pub const M_ASSOCIATED_ENTITY_NAMES: usize = 0x50; // CNetworkUtlVectorBase< CUtlSymbolLarge >
            }
            // Parent: c_body_component_skeleton_instance
            // Fields count: 1
            pub mod c_body_component_base_model_entity {
                pub const M_P_CHAIN_ENTITY: usize = 0x460; // CNetworkVarChainer
            }
            // Parent: None
            // Fields count: 2
            pub mod c_rope_overlap_hit {
                pub const M_H_ENTITY: usize = 0x0; // CHandle< CBaseEntity >
                pub const M_VEC_OVERLAPPING_LINKS: usize = 0x8; // CUtlVector< int32 >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hOwner (CEntityHandle)
            // NetworkVarNames: m_name (CUtlStringToken)
            pub mod c_game_scene_node_handle {
                pub const M_H_OWNER: usize = 0x8; // CEntityHandle
                pub const M_NAME: usize = 0xC; // CUtlStringToken
            }
            // Parent: None
            // Fields count: 32
            //
            // Metadata:
            // NetworkVarNames: m_hParent (CGameSceneNodeHandle)
            // NetworkVarNames: m_vecOrigin (CNetworkOriginCellCoordQuantizedVector)
            // NetworkVarNames: m_angRotation (QAngle)
            // NetworkVarNames: m_flScale (float)
            // NetworkVarNames: m_name (CUtlStringToken)
            // NetworkVarNames: m_hierarchyAttachName (CUtlStringToken)
            pub mod c_game_scene_node {
                pub const M_NODE_TO_WORLD: usize = 0x10; // CTransform
                pub const M_P_OWNER: usize = 0x30; // CEntityInstance*
                pub const M_P_PARENT: usize = 0x38; // CGameSceneNode*
                pub const M_P_CHILD: usize = 0x40; // CGameSceneNode*
                pub const M_P_NEXT_SIBLING: usize = 0x48; // CGameSceneNode*
                pub const M_H_PARENT: usize = 0x70; // CGameSceneNodeHandle
                pub const M_VEC_ORIGIN: usize = 0x80; // CNetworkOriginCellCoordQuantizedVector
                pub const M_ANG_ROTATION: usize = 0xB8; // QAngle
                pub const M_FL_SCALE: usize = 0xC4; // float32
                pub const M_VEC_ABS_ORIGIN: usize = 0xC8; // 
                pub const M_ANG_ABS_ROTATION: usize = 0xD4; // QAngle
                pub const M_FL_ABS_SCALE: usize = 0xE0; // float32
                pub const M_N_PARENT_ATTACHMENT_OR_BONE: usize = 0xE4; // int16
                pub const M_B_DEBUG_ABS_ORIGIN_CHANGES: usize = 0xE6; // bool
                pub const M_B_DORMANT: usize = 0xE7; // bool
                pub const M_B_FORCE_PARENT_TO_BE_NETWORKED: usize = 0xE8; // bool
                pub const M_B_DIRTY_HIERARCHY: usize = 0x0; // bitfield:1
                pub const M_B_DIRTY_BONE_MERGE_INFO: usize = 0x0; // bitfield:1
                pub const M_B_NETWORKED_POSITION_CHANGED: usize = 0x0; // bitfield:1
                pub const M_B_NETWORKED_ANGLES_CHANGED: usize = 0x0; // bitfield:1
                pub const M_B_NETWORKED_SCALE_CHANGED: usize = 0x0; // bitfield:1
                pub const M_B_WILL_BE_CALLING_POST_DATA_UPDATE: usize = 0x0; // bitfield:1
                pub const M_B_BONE_MERGE_FLEX: usize = 0x0; // bitfield:1
                pub const M_N_LATCH_ABS_ORIGIN: usize = 0x0; // bitfield:2
                pub const M_B_DIRTY_BONE_MERGE_BONE_TO_ROOT: usize = 0x0; // bitfield:1
                pub const M_N_HIERARCHICAL_DEPTH: usize = 0xEB; // uint8
                pub const M_N_HIERARCHY_TYPE: usize = 0xEC; // uint8
                pub const M_N_DO_NOT_SET_ANIM_TIME_IN_INVALIDATE_PHYSICS_COUNT: usize = 0xED; // uint8
                pub const M_NAME: usize = 0xF0; // CUtlStringToken
                pub const M_HIERARCHY_ATTACH_NAME: usize = 0x130; // CUtlStringToken
                pub const M_FL_Z_OFFSET: usize = 0x134; // float32
                pub const M_V_RENDER_ORIGIN: usize = 0x138; // 
            }
            // Parent: None
            // Fields count: 0
            pub mod i_choreo_services {
            }
            // Parent: None
            // Fields count: 1
            pub mod c_in_button_state {
                pub const M_P_BUTTON_STATES: usize = 0x8; // uint64[3]
            }
            // Parent: None
            // Fields count: 6
            pub mod sequence_history_t {
                pub const M_H_SEQUENCE: usize = 0x0; // HSequence
                pub const M_FL_SEQ_START_TIME: usize = 0x4; // GameTime_t
                pub const M_FL_SEQ_FIXED_CYCLE: usize = 0x8; // float32
                pub const M_N_SEQ_LOOP_MODE: usize = 0xC; // AnimLoopMode_t
                pub const M_FL_PLAYBACK_RATE: usize = 0x10; // float32
                pub const M_FL_CYCLES_PER_SECOND: usize = 0x14; // float32
            }
            // Parent: None
            // Fields count: 0
            pub mod i_skeleton_animation_controller {
            }
            // Parent: i_skeleton_animation_controller
            // Fields count: 1
            pub mod c_skeleton_animation_controller {
                pub const M_P_SKELETON_INSTANCE: usize = 0x8; // CSkeletonInstance*
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_hSequence (HSequence)
            // NetworkVarNames: m_flPrevCycle (float32)
            // NetworkVarNames: m_flCycle (float32)
            pub mod c_networked_sequence_operation {
                pub const M_H_SEQUENCE: usize = 0x8; // HSequence
                pub const M_FL_PREV_CYCLE: usize = 0xC; // float32
                pub const M_FL_CYCLE: usize = 0x10; // float32
                pub const M_FL_WEIGHT: usize = 0x14; // CNetworkedQuantizedFloat
                pub const M_B_SEQUENCE_CHANGE_NETWORKED: usize = 0x1C; // bool
                pub const M_B_DISCONTINUITY: usize = 0x1D; // bool
                pub const M_FL_PREV_CYCLE_FROM_DISCONTINUITY: usize = 0x20; // float32
                pub const M_FL_PREV_CYCLE_FOR_ANIM_EVENT_DETECTION: usize = 0x24; // float32
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_hModel (HModelStrong)
            // NetworkVarNames: m_bClientClothCreationSuppressed (bool)
            // NetworkVarNames: m_MeshGroupMask (MeshGroupMask_t)
            // NetworkVarNames: m_nIdealMotionType (int8)
            pub mod c_model_state {
                pub const M_H_MODEL: usize = 0xA0; // CStrongHandle< InfoForResourceTypeCModel >
                pub const M_MODEL_NAME: usize = 0xA8; // CUtlSymbolLarge
                pub const M_B_CLIENT_CLOTH_CREATION_SUPPRESSED: usize = 0xE8; // bool
                pub const M_MESH_GROUP_MASK: usize = 0x180; // uint64
                pub const M_N_IDEAL_MOTION_TYPE: usize = 0x202; // int8
                pub const M_N_FORCE_LOD: usize = 0x203; // int8
                pub const M_N_CLOTH_UPDATE_FLAGS: usize = 0x204; // int8
            }
            // Parent: c_game_scene_node
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_modelState (CModelState)
            // NetworkVarNames: m_bIsAnimationEnabled (bool)
            // NetworkVarNames: m_bUseParentRenderBounds (bool)
            // NetworkVarNames: m_materialGroup (CUtlStringToken)
            // NetworkVarNames: m_nHitboxSet (uint8)
            pub mod c_skeleton_instance {
                pub const M_MODEL_STATE: usize = 0x160; // CModelState
                pub const M_B_IS_ANIMATION_ENABLED: usize = 0x370; // bool
                pub const M_B_USE_PARENT_RENDER_BOUNDS: usize = 0x371; // bool
                pub const M_B_DISABLE_SOLID_COLLISIONS_FOR_HIERARCHY: usize = 0x372; // bool
                pub const M_B_DIRTY_MOTION_TYPE: usize = 0x0; // bitfield:1
                pub const M_B_IS_GENERATING_LATCHED_PARENT_SPACE_STATE: usize = 0x0; // bitfield:1
                pub const M_MATERIAL_GROUP: usize = 0x374; // CUtlStringToken
                pub const M_N_HITBOX_SET: usize = 0x378; // uint8
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_timestamp (GameTime_t)
            // NetworkVarNames: m_nWorldGroupId (WorldGroupId_t)
            pub mod interval_timer {
                pub const M_TIMESTAMP: usize = 0x8; // GameTime_t
                pub const M_N_WORLD_GROUP_ID: usize = 0xC; // WorldGroupId_t
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_duration (float32)
            // NetworkVarNames: m_timestamp (GameTime_t)
            // NetworkVarNames: m_timescale (float32)
            // NetworkVarNames: m_nWorldGroupId (WorldGroupId_t)
            pub mod countdown_timer {
                pub const M_DURATION: usize = 0x8; // float32
                pub const M_TIMESTAMP: usize = 0xC; // GameTime_t
                pub const M_TIMESCALE: usize = 0x10; // float32
                pub const M_N_WORLD_GROUP_ID: usize = 0x14; // WorldGroupId_t
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_duration (float32)
            // NetworkVarNames: m_timestamp (float32)
            // NetworkVarNames: m_timescale (float32)
            pub mod engine_countdown_timer {
                pub const M_DURATION: usize = 0x8; // float32
                pub const M_TIMESTAMP: usize = 0xC; // float32
                pub const M_TIMESCALE: usize = 0x10; // float32
            }
            // Parent: interval_timer
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_flValues (float)
            // NetworkVarNames: m_nValueCounts (int)
            // NetworkVarNames: m_nBucketCount (int)
            // NetworkVarNames: m_flInterval (float)
            // NetworkVarNames: m_flFinalValue (float)
            // NetworkVarNames: m_nCompressionType (TimelineCompression_t)
            // NetworkVarNames: m_bStopped (bool)
            pub mod c_timeline {
                pub const M_FL_VALUES: usize = 0x10; // float32[64]
                pub const M_N_VALUE_COUNTS: usize = 0x110; // int32[64]
                pub const M_N_BUCKET_COUNT: usize = 0x210; // int32
                pub const M_FL_INTERVAL: usize = 0x214; // float32
                pub const M_FL_FINAL_VALUE: usize = 0x218; // float32
                pub const M_N_COMPRESSION_TYPE: usize = 0x21C; // TimelineCompression_t
                pub const M_B_STOPPED: usize = 0x220; // bool
            }
            // Parent: None
            // Fields count: 24
            //
            // Metadata:
            // NetworkVarNames: m_PredNetBoolVariables (uint32)
            // NetworkVarNames: m_PredNetByteVariables (byte)
            // NetworkVarNames: m_PredNetUInt16Variables (uint16)
            // NetworkVarNames: m_PredNetIntVariables (int32)
            // NetworkVarNames: m_PredNetUInt32Variables (uint32)
            // NetworkVarNames: m_PredNetUInt64Variables (uint64)
            // NetworkVarNames: m_PredNetFloatVariables (float)
            // NetworkVarNames: m_PredNetVectorVariables (Vector)
            // NetworkVarNames: m_PredNetQuaternionVariables (Quaternion)
            // NetworkVarNames: m_PredNetGlobalSymbolVariables (CGlobalSymbol)
            // NetworkVarNames: m_OwnerOnlyPredNetBoolVariables (uint32)
            // NetworkVarNames: m_OwnerOnlyPredNetByteVariables (byte)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt16Variables (uint16)
            // NetworkVarNames: m_OwnerOnlyPredNetIntVariables (int32)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt32Variables (uint32)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt64Variables (uint64)
            // NetworkVarNames: m_OwnerOnlyPredNetFloatVariables (float)
            // NetworkVarNames: m_OwnerOnlyPredNetVectorVariables (Vector)
            // NetworkVarNames: m_OwnerOnlyPredNetQuaternionVariables (Quaternion)
            // NetworkVarNames: m_OwnerOnlyPredNetGlobalSymbolVariables (CGlobalSymbol)
            // NetworkVarNames: m_nBoolVariablesCount (int)
            // NetworkVarNames: m_nOwnerOnlyBoolVariablesCount (int)
            // NetworkVarNames: m_nRandomSeedOffset (int)
            // NetworkVarNames: m_flLastTeleportTime (float)
            pub mod c_anim_graph_networked_variables {
                pub const M_PRED_NET_BOOL_VARIABLES: usize = 0x8; // CNetworkUtlVectorBase< uint32 >
                pub const M_PRED_NET_BYTE_VARIABLES: usize = 0x20; // CNetworkUtlVectorBase< uint8 >
                pub const M_PRED_NET_U_INT16_VARIABLES: usize = 0x38; // CNetworkUtlVectorBase< uint16 >
                pub const M_PRED_NET_INT_VARIABLES: usize = 0x50; // CNetworkUtlVectorBase< int32 >
                pub const M_PRED_NET_U_INT32_VARIABLES: usize = 0x68; // CNetworkUtlVectorBase< uint32 >
                pub const M_PRED_NET_U_INT64_VARIABLES: usize = 0x80; // CNetworkUtlVectorBase< uint64 >
                pub const M_PRED_NET_FLOAT_VARIABLES: usize = 0x98; // CNetworkUtlVectorBase< float32 >
                pub const M_PRED_NET_VECTOR_VARIABLES: usize = 0xB0; // CNetworkUtlVectorBase< Vector >
                pub const M_PRED_NET_QUATERNION_VARIABLES: usize = 0xC8; // CNetworkUtlVectorBase< Quaternion >
                pub const M_PRED_NET_GLOBAL_SYMBOL_VARIABLES: usize = 0xE0; // CNetworkUtlVectorBase< CGlobalSymbol >
                pub const M_OWNER_ONLY_PRED_NET_BOOL_VARIABLES: usize = 0xF8; // CNetworkUtlVectorBase< uint32 >
                pub const M_OWNER_ONLY_PRED_NET_BYTE_VARIABLES: usize = 0x110; // CNetworkUtlVectorBase< uint8 >
                pub const M_OWNER_ONLY_PRED_NET_U_INT16_VARIABLES: usize = 0x128; // CNetworkUtlVectorBase< uint16 >
                pub const M_OWNER_ONLY_PRED_NET_INT_VARIABLES: usize = 0x140; // CNetworkUtlVectorBase< int32 >
                pub const M_OWNER_ONLY_PRED_NET_U_INT32_VARIABLES: usize = 0x158; // CNetworkUtlVectorBase< uint32 >
                pub const M_OWNER_ONLY_PRED_NET_U_INT64_VARIABLES: usize = 0x170; // CNetworkUtlVectorBase< uint64 >
                pub const M_OWNER_ONLY_PRED_NET_FLOAT_VARIABLES: usize = 0x188; // CNetworkUtlVectorBase< float32 >
                pub const M_OWNER_ONLY_PRED_NET_VECTOR_VARIABLES: usize = 0x1A0; // CNetworkUtlVectorBase< Vector >
                pub const M_OWNER_ONLY_PRED_NET_QUATERNION_VARIABLES: usize = 0x1B8; // CNetworkUtlVectorBase< Quaternion >
                pub const M_OWNER_ONLY_PRED_NET_GLOBAL_SYMBOL_VARIABLES: usize = 0x1D0; // CNetworkUtlVectorBase< CGlobalSymbol >
                pub const M_N_BOOL_VARIABLES_COUNT: usize = 0x1E8; // int32
                pub const M_N_OWNER_ONLY_BOOL_VARIABLES_COUNT: usize = 0x1EC; // int32
                pub const M_N_RANDOM_SEED_OFFSET: usize = 0x1F0; // int32
                pub const M_FL_LAST_TELEPORT_TIME: usize = 0x1F4; // float32
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPropertyCustomFGDType
            pub mod c_footstep_table_handle {
            }
            // Parent: None
            // Fields count: 8
            pub mod response_followup {
                pub const FOLLOWUP_CONCEPT: usize = 0x0; // char*
                pub const FOLLOWUP_CONTEXTS: usize = 0x8; // char*
                pub const FOLLOWUP_DELAY: usize = 0x10; // float32
                pub const FOLLOWUP_TARGET: usize = 0x14; // char*
                pub const FOLLOWUP_ENTITYIOTARGET: usize = 0x1C; // char*
                pub const FOLLOWUP_ENTITYIOINPUT: usize = 0x24; // char*
                pub const FOLLOWUP_ENTITYIODELAY: usize = 0x2C; // float32
                pub const B_FIRED: usize = 0x30; // bool
            }
            // Parent: None
            // Fields count: 3
            pub mod response_params {
                pub const ODDS: usize = 0x10; // int16
                pub const FLAGS: usize = 0x12; // int16
                pub const M_P_FOLLOWUP: usize = 0x18; // ResponseFollowup*
            }
            // Parent: None
            // Fields count: 2
            pub mod c_response_criteria_set {
                pub const M_N_NUM_PREFIXED_CONTEXTS: usize = 0x28; // int32
                pub const M_B_OVERRIDE_ON_APPEND: usize = 0x2C; // bool
            }
            // Parent: None
            // Fields count: 10
            pub mod crr_response {
                pub const M_TYPE: usize = 0x0; // uint8
                pub const M_SZ_RESPONSE_NAME: usize = 0x1; // char[192]
                pub const M_SZ_MATCHING_RULE: usize = 0xC1; // char[128]
                pub const M_PARAMS: usize = 0x148; // ResponseParams
                pub const M_F_MATCH_SCORE: usize = 0x168; // float32
                pub const M_SZ_SPEAKER_CONTEXT: usize = 0x170; // char*
                pub const M_SZ_WORLD_CONTEXT: usize = 0x178; // char*
                pub const M_FOLLOWUP: usize = 0x180; // ResponseFollowup
                pub const M_PCH_CRITERIA_NAMES: usize = 0x1B8; // CUtlVector< CUtlSymbol >
                pub const M_PCH_CRITERIA_VALUES: usize = 0x1D0; // CUtlVector< char* >
            }
            // Parent: None
            // Fields count: 2
            pub mod concept_history_t {
                pub const TIME_SPOKEN: usize = 0x0; // float32
                pub const M_RESPONSE: usize = 0x8; // CRR_Response
            }
            // Parent: None
            // Fields count: 9
            pub mod cai_expresser {
                pub const M_FL_STOP_TALK_TIME: usize = 0x38; // GameTime_t
                pub const M_FL_STOP_TALK_TIME_WITHOUT_DELAY: usize = 0x3C; // GameTime_t
                pub const M_FL_BLOCKED_TALK_TIME: usize = 0x40; // GameTime_t
                pub const M_VOICE_PITCH: usize = 0x44; // int32
                pub const M_FL_LAST_TIME_ACCEPTED_SPEAK: usize = 0x48; // GameTime_t
                pub const M_B_ALLOW_SPEAKING_INTERRUPTS: usize = 0x4C; // bool
                pub const M_B_CONSIDER_SCENE_INVOLVEMENT_AS_SPEECH: usize = 0x4D; // bool
                pub const M_N_LAST_SPOKEN_PRIORITY: usize = 0x50; // int32
                pub const M_P_OUTER: usize = 0x58; // CBaseFlex*
            }
            // Parent: None
            // Fields count: 1
            pub mod c_response_queue {
                pub const M_EXPRESSER_TARGETS: usize = 0x48; // CUtlVector< CAI_Expresser* >
            }
            // Parent: None
            // Fields count: 5
            pub mod c_response_queue_c_deferred_response {
                pub const M_CONTEXTS: usize = 0x8; // CResponseCriteriaSet
                pub const M_F_DISPATCH_TIME: usize = 0x38; // float32
                pub const M_H_ISSUER: usize = 0x3C; // CHandle< CBaseEntity >
                pub const M_RESPONSE: usize = 0x48; // CRR_Response
                pub const M_B_RESPONSE_VALID: usize = 0x230; // bool
            }
            // Parent: cai_expresser
            // Fields count: 1
            pub mod cai_expresser_with_followup {
                pub const M_P_POSTPONED_FOLLOWUP: usize = 0x60; // ResponseFollowup*
            }
            // Parent: cai_expresser_with_followup
            // Fields count: 1
            pub mod c_multiplayer_expresser {
                pub const M_B_ALLOW_MULTIPLE_SCENES: usize = 0x70; // bool
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod c_base_entity_api {
            }
            // Parent: None
            // Fields count: 10
            pub mod c_commentary_system {
                pub const M_B_COMMENTARY_CONVARS_CHANGING: usize = 0x11; // bool
                pub const M_B_COMMENTARY_ENABLED_MID_GAME: usize = 0x12; // bool
                pub const M_FL_NEXT_TELEPORT_TIME: usize = 0x14; // GameTime_t
                pub const M_I_TELEPORT_STAGE: usize = 0x18; // int32
                pub const M_B_CHEAT_STATE: usize = 0x1C; // bool
                pub const M_B_IS_FIRST_SPAWN_GROUP_TO_LOAD: usize = 0x1D; // bool
                pub const M_H_CURRENT_NODE: usize = 0x38; // CHandle< CPointCommentaryNode >
                pub const M_H_ACTIVE_COMMENTARY_NODE: usize = 0x3C; // CHandle< CPointCommentaryNode >
                pub const M_H_LAST_COMMENTARY_NODE: usize = 0x40; // CHandle< CPointCommentaryNode >
                pub const M_VEC_NODES: usize = 0x48; // CUtlVector< CHandle< CPointCommentaryNode > >
            }
            // Parent: None
            // Fields count: 1
            pub mod c_physics_shake {
                pub const M_FORCE: usize = 0x8; // 
            }
            // Parent: None
            // Fields count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_game_scripted_move_data {
                pub const M_V_ACCUMULATED_ROOT_MOTION: usize = 0x0; // 
                pub const M_V_DEST: usize = 0xC; // 
                pub const M_V_SRC: usize = 0x18; // 
                pub const M_ANG_SRC: usize = 0x24; // QAngle
                pub const M_ANG_DST: usize = 0x30; // QAngle
                pub const M_ANG_CURRENT: usize = 0x3C; // QAngle
                pub const M_FL_ANG_RATE: usize = 0x48; // float32
                pub const M_FL_DURATION: usize = 0x4C; // float32
                pub const M_FL_START_TIME: usize = 0x50; // GameTime_t
                pub const M_B_ACTIVE: usize = 0x54; // bool
                pub const M_B_TELEPORT_ON_END: usize = 0x55; // bool
                pub const M_B_IGNORE_ROTATION: usize = 0x56; // bool
                pub const M_N_TYPE: usize = 0x58; // ScriptedMoveType_t
                pub const M_B_SUCCESS: usize = 0x5C; // bool
                pub const M_N_FORCED_CROUCH_STATE: usize = 0x60; // ForcedCrouchState_t
                pub const M_B_IGNORE_COLLISIONS: usize = 0x64; // bool
            }
            // Parent: i_choreo_services
            // Fields count: 5
            pub mod c_game_choreo_services {
                pub const M_H_OWNER: usize = 0x8; // CHandle< CBaseAnimGraph >
                pub const M_H_SCRIPTED_SEQUENCE: usize = 0xC; // CHandle< CScriptedSequence >
                pub const M_SCRIPT_STATE: usize = 0x10; // IChoreoServices::ScriptState_t
                pub const M_CHOREO_STATE: usize = 0x14; // IChoreoServices::ChoreoState_t
                pub const M_FL_TIME_STARTED_STATE: usize = 0x18; // GameTime_t
            }
            // Parent: None
            // Fields count: 10
            pub mod hull_flags_t {
                pub const M_B_HULL_HUMAN: usize = 0x0; // bool
                pub const M_B_HULL_SMALL_CENTERED: usize = 0x1; // bool
                pub const M_B_HULL_WIDE_HUMAN: usize = 0x2; // bool
                pub const M_B_HULL_TINY: usize = 0x3; // bool
                pub const M_B_HULL_MEDIUM: usize = 0x4; // bool
                pub const M_B_HULL_TINY_CENTERED: usize = 0x5; // bool
                pub const M_B_HULL_LARGE: usize = 0x6; // bool
                pub const M_B_HULL_LARGE_CENTERED: usize = 0x7; // bool
                pub const M_B_HULL_MEDIUM_TALL: usize = 0x8; // bool
                pub const M_B_HULL_SMALL: usize = 0x9; // bool
            }
            // Parent: None
            // Fields count: 4
            pub mod c_constant_force_controller {
                pub const M_LINEAR: usize = 0xC; // 
                pub const M_ANGULAR: usize = 0x18; // RotationVector
                pub const M_LINEAR_SAVE: usize = 0x24; // 
                pub const M_ANGULAR_SAVE: usize = 0x30; // RotationVector
            }
            // Parent: None
            // Fields count: 4
            pub mod c_motor_controller {
                pub const M_SPEED: usize = 0x8; // float32
                pub const M_MAX_TORQUE: usize = 0xC; // float32
                pub const M_AXIS: usize = 0x10; // 
                pub const M_INERTIA_FACTOR: usize = 0x1C; // float32
            }
            // Parent: None
            // Fields count: 4
            pub mod c_sound_envelope {
                pub const M_CURRENT: usize = 0x0; // float32
                pub const M_TARGET: usize = 0x4; // float32
                pub const M_RATE: usize = 0x8; // float32
                pub const M_FORCEUPDATE: usize = 0xC; // bool
            }
            // Parent: None
            // Fields count: 2
            pub mod c_copy_recipient_filter {
                pub const M_FLAGS: usize = 0x8; // int32
                pub const M_RECIPIENTS: usize = 0x10; // CUtlVector< CPlayerSlot >
            }
            // Parent: None
            // Fields count: 13
            pub mod c_sound_patch {
                pub const M_PITCH: usize = 0x8; // CSoundEnvelope
                pub const M_VOLUME: usize = 0x18; // CSoundEnvelope
                pub const M_SHUTDOWN_TIME: usize = 0x30; // float32
                pub const M_FL_LAST_TIME: usize = 0x34; // float32
                pub const M_ISZ_SOUND_SCRIPT_NAME: usize = 0x38; // CUtlSymbolLarge
                pub const M_H_ENT: usize = 0x40; // CHandle< CBaseEntity >
                pub const M_SOUND_ENTITY_INDEX: usize = 0x44; // CEntityIndex
                pub const M_SOUND_ORIGIN: usize = 0x48; // 
                pub const M_IS_PLAYING: usize = 0x54; // int32
                pub const M_FILTER: usize = 0x58; // CCopyRecipientFilter
                pub const M_FL_CLOSE_CAPTION_DURATION: usize = 0x80; // float32
                pub const M_B_UPDATED_SOUND_ORIGIN: usize = 0x84; // bool
                pub const M_ISZ_CLASS_NAME: usize = 0x88; // CUtlSymbolLarge
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod c_take_damage_info_api {
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod c_pulse_graph_instance_server_entity {
                pub const M_P_COMPONENT: usize = 0xD8; // CPulseGraphComponentBase*
            }
            // Parent: None
            // Fields count: 4
            pub mod c_pulse_graph_component_base {
                pub const M_H_OWNER: usize = 0x8; // CHandle< CBaseEntity >
                pub const M_S_NAME_FIXUP_STATIC_PREFIX: usize = 0x10; // CUtlSymbolLarge
                pub const M_S_NAME_FIXUP_PARENT: usize = 0x18; // CUtlSymbolLarge
                pub const M_S_NAME_FIXUP_LOCAL: usize = 0x20; // CUtlSymbolLarge
            }
            // Parent: c_pulse_graph_component_base
            // Fields count: 0
            pub mod c_pulse_graph_component_point_server {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod c_pulse_server_funcs {
            }
            // Parent: c_pulse_cell_base_flow
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod c_pulse_cell_sound_event_start {
                pub const M_TYPE: usize = 0x48; // SoundEventStartType_t
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod c_pulse_server_funcs_sounds {
            }
            // Parent: c_pulse_cell_base_yielding_inflow
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_pulse_cell_outflow_play_scene_base {
                pub const M_ON_FINISHED: usize = 0x48; // CPulse_ResumePoint
                pub const M_ON_CANCELED: usize = 0x58; // CPulse_ResumePoint
                pub const M_TRIGGERS: usize = 0x68; // CUtlVector< CPulse_OutflowConnection >
            }
            // Parent: None
            // Fields count: 1
            pub mod c_pulse_cell_outflow_play_scene_base_cursor_state_t {
                pub const M_SCENE_INSTANCE: usize = 0x0; // CHandle< CBaseEntity >
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod pulse_scripted_sequence_data_t {
                pub const M_N_ACTOR_ID: usize = 0x0; // int32
                pub const M_SZ_PRE_IDLE_SEQUENCE: usize = 0x8; // CUtlString
                pub const M_SZ_ENTRY_SEQUENCE: usize = 0x10; // CUtlString
                pub const M_SZ_SEQUENCE: usize = 0x18; // CUtlString
                pub const M_SZ_EXIT_SEQUENCE: usize = 0x20; // CUtlString
                pub const M_B_LOOP_PRE_IDLE_SEQUENCE: usize = 0x28; // bool
                pub const M_B_LOOP_ACTION_SEQUENCE: usize = 0x29; // bool
                pub const M_B_LOOP_POST_IDLE_SEQUENCE: usize = 0x2A; // bool
                pub const M_B_IGNORE_LOOK_AT: usize = 0x2B; // bool
            }
            // Parent: None
            // Fields count: 1
            pub mod c_pulse_cell_outflow_scripted_sequence_cursor_state_t {
                pub const M_SCRIPTED_SEQUENCE: usize = 0x0; // CHandle< CBaseEntity >
            }
            // Parent: c_entity_component
            // Fields count: 0
            pub mod c_touch_expansion_component {
            }
            // Parent: None
            // Fields count: 25
            pub mod dynpitchvol_base_t {
                pub const PRESET: usize = 0x0; // int32
                pub const PITCHRUN: usize = 0x4; // int32
                pub const PITCHSTART: usize = 0x8; // int32
                pub const SPINUP: usize = 0xC; // int32
                pub const SPINDOWN: usize = 0x10; // int32
                pub const VOLRUN: usize = 0x14; // int32
                pub const VOLSTART: usize = 0x18; // int32
                pub const FADEIN: usize = 0x1C; // int32
                pub const FADEOUT: usize = 0x20; // int32
                pub const LFOTYPE: usize = 0x24; // int32
                pub const LFORATE: usize = 0x28; // int32
                pub const LFOMODPITCH: usize = 0x2C; // int32
                pub const LFOMODVOL: usize = 0x30; // int32
                pub const CSPINUP: usize = 0x34; // int32
                pub const CSPINCOUNT: usize = 0x38; // int32
                pub const PITCH: usize = 0x3C; // int32
                pub const SPINUPSAV: usize = 0x40; // int32
                pub const SPINDOWNSAV: usize = 0x44; // int32
                pub const PITCHFRAC: usize = 0x48; // int32
                pub const VOL: usize = 0x4C; // int32
                pub const FADEINSAV: usize = 0x50; // int32
                pub const FADEOUTSAV: usize = 0x54; // int32
                pub const VOLFRAC: usize = 0x58; // int32
                pub const LFOFRAC: usize = 0x5C; // int32
                pub const LFOMULT: usize = 0x60; // int32
            }
            // Parent: dynpitchvol_base_t
            // Fields count: 0
            pub mod dynpitchvol_t {
            }
            // Parent: None
            // Fields count: 3
            pub mod response_context_t {
                pub const M_ISZ_NAME: usize = 0x0; // CUtlSymbolLarge
                pub const M_ISZ_VALUE: usize = 0x8; // CUtlSymbolLarge
                pub const M_F_EXPIRATION_TIME: usize = 0x10; // GameTime_t
            }
            // Parent: None
            // Fields count: 2
            pub mod relationship_t {
                pub const DISPOSITION: usize = 0x0; // Disposition_t
                pub const PRIORITY: usize = 0x4; // int32
            }
            // Parent: c_entity_instance
            // Fields count: 75
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // NetworkVarNames: m_CBodyComponent (CBodyComponent::Storage_t)
            // NetworkVarNames: m_iHealth (int32)
            // NetworkVarNames: m_iMaxHealth (int32)
            // NetworkVarNames: m_lifeState (uint8)
            // NetworkVarNames: m_bTakesDamage (bool)
            // NetworkVarNames: m_nTakeDamageFlags (TakeDamageFlags_t)
            // NetworkVarNames: m_bIsPlatform (bool)
            // NetworkVarNames: m_MoveCollide (MoveCollide_t)
            // NetworkVarNames: m_MoveType (MoveType_t)
            // NetworkVarNames: m_nSubclassID (EntitySubclassID_t)
            // NetworkVarNames: m_flAnimTime (float32)
            // NetworkVarNames: m_flSimulationTime (float32)
            // NetworkVarNames: m_flCreateTime (GameTime_t)
            // NetworkVarNames: m_bClientSideRagdoll (bool)
            // NetworkVarNames: m_ubInterpolationFrame (uint8)
            // NetworkVarNames: m_iTeamNum (uint8)
            // NetworkVarNames: m_spawnflags (uint32)
            // NetworkVarNames: m_nNextThinkTick (GameTick_t)
            // NetworkVarNames: m_fFlags (uint32)
            // NetworkVarNames: m_vecVelocity (CNetworkVelocityVector)
            // NetworkVarNames: m_vecBaseVelocity (Vector)
            // NetworkVarNames: m_hEffectEntity (CHandle< CBaseEntity>)
            // NetworkVarNames: m_hOwnerEntity (CHandle< CBaseEntity>)
            // NetworkVarNames: m_fEffects (uint32)
            // NetworkVarNames: m_hGroundEntity (CHandle< CBaseEntity>)
            // NetworkVarNames: m_flFriction (float32)
            // NetworkVarNames: m_flElasticity (float32)
            // NetworkVarNames: m_flGravityScale (float32)
            // NetworkVarNames: m_flTimeScale (float32)
            // NetworkVarNames: m_flWaterLevel (float)
            // NetworkVarNames: m_bAnimatedEveryTick (bool)
            // NetworkVarNames: m_flNavIgnoreUntilTime (GameTime_t)
            pub mod c_base_entity {
                pub const M_C_BODY_COMPONENT: usize = 0x38; // CBodyComponent*
                pub const M_NETWORK_TRANSMIT_COMPONENT: usize = 0x40; // CNetworkTransmitComponent
                pub const M_A_THINK_FUNCTIONS: usize = 0x230; // CUtlVector< thinkfunc_t >
                pub const M_I_CURRENT_THINK_CONTEXT: usize = 0x248; // int32
                pub const M_N_LAST_THINK_TICK: usize = 0x24C; // GameTick_t
                pub const M_N_DISABLE_CONTEXT_THINK_START_TICK: usize = 0x250; // GameTick_t
                pub const M_IS_STEADY_STATE: usize = 0x260; // CBitVec< 64 >
                pub const M_LAST_NETWORK_CHANGE: usize = 0x268; // float32
                pub const M_RESPONSE_CONTEXTS: usize = 0x278; // CUtlVector< ResponseContext_t >
                pub const M_ISZ_RESPONSE_CONTEXT: usize = 0x290; // CUtlSymbolLarge
                pub const M_I_HEALTH: usize = 0x2B8; // int32
                pub const M_I_MAX_HEALTH: usize = 0x2BC; // int32
                pub const M_LIFE_STATE: usize = 0x2C0; // uint8
                pub const M_FL_DAMAGE_ACCUMULATOR: usize = 0x2C4; // float32
                pub const M_B_TAKES_DAMAGE: usize = 0x2C8; // bool
                pub const M_N_TAKE_DAMAGE_FLAGS: usize = 0x2CC; // TakeDamageFlags_t
                pub const M_B_IS_PLATFORM: usize = 0x2D0; // bool
                pub const M_MOVE_COLLIDE: usize = 0x2D2; // MoveCollide_t
                pub const M_MOVE_TYPE: usize = 0x2D3; // MoveType_t
                pub const M_N_ACTUAL_MOVE_TYPE: usize = 0x2D4; // MoveType_t
                pub const M_N_WATER_TOUCH: usize = 0x2D5; // uint8
                pub const M_N_SLIME_TOUCH: usize = 0x2D6; // uint8
                pub const M_B_RESTORE_IN_HIERARCHY: usize = 0x2D7; // bool
                pub const M_TARGET: usize = 0x2D8; // CUtlSymbolLarge
                pub const M_H_DAMAGE_FILTER: usize = 0x2E0; // CHandle< CBaseFilter >
                pub const M_ISZ_DAMAGE_FILTER_NAME: usize = 0x2E8; // CUtlSymbolLarge
                pub const M_FL_MOVE_DONE_TIME: usize = 0x2F0; // float32
                pub const M_N_SUBCLASS_ID: usize = 0x2F4; // CUtlStringToken
                pub const M_FL_ANIM_TIME: usize = 0x300; // float32
                pub const M_FL_SIMULATION_TIME: usize = 0x304; // float32
                pub const M_FL_CREATE_TIME: usize = 0x308; // GameTime_t
                pub const M_B_CLIENT_SIDE_RAGDOLL: usize = 0x30C; // bool
                pub const M_UB_INTERPOLATION_FRAME: usize = 0x30D; // uint8
                pub const M_V_PREV_V_PHYSICS_UPDATE_POS: usize = 0x310; // 
                pub const M_I_TEAM_NUM: usize = 0x31C; // uint8
                pub const M_I_GLOBALNAME: usize = 0x320; // CUtlSymbolLarge
                pub const M_I_SENT_TO_CLIENTS: usize = 0x328; // int32
                pub const M_FL_SPEED: usize = 0x32C; // float32
                pub const M_S_UNIQUE_HAMMER_ID: usize = 0x330; // CUtlString
                pub const M_SPAWNFLAGS: usize = 0x338; // uint32
                pub const M_N_NEXT_THINK_TICK: usize = 0x33C; // GameTick_t
                pub const M_N_SIMULATION_TICK: usize = 0x340; // int32
                pub const M_ON_KILLED: usize = 0x348; // CEntityIOOutput
                pub const M_F_FLAGS: usize = 0x370; // uint32
                pub const M_VEC_ABS_VELOCITY: usize = 0x374; // 
                pub const M_VEC_VELOCITY: usize = 0x380; // CNetworkVelocityVector
                pub const M_VEC_BASE_VELOCITY: usize = 0x3B0; // 
                pub const M_N_PUSH_ENUM_COUNT: usize = 0x3BC; // int32
                pub const M_P_COLLISION: usize = 0x3C0; // CCollisionProperty*
                pub const M_H_EFFECT_ENTITY: usize = 0x3C8; // CHandle< CBaseEntity >
                pub const M_H_OWNER_ENTITY: usize = 0x3CC; // CHandle< CBaseEntity >
                pub const M_F_EFFECTS: usize = 0x3D0; // uint32
                pub const M_H_GROUND_ENTITY: usize = 0x3D4; // CHandle< CBaseEntity >
                pub const M_FL_FRICTION: usize = 0x3D8; // float32
                pub const M_FL_ELASTICITY: usize = 0x3DC; // float32
                pub const M_FL_GRAVITY_SCALE: usize = 0x3E0; // float32
                pub const M_FL_TIME_SCALE: usize = 0x3E4; // float32
                pub const M_FL_WATER_LEVEL: usize = 0x3E8; // float32
                pub const M_B_ANIMATED_EVERY_TICK: usize = 0x3EC; // bool
                pub const M_B_DISABLE_LOW_VIOLENCE: usize = 0x3ED; // bool
                pub const M_N_WATER_TYPE: usize = 0x3EE; // uint8
                pub const M_I_E_FLAGS: usize = 0x3F0; // int32
                pub const M_ON_USER1: usize = 0x3F8; // CEntityIOOutput
                pub const M_ON_USER2: usize = 0x420; // CEntityIOOutput
                pub const M_ON_USER3: usize = 0x448; // CEntityIOOutput
                pub const M_ON_USER4: usize = 0x470; // CEntityIOOutput
                pub const M_I_INITIAL_TEAM_NUM: usize = 0x498; // int32
                pub const M_FL_NAV_IGNORE_UNTIL_TIME: usize = 0x49C; // GameTime_t
                pub const M_VEC_ANG_VELOCITY: usize = 0x4A0; // QAngle
                pub const M_B_NETWORK_QUANTIZE_ORIGIN_AND_ANGLES: usize = 0x4AC; // bool
                pub const M_B_LAG_COMPENSATE: usize = 0x4AD; // bool
                pub const M_FL_OVERRIDDEN_FRICTION: usize = 0x4B0; // float32
                pub const M_P_BLOCKER: usize = 0x4B4; // CHandle< CBaseEntity >
                pub const M_FL_LOCAL_TIME: usize = 0x4B8; // float32
                pub const M_FL_V_PHYSICS_UPDATE_LOCAL_TIME: usize = 0x4BC; // float32
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_server_only_entity {
            }
            // Parent: c_server_only_entity
            // Fields count: 0
            pub mod c_server_only_point_entity {
            }
            // Parent: c_server_only_entity
            // Fields count: 0
            pub mod c_logical_entity {
            }
            // Parent: c_base_entity
            // Fields count: 17
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // NetworkVarNames: m_flFadeInDuration (float32)
            // NetworkVarNames: m_flFadeOutDuration (float32)
            // NetworkVarNames: m_flMaxWeight (float32)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bMaster (bool)
            // NetworkVarNames: m_bClientSide (bool)
            // NetworkVarNames: m_bExclusive (bool)
            // NetworkVarNames: m_MinFalloff (float32)
            // NetworkVarNames: m_MaxFalloff (float32)
            // NetworkVarNames: m_flCurWeight (float32)
            // NetworkVarNames: m_netlookupFilename (char)
            pub mod c_color_correction {
                pub const M_FL_FADE_IN_DURATION: usize = 0x4C0; // float32
                pub const M_FL_FADE_OUT_DURATION: usize = 0x4C4; // float32
                pub const M_FL_START_FADE_IN_WEIGHT: usize = 0x4C8; // float32
                pub const M_FL_START_FADE_OUT_WEIGHT: usize = 0x4CC; // float32
                pub const M_FL_TIME_START_FADE_IN: usize = 0x4D0; // GameTime_t
                pub const M_FL_TIME_START_FADE_OUT: usize = 0x4D4; // GameTime_t
                pub const M_FL_MAX_WEIGHT: usize = 0x4D8; // float32
                pub const M_B_START_DISABLED: usize = 0x4DC; // bool
                pub const M_B_ENABLED: usize = 0x4DD; // bool
                pub const M_B_MASTER: usize = 0x4DE; // bool
                pub const M_B_CLIENT_SIDE: usize = 0x4DF; // bool
                pub const M_B_EXCLUSIVE: usize = 0x4E0; // bool
                pub const M_MIN_FALLOFF: usize = 0x4E4; // float32
                pub const M_MAX_FALLOFF: usize = 0x4E8; // float32
                pub const M_FL_CUR_WEIGHT: usize = 0x4EC; // float32
                pub const M_NETLOOKUP_FILENAME: usize = 0x4F0; // char[512]
                pub const M_LOOKUP_FILENAME: usize = 0x6F0; // CUtlSymbolLarge
            }
            // Parent: c_base_entity
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_hEntAttached (CHandle< CBaseEntity>)
            // NetworkVarNames: m_bCheapEffect (bool)
            pub mod c_entity_flame {
                pub const M_H_ENT_ATTACHED: usize = 0x4C0; // CHandle< CBaseEntity >
                pub const M_B_CHEAP_EFFECT: usize = 0x4C4; // bool
                pub const M_FL_SIZE: usize = 0x4C8; // float32
                pub const M_B_USE_HITBOXES: usize = 0x4CC; // bool
                pub const M_I_NUM_HITBOX_FIRES: usize = 0x4D0; // int32
                pub const M_FL_HITBOX_FIRE_SCALE: usize = 0x4D4; // float32
                pub const M_FL_LIFETIME: usize = 0x4D8; // GameTime_t
                pub const M_H_ATTACKER: usize = 0x4DC; // CHandle< CBaseEntity >
                pub const M_I_DANGER_SOUND: usize = 0x4E0; // int32
                pub const M_FL_DIRECT_DAMAGE_PER_SECOND: usize = 0x4E4; // float32
                pub const M_I_CUSTOM_DAMAGE_TYPE: usize = 0x4E8; // int32
            }
            // Parent: c_logical_entity
            // Fields count: 3
            pub mod c_base_filter {
                pub const M_B_NEGATED: usize = 0x4C0; // bool
                pub const M_ON_PASS: usize = 0x4C8; // CEntityIOOutput
                pub const M_ON_FAIL: usize = 0x4F0; // CEntityIOOutput
            }
            // Parent: c_base_filter
            // Fields count: 4
            pub mod c_filter_multiple {
                pub const M_N_FILTER_TYPE: usize = 0x518; // filter_t
                pub const M_I_FILTER_NAME: usize = 0x520; // CUtlSymbolLarge[10]
                pub const M_H_FILTER: usize = 0x570; // CHandle< CBaseEntity >[10]
                pub const M_N_FILTER_COUNT: usize = 0x598; // int32
            }
            // Parent: c_base_filter
            // Fields count: 1
            pub mod c_filter_proximity {
                pub const M_FL_RADIUS: usize = 0x518; // float32
            }
            // Parent: c_base_filter
            // Fields count: 0
            pub mod c_filter_los {
            }
            // Parent: c_base_filter
            // Fields count: 1
            pub mod c_filter_class {
                pub const M_I_FILTER_CLASS: usize = 0x518; // CUtlSymbolLarge
            }
            // Parent: c_base_entity
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_flScale (float32)
            // NetworkVarNames: m_flStartScale (float32)
            // NetworkVarNames: m_flScaleTime (float32)
            // NetworkVarNames: m_nFlags (uint32)
            pub mod c_base_fire {
                pub const M_FL_SCALE: usize = 0x4C0; // float32
                pub const M_FL_START_SCALE: usize = 0x4C4; // float32
                pub const M_FL_SCALE_TIME: usize = 0x4C8; // float32
                pub const M_N_FLAGS: usize = 0x4CC; // uint32
            }
            // Parent: c_base_fire
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_nFlameModelIndex (int32)
            // NetworkVarNames: m_nFlameFromAboveModelIndex (int32)
            pub mod c_fire_smoke {
                pub const M_N_FLAME_MODEL_INDEX: usize = 0x4D0; // int32
                pub const M_N_FLAME_FROM_ABOVE_MODEL_INDEX: usize = 0x4D4; // int32
            }
            // Parent: c_base_entity
            // Fields count: 7
            pub mod c_fish_pool {
                pub const M_FISH_COUNT: usize = 0x4D0; // int32
                pub const M_MAX_RANGE: usize = 0x4D4; // float32
                pub const M_SWIM_DEPTH: usize = 0x4D8; // float32
                pub const M_WATER_LEVEL: usize = 0x4DC; // float32
                pub const M_IS_DORMANT: usize = 0x4E0; // bool
                pub const M_FISHES: usize = 0x4E8; // CUtlVector< CHandle< CFish > >
                pub const M_VIS_TIMER: usize = 0x500; // CountdownTimer
            }
            // Parent: c_server_only_entity
            // Fields count: 0
            pub mod c_info_data {
            }
            // Parent: None
            // Fields count: 3
            pub mod locksound_t {
                pub const S_LOCKED_SOUND: usize = 0x8; // CUtlSymbolLarge
                pub const S_UNLOCKED_SOUND: usize = 0x10; // CUtlSymbolLarge
                pub const FLWAIT_SOUND: usize = 0x18; // GameTime_t
            }
            // Parent: c_logical_entity
            // Fields count: 4
            pub mod c_logic_branch {
                pub const M_B_IN_VALUE: usize = 0x4C0; // bool
                pub const M_LISTENERS: usize = 0x4C8; // CUtlVector< CHandle< CBaseEntity > >
                pub const M_ON_TRUE: usize = 0x4E0; // CEntityIOOutput
                pub const M_ON_FALSE: usize = 0x508; // CEntityIOOutput
            }
            // Parent: c_logical_entity
            // Fields count: 7
            pub mod c_logic_distance_check {
                pub const M_ISZ_ENTITY_A: usize = 0x4C0; // CUtlSymbolLarge
                pub const M_ISZ_ENTITY_B: usize = 0x4C8; // CUtlSymbolLarge
                pub const M_FL_ZONE1_DISTANCE: usize = 0x4D0; // float32
                pub const M_FL_ZONE2_DISTANCE: usize = 0x4D4; // float32
                pub const M_IN_ZONE1: usize = 0x4D8; // CEntityIOOutput
                pub const M_IN_ZONE2: usize = 0x500; // CEntityIOOutput
                pub const M_IN_ZONE3: usize = 0x528; // CEntityIOOutput
            }
            // Parent: None
            // Fields count: 3
            pub mod velocity_sampler {
                pub const M_PREV_SAMPLE: usize = 0x0; // 
                pub const M_F_PREV_SAMPLE_TIME: usize = 0xC; // GameTime_t
                pub const M_F_IDEAL_SAMPLE_RATE: usize = 0x10; // float32
            }
            // Parent: None
            // Fields count: 3
            pub mod simple_constraint_sound_profile {
                pub const E_KEYPOINTS: usize = 0x8; // SimpleConstraintSoundProfile::SimpleConstraintsSoundProfileKeypoints_t
                pub const M_KEY_POINTS: usize = 0xC; // float32[2]
                pub const M_REVERSAL_SOUND_THRESHOLDS: usize = 0x14; // float32[3]
            }
            // Parent: None
            // Fields count: 8
            pub mod constraint_sound_info {
                pub const M_V_SAMPLER: usize = 0x8; // VelocitySampler
                pub const M_SOUND_PROFILE: usize = 0x20; // SimpleConstraintSoundProfile
                pub const M_FORWARD_AXIS: usize = 0x40; // 
                pub const M_ISZ_TRAVEL_SOUND_FWD: usize = 0x50; // CUtlSymbolLarge
                pub const M_ISZ_TRAVEL_SOUND_BACK: usize = 0x58; // CUtlSymbolLarge
                pub const M_ISZ_REVERSAL_SOUNDS: usize = 0x68; // CUtlSymbolLarge[3]
                pub const M_B_PLAY_TRAVEL_SOUND: usize = 0x80; // bool
                pub const M_B_PLAY_REVERSAL_SOUND: usize = 0x81; // bool
            }
            // Parent: None
            // Fields count: 5
            pub mod c_smooth_func {
                pub const M_FL_SMOOTH_AMPLITUDE: usize = 0x8; // float32
                pub const M_FL_SMOOTH_BIAS: usize = 0xC; // float32
                pub const M_FL_SMOOTH_DURATION: usize = 0x10; // float32
                pub const M_FL_SMOOTH_REMAINING_TIME: usize = 0x14; // float32
                pub const M_N_SMOOTH_DIR: usize = 0x18; // int32
            }
            // Parent: None
            // Fields count: 1
            pub mod magnetted_objects_t {
                pub const H_ENTITY: usize = 0x8; // CHandle< CBaseEntity >
            }
            // Parent: c_server_only_point_entity
            // Fields count: 6
            pub mod c_point_prefab {
                pub const M_TARGET_MAP_NAME: usize = 0x4C0; // CUtlSymbolLarge
                pub const M_FORCE_WORLD_GROUP_ID: usize = 0x4C8; // CUtlSymbolLarge
                pub const M_ASSOCIATED_RELAY_TARGET_NAME: usize = 0x4D0; // CUtlSymbolLarge
                pub const M_FIXUP_NAMES: usize = 0x4D8; // bool
                pub const M_B_LOAD_DYNAMIC: usize = 0x4D9; // bool
                pub const M_ASSOCIATED_RELAY_ENTITY: usize = 0x4DC; // CHandle< CPointPrefab >
            }
            // Parent: c_base_entity
            // Fields count: 2
            pub mod c_skybox_reference {
                pub const M_WORLD_GROUP_ID: usize = 0x4C0; // WorldGroupId_t
                pub const M_H_SKY_CAMERA: usize = 0x4C4; // CHandle< CSkyCamera >
            }
            // Parent: c_base_entity
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_skyboxData (sky3dparams_t)
            // NetworkVarNames: m_skyboxSlotToken (CUtlStringToken)
            pub mod c_sky_camera {
                pub const M_SKYBOX_DATA: usize = 0x4C0; // sky3dparams_t
                pub const M_SKYBOX_SLOT_TOKEN: usize = 0x550; // CUtlStringToken
                pub const M_B_USE_ANGLES: usize = 0x554; // bool
                pub const M_P_NEXT: usize = 0x558; // CSkyCamera*
            }
            // Parent: None
            // Fields count: 12
            pub mod c_sound {
                pub const M_H_OWNER: usize = 0x0; // CHandle< CBaseEntity >
                pub const M_H_TARGET: usize = 0x4; // CHandle< CBaseEntity >
                pub const M_I_VOLUME: usize = 0x8; // int32
                pub const M_FL_OCCLUSION_SCALE: usize = 0xC; // float32
                pub const M_I_TYPE: usize = 0x10; // int32
                pub const M_I_NEXT_AUDIBLE: usize = 0x14; // int32
                pub const M_FL_EXPIRE_TIME: usize = 0x18; // GameTime_t
                pub const M_I_NEXT: usize = 0x1C; // int16
                pub const M_B_NO_EXPIRATION_TIME: usize = 0x1E; // bool
                pub const M_OWNER_CHANNEL_INDEX: usize = 0x20; // int32
                pub const M_VEC_ORIGIN: usize = 0x24; // 
                pub const M_B_HAS_OWNER: usize = 0x30; // bool
            }
            // Parent: c_server_only_entity
            // Fields count: 11
            pub mod c_env_soundscape {
                pub const M_ON_PLAY: usize = 0x4C0; // CEntityIOOutput
                pub const M_FL_RADIUS: usize = 0x4E8; // float32
                pub const M_SOUNDSCAPE_NAME: usize = 0x4F0; // CUtlSymbolLarge
                pub const M_SOUND_EVENT_NAME: usize = 0x4F8; // CUtlSymbolLarge
                pub const M_B_OVERRIDE_WITH_EVENT: usize = 0x500; // bool
                pub const M_SOUNDSCAPE_INDEX: usize = 0x504; // int32
                pub const M_SOUNDSCAPE_ENTITY_LIST_ID: usize = 0x508; // int32
                pub const M_SOUND_EVENT_HASH: usize = 0x50C; // uint32
                pub const M_POSITION_NAMES: usize = 0x510; // CUtlSymbolLarge[8]
                pub const M_H_PROXY_SOUNDSCAPE: usize = 0x550; // CHandle< CEnvSoundscape >
                pub const M_B_DISABLED: usize = 0x554; // bool
            }
            // Parent: c_env_soundscape
            // Fields count: 1
            pub mod c_env_soundscape_proxy {
                pub const M_MAIN_SOUNDSCAPE_NAME: usize = 0x558; // CUtlSymbolLarge
            }
            // Parent: c_env_soundscape
            // Fields count: 0
            pub mod c_env_soundscape_triggerable {
            }
            // Parent: None
            // Fields count: 6
            pub mod lerpdata_t {
                pub const M_H_ENT: usize = 0x0; // CHandle< CBaseEntity >
                pub const M_MOVE_TYPE: usize = 0x4; // MoveType_t
                pub const M_FL_START_TIME: usize = 0x8; // GameTime_t
                pub const M_VEC_START_ORIGIN: usize = 0xC; // 
                pub const M_Q_START_ROT: usize = 0x20; // Quaternion
                pub const M_N_FX_INDEX: usize = 0x30; // ParticleIndex_t
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_nav_link_animgraph_var {
                pub const M_STR_ANIMGRAPH_VAR: usize = 0x0; // CUtlString
                pub const M_UN_ALIGNMENT_DEGREES: usize = 0x8; // uint32
            }
            // Parent: c_nav_volume
            // Fields count: 0
            pub mod c_nav_volume_markup_volume {
            }
            // Parent: c_nav_volume
            // Fields count: 0
            pub mod c_nav_volume_calculated_vector {
            }
            // Parent: c_nav_volume_calculated_vector
            // Fields count: 2
            pub mod c_nav_volume_breadth_first_search {
                pub const M_V_START_POS: usize = 0xA0; // 
                pub const M_FL_SEARCH_DIST: usize = 0xAC; // float32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_nInteractsAs (uint64)
            // NetworkVarNames: m_nInteractsWith (uint64)
            // NetworkVarNames: m_nInteractsExclude (uint64)
            // NetworkVarNames: m_nEntityId (uint32)
            // NetworkVarNames: m_nOwnerId (uint32)
            // NetworkVarNames: m_nHierarchyId (uint16)
            // NetworkVarNames: m_nCollisionGroup (uint8)
            // NetworkVarNames: m_nCollisionFunctionMask (uint8)
            pub mod v_physics_collision_attribute_t {
                pub const M_N_INTERACTS_AS: usize = 0x8; // uint64
                pub const M_N_INTERACTS_WITH: usize = 0x10; // uint64
                pub const M_N_INTERACTS_EXCLUDE: usize = 0x18; // uint64
                pub const M_N_ENTITY_ID: usize = 0x20; // uint32
                pub const M_N_OWNER_ID: usize = 0x24; // uint32
                pub const M_N_HIERARCHY_ID: usize = 0x28; // uint16
                pub const M_N_COLLISION_GROUP: usize = 0x2A; // uint8
                pub const M_N_COLLISION_FUNCTION_MASK: usize = 0x2B; // uint8
            }
            // Parent: None
            // Fields count: 17
            //
            // Metadata:
            // NetworkVarNames: m_collisionAttribute (VPhysicsCollisionAttribute_t)
            // NetworkVarNames: m_vecMins (Vector)
            // NetworkVarNames: m_vecMaxs (Vector)
            // NetworkVarNames: m_usSolidFlags (uint8)
            // NetworkVarNames: m_nSolidType (SolidType_t)
            // NetworkVarNames: m_triggerBloat (uint8)
            // NetworkVarNames: m_nSurroundType (SurroundingBoundsType_t)
            // NetworkVarNames: m_CollisionGroup (uint8)
            // NetworkVarNames: m_nEnablePhysics (uint8)
            // NetworkVarNames: m_vecSpecifiedSurroundingMins (Vector)
            // NetworkVarNames: m_vecSpecifiedSurroundingMaxs (Vector)
            // NetworkVarNames: m_vCapsuleCenter1 (Vector)
            // NetworkVarNames: m_vCapsuleCenter2 (Vector)
            // NetworkVarNames: m_flCapsuleRadius (float)
            pub mod c_collision_property {
                pub const M_COLLISION_ATTRIBUTE: usize = 0x10; // VPhysicsCollisionAttribute_t
                pub const M_VEC_MINS: usize = 0x40; // 
                pub const M_VEC_MAXS: usize = 0x4C; // 
                pub const M_US_SOLID_FLAGS: usize = 0x5A; // uint8
                pub const M_N_SOLID_TYPE: usize = 0x5B; // SolidType_t
                pub const M_TRIGGER_BLOAT: usize = 0x5C; // uint8
                pub const M_N_SURROUND_TYPE: usize = 0x5D; // SurroundingBoundsType_t
                pub const M_COLLISION_GROUP: usize = 0x5E; // uint8
                pub const M_N_ENABLE_PHYSICS: usize = 0x5F; // uint8
                pub const M_FL_BOUNDING_RADIUS: usize = 0x60; // float32
                pub const M_VEC_SPECIFIED_SURROUNDING_MINS: usize = 0x64; // 
                pub const M_VEC_SPECIFIED_SURROUNDING_MAXS: usize = 0x70; // 
                pub const M_VEC_SURROUNDING_MAXS: usize = 0x7C; // 
                pub const M_VEC_SURROUNDING_MINS: usize = 0x88; // 
                pub const M_V_CAPSULE_CENTER1: usize = 0x94; // 
                pub const M_V_CAPSULE_CENTER2: usize = 0xA0; // 
                pub const M_FL_CAPSULE_RADIUS: usize = 0xAC; // float32
            }
            // Parent: None
            // Fields count: 21
            //
            // Metadata:
            // NetworkVarNames: m_vOrigin (Vector)
            // NetworkVarNames: m_vStart (Vector)
            // NetworkVarNames: m_vNormal (Vector)
            // NetworkVarNames: m_vAngles (QAngle)
            // NetworkVarNames: m_hEntity (CEntityHandle)
            // NetworkVarNames: m_hOtherEntity (CEntityHandle)
            // NetworkVarNames: m_flScale (float32)
            // NetworkVarNames: m_flMagnitude (float32)
            // NetworkVarNames: m_flRadius (float32)
            // NetworkVarNames: m_nSurfaceProp (CUtlStringToken)
            // NetworkVarNames: m_nEffectIndex (HParticleSystemDefinition)
            // NetworkVarNames: m_nDamageType (uint32)
            // NetworkVarNames: m_nPenetrate (uint8)
            // NetworkVarNames: m_nMaterial (uint16)
            // NetworkVarNames: m_nHitBox (uint16)
            // NetworkVarNames: m_nColor (uint8)
            // NetworkVarNames: m_fFlags (uint8)
            // NetworkVarNames: m_nAttachmentIndex (AttachmentHandle_t)
            // NetworkVarNames: m_nAttachmentName (CUtlStringToken)
            // NetworkVarNames: m_iEffectName (uint16)
            // NetworkVarNames: m_nExplosionType (uint8)
            pub mod c_effect_data {
                pub const M_V_ORIGIN: usize = 0x8; // 
                pub const M_V_START: usize = 0x14; // 
                pub const M_V_NORMAL: usize = 0x20; // 
                pub const M_V_ANGLES: usize = 0x2C; // QAngle
                pub const M_H_ENTITY: usize = 0x38; // CEntityHandle
                pub const M_H_OTHER_ENTITY: usize = 0x3C; // CEntityHandle
                pub const M_FL_SCALE: usize = 0x40; // float32
                pub const M_FL_MAGNITUDE: usize = 0x44; // float32
                pub const M_FL_RADIUS: usize = 0x48; // float32
                pub const M_N_SURFACE_PROP: usize = 0x4C; // CUtlStringToken
                pub const M_N_EFFECT_INDEX: usize = 0x50; // CWeakHandle< InfoForResourceTypeIParticleSystemDefinition >
                pub const M_N_DAMAGE_TYPE: usize = 0x58; // uint32
                pub const M_N_PENETRATE: usize = 0x5C; // uint8
                pub const M_N_MATERIAL: usize = 0x5E; // uint16
                pub const M_N_HIT_BOX: usize = 0x60; // uint16
                pub const M_N_COLOR: usize = 0x62; // uint8
                pub const M_F_FLAGS: usize = 0x63; // uint8
                pub const M_N_ATTACHMENT_INDEX: usize = 0x64; // AttachmentHandle_t
                pub const M_N_ATTACHMENT_NAME: usize = 0x68; // CUtlStringToken
                pub const M_I_EFFECT_NAME: usize = 0x6C; // uint16
                pub const M_N_EXPLOSION_TYPE: usize = 0x6E; // uint8
            }
            // Parent: c_base_entity
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_flFadeStartDist (float32)
            // NetworkVarNames: m_flFadeEndDist (float32)
            pub mod c_env_detail_controller {
                pub const M_FL_FADE_START_DIST: usize = 0x4C0; // float32
                pub const M_FL_FADE_END_DIST: usize = 0x4C4; // float32
            }
            // Parent: None
            // Fields count: 31
            //
            // Metadata:
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_iWindSeed (uint32)
            // NetworkVarNames: m_iMinWind (uint16)
            // NetworkVarNames: m_iMaxWind (uint16)
            // NetworkVarNames: m_windRadius (int32)
            // NetworkVarNames: m_iMinGust (uint16)
            // NetworkVarNames: m_iMaxGust (uint16)
            // NetworkVarNames: m_flMinGustDelay (float32)
            // NetworkVarNames: m_flMaxGustDelay (float32)
            // NetworkVarNames: m_flGustDuration (float32)
            // NetworkVarNames: m_iGustDirChange (uint16)
            // NetworkVarNames: m_location (Vector)
            // NetworkVarNames: m_iInitialWindDir (uint16)
            // NetworkVarNames: m_flInitialWindSpeed (float32)
            pub mod c_env_wind_shared {
                pub const M_FL_START_TIME: usize = 0x8; // GameTime_t
                pub const M_I_WIND_SEED: usize = 0xC; // uint32
                pub const M_I_MIN_WIND: usize = 0x10; // uint16
                pub const M_I_MAX_WIND: usize = 0x12; // uint16
                pub const M_WIND_RADIUS: usize = 0x14; // int32
                pub const M_I_MIN_GUST: usize = 0x18; // uint16
                pub const M_I_MAX_GUST: usize = 0x1A; // uint16
                pub const M_FL_MIN_GUST_DELAY: usize = 0x1C; // float32
                pub const M_FL_MAX_GUST_DELAY: usize = 0x20; // float32
                pub const M_FL_GUST_DURATION: usize = 0x24; // float32
                pub const M_I_GUST_DIR_CHANGE: usize = 0x28; // uint16
                pub const M_LOCATION: usize = 0x2C; // 
                pub const M_ISZ_GUST_SOUND: usize = 0x38; // int32
                pub const M_I_WIND_DIR: usize = 0x3C; // int32
                pub const M_FL_WIND_SPEED: usize = 0x40; // float32
                pub const M_CURRENT_WIND_VECTOR: usize = 0x44; // 
                pub const M_CURRENT_SWAY_VECTOR: usize = 0x50; // 
                pub const M_PREV_SWAY_VECTOR: usize = 0x5C; // 
                pub const M_I_INITIAL_WIND_DIR: usize = 0x68; // uint16
                pub const M_FL_INITIAL_WIND_SPEED: usize = 0x6C; // float32
                pub const M_ON_GUST_START: usize = 0x70; // CEntityIOOutput
                pub const M_ON_GUST_END: usize = 0x98; // CEntityIOOutput
                pub const M_FL_VARIATION_TIME: usize = 0xC0; // GameTime_t
                pub const M_FL_SWAY_TIME: usize = 0xC4; // GameTime_t
                pub const M_FL_SIM_TIME: usize = 0xC8; // GameTime_t
                pub const M_FL_SWITCH_TIME: usize = 0xCC; // GameTime_t
                pub const M_FL_AVE_WIND_SPEED: usize = 0xD0; // float32
                pub const M_B_GUSTING: usize = 0xD4; // bool
                pub const M_FL_WIND_ANGLE_VARIATION: usize = 0xD8; // float32
                pub const M_FL_WIND_SPEED_VARIATION: usize = 0xDC; // float32
                pub const M_I_ENT_INDEX: usize = 0xE0; // CEntityIndex
            }
            // Parent: None
            // Fields count: 2
            pub mod c_env_wind_shared_wind_ave_event_t {
                pub const M_FL_START_WIND_SPEED: usize = 0x0; // float32
                pub const M_FL_AVE_WIND_SPEED: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 2
            pub mod c_env_wind_shared_wind_variation_event_t {
                pub const M_FL_WIND_ANGLE_VARIATION: usize = 0x0; // float32
                pub const M_FL_WIND_SPEED_VARIATION: usize = 0x4; // float32
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_info_ladder_dismount {
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // NetworkVarNames: m_nModelID (int32)
            // NetworkVarNames: m_hMaterial (HMaterialStrong)
            // NetworkVarNames: m_solid (ShardSolid_t)
            // NetworkVarNames: m_ShatterPanelMode (ShatterPanelMode)
            // NetworkVarNames: m_vecPanelSize (Vector2D)
            // NetworkVarNames: m_vecStressPositionA (Vector2D)
            // NetworkVarNames: m_vecStressPositionB (Vector2D)
            // NetworkVarNames: m_vecPanelVertices (Vector2D)
            // NetworkVarNames: m_flGlassHalfThickness (float)
            // NetworkVarNames: m_bHasParent (bool)
            // NetworkVarNames: m_bParentFrozen (bool)
            // NetworkVarNames: m_SurfacePropStringToken (CUtlStringToken)
            pub mod shard_model_desc_t {
                pub const M_N_MODEL_ID: usize = 0x8; // int32
                pub const M_H_MATERIAL: usize = 0x10; // CStrongHandle< InfoForResourceTypeIMaterial2 >
                pub const M_SOLID: usize = 0x18; // ShardSolid_t
                pub const M_SHATTER_PANEL_MODE: usize = 0x19; // ShatterPanelMode
                pub const M_VEC_PANEL_SIZE: usize = 0x1C; // Vector2D
                pub const M_VEC_STRESS_POSITION_A: usize = 0x24; // Vector2D
                pub const M_VEC_STRESS_POSITION_B: usize = 0x2C; // Vector2D
                pub const M_VEC_PANEL_VERTICES: usize = 0x38; // CNetworkUtlVectorBase< Vector2D >
                pub const M_FL_GLASS_HALF_THICKNESS: usize = 0x50; // float32
                pub const M_B_HAS_PARENT: usize = 0x54; // bool
                pub const M_B_PARENT_FROZEN: usize = 0x55; // bool
                pub const M_SURFACE_PROP_STRING_TOKEN: usize = 0x58; // CUtlStringToken
            }
            // Parent: None
            // Fields count: 30
            pub mod c_shatter_glass_shard {
                pub const M_H_SHARD_HANDLE: usize = 0x8; // uint32
                pub const M_VEC_PANEL_VERTICES: usize = 0x10; // CUtlVector< Vector2D >
                pub const M_V_LOCAL_PANEL_SPACE_ORIGIN: usize = 0x28; // Vector2D
                pub const M_H_MODEL: usize = 0x30; // CStrongHandle< InfoForResourceTypeCModel >
                pub const M_H_PHYSICS_ENTITY: usize = 0x38; // CHandle< CShatterGlassShardPhysics >
                pub const M_H_PARENT_PANEL: usize = 0x3C; // CHandle< CFuncShatterglass >
                pub const M_H_PARENT_SHARD: usize = 0x40; // uint32
                pub const M_SHATTER_STRESS_TYPE: usize = 0x44; // ShatterGlassStressType
                pub const M_VEC_STRESS_VELOCITY: usize = 0x48; // 
                pub const M_B_CREATED_MODEL: usize = 0x54; // bool
                pub const M_FL_LONGEST_EDGE: usize = 0x58; // float32
                pub const M_FL_SHORTEST_EDGE: usize = 0x5C; // float32
                pub const M_FL_LONGEST_ACROSS: usize = 0x60; // float32
                pub const M_FL_SHORTEST_ACROSS: usize = 0x64; // float32
                pub const M_FL_SUM_OF_ALL_EDGES: usize = 0x68; // float32
                pub const M_FL_AREA: usize = 0x6C; // float32
                pub const M_N_ON_FRAME_EDGE: usize = 0x70; // OnFrame
                pub const M_N_PARENT_PANELS_NTH_SHARD: usize = 0x74; // int32
                pub const M_N_SUB_SHARD_GENERATION: usize = 0x78; // int32
                pub const M_VEC_AVERAGE_VERT_POSITION: usize = 0x7C; // Vector2D
                pub const M_B_AVERAGE_VERT_POSITION_IS_VALID: usize = 0x84; // bool
                pub const M_VEC_PANEL_SPACE_STRESS_POSITION_A: usize = 0x88; // Vector2D
                pub const M_VEC_PANEL_SPACE_STRESS_POSITION_B: usize = 0x90; // Vector2D
                pub const M_B_STRESS_POSITION_A_IS_VALID: usize = 0x98; // bool
                pub const M_B_STRESS_POSITION_B_IS_VALID: usize = 0x99; // bool
                pub const M_B_FLAGGED_FOR_REMOVAL: usize = 0x9A; // bool
                pub const M_FL_PHYSICS_ENTITY_SPAWNED_AT_TIME: usize = 0x9C; // GameTime_t
                pub const M_B_SHATTER_RATE_LIMITED: usize = 0xA0; // bool
                pub const M_H_ENTITY_HITTING_ME: usize = 0xA4; // CHandle< CBaseEntity >
                pub const M_VEC_NEIGHBORS: usize = 0xA8; // CUtlVector< uint32 >
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod c_game_rules_proxy {
            }
            // Parent: None
            // Fields count: 2
            pub mod c_game_rules {
                pub const M_SZ_QUEST_NAME: usize = 0x8; // char[128]
                pub const M_N_QUEST_PHASE: usize = 0x88; // int32
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_iGlowType (int32)
            // NetworkVarNames: m_iGlowTeam (int32)
            // NetworkVarNames: m_nGlowRange (int32)
            // NetworkVarNames: m_nGlowRangeMin (int32)
            // NetworkVarNames: m_glowColorOverride (Color)
            // NetworkVarNames: m_bFlashing (bool)
            // NetworkVarNames: m_flGlowTime (float)
            // NetworkVarNames: m_flGlowStartTime (float)
            // NetworkVarNames: m_bEligibleForScreenHighlight (bool)
            pub mod c_glow_property {
                pub const M_F_GLOW_COLOR: usize = 0x8; // 
                pub const M_I_GLOW_TYPE: usize = 0x30; // int32
                pub const M_I_GLOW_TEAM: usize = 0x34; // int32
                pub const M_N_GLOW_RANGE: usize = 0x38; // int32
                pub const M_N_GLOW_RANGE_MIN: usize = 0x3C; // int32
                pub const M_GLOW_COLOR_OVERRIDE: usize = 0x40; // Color
                pub const M_B_FLASHING: usize = 0x44; // bool
                pub const M_FL_GLOW_TIME: usize = 0x48; // float32
                pub const M_FL_GLOW_START_TIME: usize = 0x4C; // float32
                pub const M_B_ELIGIBLE_FOR_SCREEN_HIGHLIGHT: usize = 0x50; // bool
                pub const M_B_GLOWING: usize = 0x51; // bool
            }
            // Parent: c_game_rules
            // Fields count: 0
            pub mod c_multiplay_rules {
            }
            // Parent: None
            // Fields count: 25
            //
            // Metadata:
            // NetworkVarNames: dirPrimary (Vector)
            // NetworkVarNames: colorPrimary (Color)
            // NetworkVarNames: colorSecondary (Color)
            // NetworkVarNames: colorPrimaryLerpTo (Color)
            // NetworkVarNames: colorSecondaryLerpTo (Color)
            // NetworkVarNames: start (float32)
            // NetworkVarNames: end (float32)
            // NetworkVarNames: farz (float32)
            // NetworkVarNames: maxdensity (float32)
            // NetworkVarNames: exponent (float32)
            // NetworkVarNames: HDRColorScale (float32)
            // NetworkVarNames: skyboxFogFactor (float32)
            // NetworkVarNames: skyboxFogFactorLerpTo (float32)
            // NetworkVarNames: startLerpTo (float32)
            // NetworkVarNames: endLerpTo (float32)
            // NetworkVarNames: maxdensityLerpTo (float32)
            // NetworkVarNames: lerptime (GameTime_t)
            // NetworkVarNames: duration (float32)
            // NetworkVarNames: blendtobackground (float32)
            // NetworkVarNames: scattering (float32)
            // NetworkVarNames: locallightscale (float32)
            // NetworkVarNames: enable (bool)
            // NetworkVarNames: blend (bool)
            // NetworkVarNames: m_bNoReflectionFog (bool)
            pub mod fogparams_t {
                pub const DIR_PRIMARY: usize = 0x8; // 
                pub const COLOR_PRIMARY: usize = 0x14; // Color
                pub const COLOR_SECONDARY: usize = 0x18; // Color
                pub const COLOR_PRIMARY_LERP_TO: usize = 0x1C; // Color
                pub const COLOR_SECONDARY_LERP_TO: usize = 0x20; // Color
                pub const START: usize = 0x24; // float32
                pub const END: usize = 0x28; // float32
                pub const FARZ: usize = 0x2C; // float32
                pub const MAXDENSITY: usize = 0x30; // float32
                pub const EXPONENT: usize = 0x34; // float32
                pub const HDR_COLOR_SCALE: usize = 0x38; // float32
                pub const SKYBOX_FOG_FACTOR: usize = 0x3C; // float32
                pub const SKYBOX_FOG_FACTOR_LERP_TO: usize = 0x40; // float32
                pub const START_LERP_TO: usize = 0x44; // float32
                pub const END_LERP_TO: usize = 0x48; // float32
                pub const MAXDENSITY_LERP_TO: usize = 0x4C; // float32
                pub const LERPTIME: usize = 0x50; // GameTime_t
                pub const DURATION: usize = 0x54; // float32
                pub const BLENDTOBACKGROUND: usize = 0x58; // float32
                pub const SCATTERING: usize = 0x5C; // float32
                pub const LOCALLIGHTSCALE: usize = 0x60; // float32
                pub const ENABLE: usize = 0x64; // bool
                pub const BLEND: usize = 0x65; // bool
                pub const M_B_NO_REFLECTION_FOG: usize = 0x66; // bool
                pub const M_B_PADDING: usize = 0x67; // bool
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_hCtrl (CHandle< CFogController>)
            pub mod fogplayerparams_t {
                pub const M_H_CTRL: usize = 0x8; // CHandle< CFogController >
                pub const M_FL_TRANSITION_TIME: usize = 0xC; // float32
                pub const M_OLD_COLOR: usize = 0x10; // Color
                pub const M_FL_OLD_START: usize = 0x14; // float32
                pub const M_FL_OLD_END: usize = 0x18; // float32
                pub const M_FL_OLD_MAX_DENSITY: usize = 0x1C; // float32
                pub const M_FL_OLD_HDR_COLOR_SCALE: usize = 0x20; // float32
                pub const M_FL_OLD_FAR_Z: usize = 0x24; // float32
                pub const M_NEW_COLOR: usize = 0x28; // Color
                pub const M_FL_NEW_START: usize = 0x2C; // float32
                pub const M_FL_NEW_END: usize = 0x30; // float32
                pub const M_FL_NEW_MAX_DENSITY: usize = 0x34; // float32
                pub const M_FL_NEW_HDR_COLOR_SCALE: usize = 0x38; // float32
                pub const M_FL_NEW_FAR_Z: usize = 0x3C; // float32
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: scale (int16)
            // NetworkVarNames: origin (Vector)
            // NetworkVarNames: bClip3DSkyBoxNearToWorldFar (bool)
            // NetworkVarNames: flClip3DSkyBoxNearToWorldFarOffset (float32)
            // NetworkVarNames: fog (fogparams_t)
            // NetworkVarNames: m_nWorldGroupID (WorldGroupId_t)
            pub mod sky3dparams_t {
                pub const SCALE: usize = 0x8; // int16
                pub const ORIGIN: usize = 0xC; // 
                pub const B_CLIP3_D_SKY_BOX_NEAR_TO_WORLD_FAR: usize = 0x18; // bool
                pub const FL_CLIP3_D_SKY_BOX_NEAR_TO_WORLD_FAR_OFFSET: usize = 0x1C; // float32
                pub const FOG: usize = 0x20; // fogparams_t
                pub const M_N_WORLD_GROUP_ID: usize = 0x88; // WorldGroupId_t
            }
            // Parent: None
            // Fields count: 5
            pub mod sndopvarlatchdata_t {
                pub const M_ISZ_STACK: usize = 0x8; // CUtlSymbolLarge
                pub const M_ISZ_OPERATOR: usize = 0x10; // CUtlSymbolLarge
                pub const M_ISZ_OPVAR: usize = 0x18; // CUtlSymbolLarge
                pub const M_FL_VAL: usize = 0x20; // float32
                pub const M_V_POS: usize = 0x24; // 
            }
            // Parent: None
            // Fields count: 0
            pub mod i_ragdoll {
            }
            // Parent: None
            // Fields count: 3
            pub mod ragdollelement_t {
                pub const ORIGIN_PARENT_SPACE: usize = 0x0; // 
                pub const PARENT_INDEX: usize = 0x20; // int32
                pub const M_FL_RADIUS: usize = 0x24; // float32
            }
            // Parent: None
            // Fields count: 4
            pub mod ragdoll_t {
                pub const LIST: usize = 0x0; // CUtlVector< ragdollelement_t >
                pub const BONE_INDEX: usize = 0x18; // CUtlVector< int32 >
                pub const ALLOW_STRETCH: usize = 0x30; // bool
                pub const UNUSED: usize = 0x31; // bool
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_Transforms (CTransform)
            // NetworkVarNames: m_hOwner (EHANDLE)
            pub mod physics_ragdoll_pose_t {
                pub const M_P_CHAIN_ENTITY: usize = 0x8; // CNetworkVarChainer
                pub const M_TRANSFORMS: usize = 0x30; // CNetworkUtlVectorBase< CTransform >
                pub const M_H_OWNER: usize = 0x48; // CHandle< CBaseEntity >
            }
            // Parent: None
            // Fields count: 13
            pub mod c_scene_event_info {
                pub const M_I_LAYER: usize = 0x0; // int32
                pub const M_I_PRIORITY: usize = 0x4; // int32
                pub const M_H_SEQUENCE: usize = 0x8; // HSequence
                pub const M_FL_WEIGHT: usize = 0xC; // float32
                pub const M_B_HAS_ARRIVED: usize = 0x10; // bool
                pub const M_N_TYPE: usize = 0x14; // int32
                pub const M_FL_NEXT: usize = 0x18; // GameTime_t
                pub const M_B_IS_GESTURE: usize = 0x1C; // bool
                pub const M_B_SHOULD_REMOVE: usize = 0x1D; // bool
                pub const M_H_TARGET: usize = 0x44; // CHandle< CBaseEntity >
                pub const M_N_SCENE_EVENT_ID: usize = 0x48; // uint32
                pub const M_B_CLIENT_SIDE: usize = 0x4C; // bool
                pub const M_B_STARTED: usize = 0x4D; // bool
            }
            // Parent: None
            // Fields count: 4
            pub mod thinkfunc_t {
                pub const M_H_FN: usize = 0x8; // HSCRIPT
                pub const M_N_CONTEXT: usize = 0x10; // CUtlStringToken
                pub const M_N_NEXT_THINK_TICK: usize = 0x14; // GameTick_t
                pub const M_N_LAST_THINK_TICK: usize = 0x18; // GameTick_t
            }
            // Parent: None
            // Fields count: 2
            pub mod ragdoll_creation_params_t {
                pub const M_V_FORCE: usize = 0x0; // 
                pub const M_N_FORCE_BONE: usize = 0xC; // int32
            }
            // Parent: None
            // Fields count: 6
            pub mod hudtextparms_t {
                pub const COLOR1: usize = 0x0; // Color
                pub const COLOR2: usize = 0x4; // Color
                pub const EFFECT: usize = 0x8; // uint8
                pub const CHANNEL: usize = 0x9; // uint8
                pub const X: usize = 0xC; // float32
                pub const Y: usize = 0x10; // float32
            }
            // Parent: None
            // Fields count: 2
            pub mod c_simple_sim_timer {
                pub const M_NEXT: usize = 0x0; // GameTime_t
                pub const M_N_WORLD_GROUP_ID: usize = 0x4; // WorldGroupId_t
            }
            // Parent: c_simple_sim_timer
            // Fields count: 1
            pub mod c_sim_timer {
                pub const M_INTERVAL: usize = 0x8; // float32
            }
            // Parent: c_simple_sim_timer
            // Fields count: 2
            pub mod c_rand_sim_timer {
                pub const M_MIN_INTERVAL: usize = 0x8; // float32
                pub const M_MAX_INTERVAL: usize = 0xC; // float32
            }
            // Parent: c_simple_sim_timer
            // Fields count: 1
            pub mod c_stopwatch_base {
                pub const M_F_IS_RUNNING: usize = 0x8; // bool
            }
            // Parent: c_stopwatch_base
            // Fields count: 0
            pub mod c_simple_stopwatch {
            }
            // Parent: c_stopwatch_base
            // Fields count: 1
            pub mod c_stopwatch {
                pub const M_INTERVAL: usize = 0xC; // float32
            }
            // Parent: c_stopwatch_base
            // Fields count: 2
            pub mod c_rand_stopwatch {
                pub const M_MIN_INTERVAL: usize = 0xC; // float32
                pub const M_MAX_INTERVAL: usize = 0x10; // float32
            }
            // Parent: c_game_rules
            // Fields count: 1
            pub mod c_singleplay_rules {
                pub const M_B_SINGLE_PLAYER_GAME_ENDING: usize = 0x90; // bool
            }
            // Parent: c_base_entity
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_iszStackName (string_t)
            // NetworkVarNames: m_iszOperatorName (string_t)
            // NetworkVarNames: m_iszOpvarName (string_t)
            // NetworkVarNames: m_iOpvarIndex (int)
            // NetworkVarNames: m_bUseAutoCompare (bool)
            pub mod c_sound_opvar_set_point_base {
                pub const M_B_DISABLED: usize = 0x4C0; // bool
                pub const M_H_SOURCE: usize = 0x4C4; // CEntityHandle
                pub const M_ISZ_SOURCE_ENTITY_NAME: usize = 0x4D0; // CUtlSymbolLarge
                pub const M_V_LAST_POSITION: usize = 0x528; // 
                pub const M_ISZ_STACK_NAME: usize = 0x538; // CUtlSymbolLarge
                pub const M_ISZ_OPERATOR_NAME: usize = 0x540; // CUtlSymbolLarge
                pub const M_ISZ_OPVAR_NAME: usize = 0x548; // CUtlSymbolLarge
                pub const M_I_OPVAR_INDEX: usize = 0x550; // int32
                pub const M_B_USE_AUTO_COMPARE: usize = 0x554; // bool
            }
            // Parent: c_sound_opvar_set_point_base
            // Fields count: 23
            pub mod c_sound_opvar_set_point_entity {
                pub const M_ON_ENTER: usize = 0x558; // CEntityIOOutput
                pub const M_ON_EXIT: usize = 0x580; // CEntityIOOutput
                pub const M_B_AUTO_DISABLE: usize = 0x5A8; // bool
                pub const M_FL_DISTANCE_MIN: usize = 0x5EC; // float32
                pub const M_FL_DISTANCE_MAX: usize = 0x5F0; // float32
                pub const M_FL_DISTANCE_MAP_MIN: usize = 0x5F4; // float32
                pub const M_FL_DISTANCE_MAP_MAX: usize = 0x5F8; // float32
                pub const M_FL_OCCLUSION_RADIUS: usize = 0x5FC; // float32
                pub const M_FL_OCCLUSION_MIN: usize = 0x600; // float32
                pub const M_FL_OCCLUSION_MAX: usize = 0x604; // float32
                pub const M_FL_VAL_SET_ON_DISABLE: usize = 0x608; // float32
                pub const M_B_SET_VALUE_ON_DISABLE: usize = 0x60C; // bool
                pub const M_N_SIMULATION_MODE: usize = 0x610; // int32
                pub const M_N_VISIBILITY_SAMPLES: usize = 0x614; // int32
                pub const M_V_DYNAMIC_PROXY_POINT: usize = 0x618; // 
                pub const M_FL_DYNAMIC_MAXIMUM_OCCLUSION: usize = 0x624; // float32
                pub const M_H_DYNAMIC_ENTITY: usize = 0x628; // CEntityHandle
                pub const M_ISZ_DYNAMIC_ENTITY_NAME: usize = 0x630; // CUtlSymbolLarge
                pub const M_FL_PATHING_DISTANCE_NORM_FACTOR: usize = 0x638; // float32
                pub const M_V_PATHING_SOURCE_POS: usize = 0x63C; // 
                pub const M_V_PATHING_LISTENER_POS: usize = 0x648; // 
                pub const M_V_PATHING_DIRECTION: usize = 0x654; // 
                pub const M_N_PATHING_SOURCE_INDEX: usize = 0x660; // int32
            }
            // Parent: c_sound_opvar_set_point_entity
            // Fields count: 9
            pub mod c_sound_opvar_set_aabb_entity {
                pub const M_V_DISTANCE_INNER_MINS: usize = 0x668; // 
                pub const M_V_DISTANCE_INNER_MAXS: usize = 0x674; // 
                pub const M_V_DISTANCE_OUTER_MINS: usize = 0x680; // 
                pub const M_V_DISTANCE_OUTER_MAXS: usize = 0x68C; // 
                pub const M_N_AABB_DIRECTION: usize = 0x698; // int32
                pub const M_V_INNER_MINS: usize = 0x69C; // 
                pub const M_V_INNER_MAXS: usize = 0x6A8; // 
                pub const M_V_OUTER_MINS: usize = 0x6B4; // 
                pub const M_V_OUTER_MAXS: usize = 0x6C0; // 
            }
            // Parent: c_sound_opvar_set_aabb_entity
            // Fields count: 0
            pub mod c_sound_opvar_set_obb_entity {
            }
            // Parent: c_sound_opvar_set_point_entity
            // Fields count: 3
            pub mod c_sound_opvar_set_path_corner_entity {
                pub const M_FL_DIST_MIN_SQR: usize = 0x680; // float32
                pub const M_FL_DIST_MAX_SQR: usize = 0x684; // float32
                pub const M_ISZ_PATH_CORNER_ENTITY_NAME: usize = 0x688; // CUtlSymbolLarge
            }
            // Parent: None
            // Fields count: 3
            pub mod sound_opvar_trace_result_t {
                pub const V_POS: usize = 0x0; // 
                pub const B_DID_HIT: usize = 0xC; // bool
                pub const FL_DIST_SQR_TO_CENTER: usize = 0x10; // float32
            }
            // Parent: c_sound_opvar_set_point_entity
            // Fields count: 4
            pub mod c_sound_opvar_set_auto_room_entity {
                pub const M_TRACE_RESULTS: usize = 0x668; // CUtlVector< SoundOpvarTraceResult_t >
                pub const M_FL_SIZE: usize = 0x698; // float32
                pub const M_FL_HEIGHT_TOLERANCE: usize = 0x69C; // float32
                pub const M_FL_SIZE_SQR: usize = 0x6A0; // float32
            }
            // Parent: c_sound_opvar_set_point_base
            // Fields count: 8
            pub mod c_sound_opvar_set_obb_wind_entity {
                pub const M_V_MINS: usize = 0x558; // 
                pub const M_V_MAXS: usize = 0x564; // 
                pub const M_V_DISTANCE_MINS: usize = 0x570; // 
                pub const M_V_DISTANCE_MAXS: usize = 0x57C; // 
                pub const M_FL_WIND_MIN: usize = 0x588; // float32
                pub const M_FL_WIND_MAX: usize = 0x58C; // float32
                pub const M_FL_WIND_MAP_MIN: usize = 0x590; // float32
                pub const M_FL_WIND_MAP_MAX: usize = 0x594; // float32
            }
            // Parent: None
            // Fields count: 18
            pub mod c_take_damage_info {
                pub const M_VEC_DAMAGE_FORCE: usize = 0x8; // 
                pub const M_VEC_DAMAGE_POSITION: usize = 0x14; // 
                pub const M_VEC_REPORTED_POSITION: usize = 0x20; // 
                pub const M_VEC_DAMAGE_DIRECTION: usize = 0x2C; // 
                pub const M_H_INFLICTOR: usize = 0x38; // CHandle< CBaseEntity >
                pub const M_H_ATTACKER: usize = 0x3C; // CHandle< CBaseEntity >
                pub const M_H_ABILITY: usize = 0x40; // CHandle< CBaseEntity >
                pub const M_FL_DAMAGE: usize = 0x44; // float32
                pub const M_BITS_DAMAGE_TYPE: usize = 0x48; // int32
                pub const M_I_DAMAGE_CUSTOM: usize = 0x4C; // int32
                pub const M_I_AMMO_TYPE: usize = 0x50; // AmmoIndex_t
                pub const M_FL_ORIGINAL_DAMAGE: usize = 0x60; // float32
                pub const M_B_SHOULD_BLEED: usize = 0x64; // bool
                pub const M_B_SHOULD_SPARK: usize = 0x65; // bool
                pub const M_N_DAMAGE_FLAGS: usize = 0x70; // TakeDamageFlags_t
                pub const M_N_NUM_OBJECTS_PENETRATED: usize = 0x74; // int32
                pub const M_H_SCRIPT_INSTANCE: usize = 0x78; // HSCRIPT
                pub const M_B_IN_TAKE_DAMAGE_FLOW: usize = 0x94; // bool
            }
            // Parent: None
            // Fields count: 2
            pub mod c_take_damage_result {
                pub const M_N_HEALTH_LOST: usize = 0x0; // int32
                pub const M_N_DAMAGE_TAKEN: usize = 0x4; // int32
            }
            // Parent: None
            // Fields count: 4
            pub mod summary_take_damage_info_t {
                pub const N_SUMMARISED_COUNT: usize = 0x0; // int32
                pub const INFO: usize = 0x8; // CTakeDamageInfo
                pub const RESULT: usize = 0xA0; // CTakeDamageResult
                pub const H_TARGET: usize = 0xA8; // CHandle< CBaseEntity >
            }
            // Parent: None
            // Fields count: 1
            pub mod c_take_damage_summary_scope_guard {
                pub const M_VEC_SUMMARIES: usize = 0x8; // CUtlVector< SummaryTakeDamageInfo_t* >
            }
            // Parent: c_multiplay_rules
            // Fields count: 0
            pub mod c_teamplay_rules {
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_Attributes (CEconItemAttribute)
            pub mod c_attribute_list {
                pub const M_ATTRIBUTES: usize = 0x8; // CUtlVectorEmbeddedNetworkVar< CEconItemAttribute >
                pub const M_P_MANAGER: usize = 0x58; // CAttributeManager*
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_iAttributeDefinitionIndex (attrib_definition_index_t)
            // NetworkVarNames: m_flValue (float)
            // NetworkVarNames: m_flInitialValue (float)
            // NetworkVarNames: m_nRefundableCurrency (int)
            // NetworkVarNames: m_bSetBonus (bool)
            pub mod c_econ_item_attribute {
                pub const M_I_ATTRIBUTE_DEFINITION_INDEX: usize = 0x30; // uint16
                pub const M_FL_VALUE: usize = 0x34; // float32
                pub const M_FL_INITIAL_VALUE: usize = 0x38; // float32
                pub const M_N_REFUNDABLE_CURRENCY: usize = 0x3C; // int32
                pub const M_B_SET_BONUS: usize = 0x40; // bool
            }
            // Parent: None
            // Fields count: 0
            pub mod i_econ_item_interface {
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: m_iReapplyProvisionParity (int)
            // NetworkVarNames: m_hOuter (EHANDLE)
            // NetworkVarNames: m_ProviderType (attributeprovidertypes_t)
            pub mod c_attribute_manager {
                pub const M_PROVIDERS: usize = 0x8; // CUtlVector< CHandle< CBaseEntity > >
                pub const M_I_REAPPLY_PROVISION_PARITY: usize = 0x20; // int32
                pub const M_H_OUTER: usize = 0x24; // CHandle< CBaseEntity >
                pub const M_B_PREVENT_LOOPBACK: usize = 0x28; // bool
                pub const M_PROVIDER_TYPE: usize = 0x2C; // attributeprovidertypes_t
                pub const M_CACHED_RESULTS: usize = 0x30; // CUtlVector< CAttributeManager::cached_attribute_float_t >
            }
            // Parent: None
            // Fields count: 3
            pub mod c_attribute_manager_cached_attribute_float_t {
                pub const FL_IN: usize = 0x0; // float32
                pub const I_ATTRIB_HOOK: usize = 0x8; // CUtlSymbolLarge
                pub const FL_OUT: usize = 0x10; // float32
            }
            // Parent: c_attribute_manager
            // Fields count: 1
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_Item (CEconItemView)
            pub mod c_attribute_container {
                pub const M_ITEM: usize = 0x50; // CEconItemView
            }
            // Parent: None
            // Fields count: 0
            pub mod i_has_attributes {
            }
            // Parent: ammo_type_info_t
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod game_ammo_type_info_t {
                pub const M_N_BUY_SIZE: usize = 0x38; // int32
                pub const M_N_COST: usize = 0x3C; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_bSpotted (bool)
            // NetworkVarNames: m_bSpottedByMask (uint32)
            pub mod entity_spotted_state_t {
                pub const M_B_SPOTTED: usize = 0x8; // bool
                pub const M_B_SPOTTED_BY_MASK: usize = 0xC; // uint32[2]
            }
            // Parent: c_server_only_point_entity
            // Fields count: 3
            pub mod spawn_point {
                pub const M_I_PRIORITY: usize = 0x4C0; // int32
                pub const M_B_ENABLED: usize = 0x4C4; // bool
                pub const M_N_TYPE: usize = 0x4C8; // int32
            }
            // Parent: spawn_point
            // Fields count: 9
            pub mod spawn_point_coop_enemy {
                pub const M_SZ_WEAPONS_TO_GIVE: usize = 0x4D0; // CUtlSymbolLarge
                pub const M_SZ_PLAYER_MODEL_TO_USE: usize = 0x4D8; // CUtlSymbolLarge
                pub const M_N_ARMOR_TO_SPAWN_WITH: usize = 0x4E0; // int32
                pub const M_N_DEFAULT_BEHAVIOR: usize = 0x4E4; // SpawnPointCoopEnemy::BotDefaultBehavior_t
                pub const M_N_BOT_DIFFICULTY: usize = 0x4E8; // int32
                pub const M_B_IS_AGRESSIVE: usize = 0x4EC; // bool
                pub const M_B_START_ASLEEP: usize = 0x4ED; // bool
                pub const M_FL_HIDE_RADIUS: usize = 0x4F0; // float32
                pub const M_SZ_BEHAVIOR_TREE_FILE: usize = 0x500; // CUtlSymbolLarge
            }
            // Parent: c_game_rules_proxy
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_pGameRules (CCSGameRules*)
            pub mod ccs_game_rules_proxy {
                pub const M_P_GAME_RULES: usize = 0x4C0; // CCSGameRules*
            }
            // Parent: c_teamplay_rules
            // Fields count: 215
            //
            // Metadata:
            // NetworkVarNames: m_bFreezePeriod (bool)
            // NetworkVarNames: m_bWarmupPeriod (bool)
            // NetworkVarNames: m_fWarmupPeriodEnd (GameTime_t)
            // NetworkVarNames: m_fWarmupPeriodStart (GameTime_t)
            // NetworkVarNames: m_nTotalPausedTicks (int)
            // NetworkVarNames: m_nPauseStartTick (int)
            // NetworkVarNames: m_bServerPaused (bool)
            // NetworkVarNames: m_bGamePaused (bool)
            // NetworkVarNames: m_bTerroristTimeOutActive (bool)
            // NetworkVarNames: m_bCTTimeOutActive (bool)
            // NetworkVarNames: m_flTerroristTimeOutRemaining (float)
            // NetworkVarNames: m_flCTTimeOutRemaining (float)
            // NetworkVarNames: m_nTerroristTimeOuts (int)
            // NetworkVarNames: m_nCTTimeOuts (int)
            // NetworkVarNames: m_bTechnicalTimeOut (bool)
            // NetworkVarNames: m_bMatchWaitingForResume (bool)
            // NetworkVarNames: m_iRoundTime (int)
            // NetworkVarNames: m_fMatchStartTime (float)
            // NetworkVarNames: m_fRoundStartTime (GameTime_t)
            // NetworkVarNames: m_flRestartRoundTime (GameTime_t)
            // NetworkVarNames: m_bGameRestart (bool)
            // NetworkVarNames: m_flGameStartTime (float)
            // NetworkVarNames: m_timeUntilNextPhaseStarts (float)
            // NetworkVarNames: m_gamePhase (int)
            // NetworkVarNames: m_totalRoundsPlayed (int)
            // NetworkVarNames: m_nRoundsPlayedThisPhase (int)
            // NetworkVarNames: m_nOvertimePlaying (int)
            // NetworkVarNames: m_iHostagesRemaining (int)
            // NetworkVarNames: m_bAnyHostageReached (bool)
            // NetworkVarNames: m_bMapHasBombTarget (bool)
            // NetworkVarNames: m_bMapHasRescueZone (bool)
            // NetworkVarNames: m_bMapHasBuyZone (bool)
            // NetworkVarNames: m_bIsQueuedMatchmaking (bool)
            // NetworkVarNames: m_nQueuedMatchmakingMode (int)
            // NetworkVarNames: m_bIsValveDS (bool)
            // NetworkVarNames: m_bLogoMap (bool)
            // NetworkVarNames: m_bPlayAllStepSoundsOnServer (bool)
            // NetworkVarNames: m_iSpectatorSlotCount (int)
            // NetworkVarNames: m_MatchDevice (int)
            // NetworkVarNames: m_bHasMatchStarted (bool)
            // NetworkVarNames: m_nNextMapInMapgroup (int)
            // NetworkVarNames: m_szTournamentEventName (char)
            // NetworkVarNames: m_szTournamentEventStage (char)
            // NetworkVarNames: m_szMatchStatTxt (char)
            // NetworkVarNames: m_szTournamentPredictionsTxt (char)
            // NetworkVarNames: m_nTournamentPredictionsPct (int)
            // NetworkVarNames: m_flCMMItemDropRevealStartTime (GameTime_t)
            // NetworkVarNames: m_flCMMItemDropRevealEndTime (GameTime_t)
            // NetworkVarNames: m_bIsDroppingItems (bool)
            // NetworkVarNames: m_bIsQuestEligible (bool)
            // NetworkVarNames: m_bIsHltvActive (bool)
            // NetworkVarNames: m_nGuardianModeWaveNumber (int)
            // NetworkVarNames: m_nGuardianModeSpecialKillsRemaining (int)
            // NetworkVarNames: m_nGuardianModeSpecialWeaponNeeded (int)
            // NetworkVarNames: m_numGlobalGiftsGiven (uint32)
            // NetworkVarNames: m_numGlobalGifters (uint32)
            // NetworkVarNames: m_numGlobalGiftsPeriodSeconds (uint32)
            // NetworkVarNames: m_arrFeaturedGiftersAccounts (uint32)
            // NetworkVarNames: m_arrFeaturedGiftersGifts (uint32)
            // NetworkVarNames: m_arrProhibitedItemIndices (uint16)
            // NetworkVarNames: m_arrTournamentActiveCasterAccounts (uint32)
            // NetworkVarNames: m_numBestOfMaps (int)
            // NetworkVarNames: m_nHalloweenMaskListSeed (int)
            // NetworkVarNames: m_bBombDropped (bool)
            // NetworkVarNames: m_bBombPlanted (bool)
            // NetworkVarNames: m_iRoundWinStatus (int)
            // NetworkVarNames: m_eRoundWinReason (int)
            // NetworkVarNames: m_bTCantBuy (bool)
            // NetworkVarNames: m_bCTCantBuy (bool)
            // NetworkVarNames: m_flGuardianBuyUntilTime (GameTime_t)
            // NetworkVarNames: m_iMatchStats_RoundResults (int)
            // NetworkVarNames: m_iMatchStats_PlayersAlive_CT (int)
            // NetworkVarNames: m_iMatchStats_PlayersAlive_T (int)
            // NetworkVarNames: m_TeamRespawnWaveTimes (float)
            // NetworkVarNames: m_flNextRespawnWave (GameTime_t)
            // NetworkVarNames: m_nServerQuestID (int)
            // NetworkVarNames: m_vMinimapMins (Vector)
            // NetworkVarNames: m_vMinimapMaxs (Vector)
            // NetworkVarNames: m_MinimapVerticalSectionHeights (float)
            // NetworkVarNames: m_nEndMatchMapGroupVoteTypes (int)
            // NetworkVarNames: m_nEndMatchMapGroupVoteOptions (int)
            // NetworkVarNames: m_nEndMatchMapVoteWinner (int)
            // NetworkVarNames: m_iNumConsecutiveCTLoses (int)
            // NetworkVarNames: m_iNumConsecutiveTerroristLoses (int)
            // NetworkVarNames: m_nMatchAbortedEarlyReason (int)
            // NetworkVarNames: m_pGameModeRules (CCSGameModeRules *)
            // NetworkVarNames: m_RetakeRules (CRetakeGameRules)
            // NetworkVarNames: m_nMatchEndCount (uint8)
            // NetworkVarNames: m_nTTeamIntroVariant (int)
            // NetworkVarNames: m_nCTTeamIntroVariant (int)
            // NetworkVarNames: m_bTeamIntroPeriod (bool)
            // NetworkVarNames: m_iRoundEndWinnerTeam (int)
            // NetworkVarNames: m_eRoundEndReason (int)
            // NetworkVarNames: m_bRoundEndShowTimerDefend (bool)
            // NetworkVarNames: m_iRoundEndTimerTime (int)
            // NetworkVarNames: m_sRoundEndFunFactToken (CUtlString)
            // NetworkVarNames: m_iRoundEndFunFactPlayerSlot (CPlayerSlot)
            // NetworkVarNames: m_iRoundEndFunFactData1 (int)
            // NetworkVarNames: m_iRoundEndFunFactData2 (int)
            // NetworkVarNames: m_iRoundEndFunFactData3 (int)
            // NetworkVarNames: m_sRoundEndMessage (CUtlString)
            // NetworkVarNames: m_iRoundEndPlayerCount (int)
            // NetworkVarNames: m_bRoundEndNoMusic (bool)
            // NetworkVarNames: m_iRoundEndLegacy (int)
            // NetworkVarNames: m_nRoundEndCount (uint8)
            // NetworkVarNames: m_iRoundStartRoundNumber (int)
            // NetworkVarNames: m_nRoundStartCount (uint8)
            pub mod ccs_game_rules {
                pub const M_P_CHAIN_ENTITY: usize = 0x98; // CNetworkVarChainer
                pub const M_COOP_MISSION_MANAGER: usize = 0xC0; // CHandle< CBaseEntity >
                pub const M_B_FREEZE_PERIOD: usize = 0xC4; // bool
                pub const M_B_WARMUP_PERIOD: usize = 0xC5; // bool
                pub const M_F_WARMUP_PERIOD_END: usize = 0xC8; // GameTime_t
                pub const M_F_WARMUP_PERIOD_START: usize = 0xCC; // GameTime_t
                pub const M_N_TOTAL_PAUSED_TICKS: usize = 0xD0; // int32
                pub const M_N_PAUSE_START_TICK: usize = 0xD4; // int32
                pub const M_B_SERVER_PAUSED: usize = 0xD8; // bool
                pub const M_B_GAME_PAUSED: usize = 0xD9; // bool
                pub const M_B_TERRORIST_TIME_OUT_ACTIVE: usize = 0xDA; // bool
                pub const M_B_CT_TIME_OUT_ACTIVE: usize = 0xDB; // bool
                pub const M_FL_TERRORIST_TIME_OUT_REMAINING: usize = 0xDC; // float32
                pub const M_FL_CT_TIME_OUT_REMAINING: usize = 0xE0; // float32
                pub const M_N_TERRORIST_TIME_OUTS: usize = 0xE4; // int32
                pub const M_N_CT_TIME_OUTS: usize = 0xE8; // int32
                pub const M_B_TECHNICAL_TIME_OUT: usize = 0xEC; // bool
                pub const M_B_MATCH_WAITING_FOR_RESUME: usize = 0xED; // bool
                pub const M_I_ROUND_TIME: usize = 0xF0; // int32
                pub const M_F_MATCH_START_TIME: usize = 0xF4; // float32
                pub const M_F_ROUND_START_TIME: usize = 0xF8; // GameTime_t
                pub const M_FL_RESTART_ROUND_TIME: usize = 0xFC; // GameTime_t
                pub const M_B_GAME_RESTART: usize = 0x100; // bool
                pub const M_FL_GAME_START_TIME: usize = 0x104; // float32
                pub const M_TIME_UNTIL_NEXT_PHASE_STARTS: usize = 0x108; // float32
                pub const M_GAME_PHASE: usize = 0x10C; // int32
                pub const M_TOTAL_ROUNDS_PLAYED: usize = 0x110; // int32
                pub const M_N_ROUNDS_PLAYED_THIS_PHASE: usize = 0x114; // int32
                pub const M_N_OVERTIME_PLAYING: usize = 0x118; // int32
                pub const M_I_HOSTAGES_REMAINING: usize = 0x11C; // int32
                pub const M_B_ANY_HOSTAGE_REACHED: usize = 0x120; // bool
                pub const M_B_MAP_HAS_BOMB_TARGET: usize = 0x121; // bool
                pub const M_B_MAP_HAS_RESCUE_ZONE: usize = 0x122; // bool
                pub const M_B_MAP_HAS_BUY_ZONE: usize = 0x123; // bool
                pub const M_B_IS_QUEUED_MATCHMAKING: usize = 0x124; // bool
                pub const M_N_QUEUED_MATCHMAKING_MODE: usize = 0x128; // int32
                pub const M_B_IS_VALVE_DS: usize = 0x12C; // bool
                pub const M_B_LOGO_MAP: usize = 0x12D; // bool
                pub const M_B_PLAY_ALL_STEP_SOUNDS_ON_SERVER: usize = 0x12E; // bool
                pub const M_I_SPECTATOR_SLOT_COUNT: usize = 0x130; // int32
                pub const M_MATCH_DEVICE: usize = 0x134; // int32
                pub const M_B_HAS_MATCH_STARTED: usize = 0x138; // bool
                pub const M_N_NEXT_MAP_IN_MAPGROUP: usize = 0x13C; // int32
                pub const M_SZ_TOURNAMENT_EVENT_NAME: usize = 0x140; // char[512]
                pub const M_SZ_TOURNAMENT_EVENT_STAGE: usize = 0x340; // char[512]
                pub const M_SZ_MATCH_STAT_TXT: usize = 0x540; // char[512]
                pub const M_SZ_TOURNAMENT_PREDICTIONS_TXT: usize = 0x740; // char[512]
                pub const M_N_TOURNAMENT_PREDICTIONS_PCT: usize = 0x940; // int32
                pub const M_FL_CMM_ITEM_DROP_REVEAL_START_TIME: usize = 0x944; // GameTime_t
                pub const M_FL_CMM_ITEM_DROP_REVEAL_END_TIME: usize = 0x948; // GameTime_t
                pub const M_B_IS_DROPPING_ITEMS: usize = 0x94C; // bool
                pub const M_B_IS_QUEST_ELIGIBLE: usize = 0x94D; // bool
                pub const M_B_IS_HLTV_ACTIVE: usize = 0x94E; // bool
                pub const M_N_GUARDIAN_MODE_WAVE_NUMBER: usize = 0x950; // int32
                pub const M_N_GUARDIAN_MODE_SPECIAL_KILLS_REMAINING: usize = 0x954; // int32
                pub const M_N_GUARDIAN_MODE_SPECIAL_WEAPON_NEEDED: usize = 0x958; // int32
                pub const M_N_GUARDIAN_GRENADES_TO_GIVE_BOTS: usize = 0x95C; // int32
                pub const M_N_NUM_HEAVIES_TO_SPAWN: usize = 0x960; // int32
                pub const M_NUM_GLOBAL_GIFTS_GIVEN: usize = 0x964; // uint32
                pub const M_NUM_GLOBAL_GIFTERS: usize = 0x968; // uint32
                pub const M_NUM_GLOBAL_GIFTS_PERIOD_SECONDS: usize = 0x96C; // uint32
                pub const M_ARR_FEATURED_GIFTERS_ACCOUNTS: usize = 0x970; // uint32[4]
                pub const M_ARR_FEATURED_GIFTERS_GIFTS: usize = 0x980; // uint32[4]
                pub const M_ARR_PROHIBITED_ITEM_INDICES: usize = 0x990; // uint16[100]
                pub const M_ARR_TOURNAMENT_ACTIVE_CASTER_ACCOUNTS: usize = 0xA58; // uint32[4]
                pub const M_NUM_BEST_OF_MAPS: usize = 0xA68; // int32
                pub const M_N_HALLOWEEN_MASK_LIST_SEED: usize = 0xA6C; // int32
                pub const M_B_BOMB_DROPPED: usize = 0xA70; // bool
                pub const M_B_BOMB_PLANTED: usize = 0xA71; // bool
                pub const M_I_ROUND_WIN_STATUS: usize = 0xA74; // int32
                pub const M_E_ROUND_WIN_REASON: usize = 0xA78; // int32
                pub const M_B_T_CANT_BUY: usize = 0xA7C; // bool
                pub const M_B_CT_CANT_BUY: usize = 0xA7D; // bool
                pub const M_FL_GUARDIAN_BUY_UNTIL_TIME: usize = 0xA80; // GameTime_t
                pub const M_I_MATCH_STATS_ROUND_RESULTS: usize = 0xA84; // int32[30]
                pub const M_I_MATCH_STATS_PLAYERS_ALIVE_CT: usize = 0xAFC; // int32[30]
                pub const M_I_MATCH_STATS_PLAYERS_ALIVE_T: usize = 0xB74; // int32[30]
                pub const M_TEAM_RESPAWN_WAVE_TIMES: usize = 0xBEC; // float32[32]
                pub const M_FL_NEXT_RESPAWN_WAVE: usize = 0xC6C; // GameTime_t[32]
                pub const M_N_SERVER_QUEST_ID: usize = 0xCEC; // int32
                pub const M_V_MINIMAP_MINS: usize = 0xCF0; // 
                pub const M_V_MINIMAP_MAXS: usize = 0xCFC; // 
                pub const M_MINIMAP_VERTICAL_SECTION_HEIGHTS: usize = 0xD08; // float32[8]
                pub const M_B_DONT_INCREMENT_COOP_WAVE: usize = 0xD28; // bool
                pub const M_B_SPAWNED_TERROR_HUNT_HEAVY: usize = 0xD29; // bool
                pub const M_N_END_MATCH_MAP_GROUP_VOTE_TYPES: usize = 0xD2C; // int32[10]
                pub const M_N_END_MATCH_MAP_GROUP_VOTE_OPTIONS: usize = 0xD54; // int32[10]
                pub const M_N_END_MATCH_MAP_VOTE_WINNER: usize = 0xD7C; // int32
                pub const M_I_NUM_CONSECUTIVE_CT_LOSES: usize = 0xD80; // int32
                pub const M_I_NUM_CONSECUTIVE_TERRORIST_LOSES: usize = 0xD84; // int32
                pub const M_B_HAS_HOSTAGE_BEEN_TOUCHED: usize = 0xDA0; // bool
                pub const M_FL_INTERMISSION_START_TIME: usize = 0xDA4; // GameTime_t
                pub const M_FL_INTERMISSION_END_TIME: usize = 0xDA8; // GameTime_t
                pub const M_B_LEVEL_INITIALIZED: usize = 0xDAC; // bool
                pub const M_I_TOTAL_ROUNDS_PLAYED: usize = 0xDB0; // int32
                pub const M_I_UN_BALANCED_ROUNDS: usize = 0xDB4; // int32
                pub const M_END_MATCH_ON_ROUND_RESET: usize = 0xDB8; // bool
                pub const M_END_MATCH_ON_THINK: usize = 0xDB9; // bool
                pub const M_I_FREEZE_TIME: usize = 0xDBC; // int32
                pub const M_I_NUM_TERRORIST: usize = 0xDC0; // int32
                pub const M_I_NUM_CT: usize = 0xDC4; // int32
                pub const M_I_NUM_SPAWNABLE_TERRORIST: usize = 0xDC8; // int32
                pub const M_I_NUM_SPAWNABLE_CT: usize = 0xDCC; // int32
                pub const M_ARR_SELECTED_HOSTAGE_SPAWN_INDICES: usize = 0xDD0; // CUtlVector< int32 >
                pub const M_N_SPAWN_POINTS_RANDOM_SEED: usize = 0xDE8; // int32
                pub const M_B_FIRST_CONNECTED: usize = 0xDEC; // bool
                pub const M_B_COMPLETE_RESET: usize = 0xDED; // bool
                pub const M_B_PICK_NEW_TEAMS_ON_RESET: usize = 0xDEE; // bool
                pub const M_B_SCRAMBLE_TEAMS_ON_RESTART: usize = 0xDEF; // bool
                pub const M_B_SWAP_TEAMS_ON_RESTART: usize = 0xDF0; // bool
                pub const M_N_END_MATCH_TIED_VOTES: usize = 0xDF8; // CUtlVector< int32 >
                pub const M_B_NEED_TO_ASK_PLAYERS_FOR_CONTINUE_VOTE: usize = 0xE14; // bool
                pub const M_NUM_QUEUED_MATCHMAKING_ACCOUNTS: usize = 0xE18; // uint32
                pub const M_P_QUEUED_MATCHMAKING_RESERVATION_STRING: usize = 0xE20; // char*
                pub const M_NUM_TOTAL_TOURNAMENT_DROPS: usize = 0xE28; // uint32
                pub const M_NUM_SPECTATORS_COUNT_MAX: usize = 0xE2C; // uint32
                pub const M_NUM_SPECTATORS_COUNT_MAX_TV: usize = 0xE30; // uint32
                pub const M_NUM_SPECTATORS_COUNT_MAX_LNK: usize = 0xE34; // uint32
                pub const M_B_FORCE_TEAM_CHANGE_SILENT: usize = 0xE40; // bool
                pub const M_B_LOADING_ROUND_BACKUP_DATA: usize = 0xE41; // bool
                pub const M_N_MATCH_INFO_SHOW_TYPE: usize = 0xE78; // int32
                pub const M_FL_MATCH_INFO_DECIDED_TIME: usize = 0xE7C; // float32
                pub const M_FL_COOP_RESPAWN_AND_HEAL_TIME: usize = 0xE98; // float32
                pub const M_COOP_BONUS_COINS_FOUND: usize = 0xE9C; // int32
                pub const M_COOP_BONUS_PISTOLS_ONLY: usize = 0xEA0; // bool
                pub const M_COOP_PLAYERS_IN_DEPLOYMENT_ZONE: usize = 0xEA1; // bool
                pub const M_COOP_MISSION_DEAD_PLAYER_RESPAWN_ENABLED: usize = 0xEA2; // bool
                pub const M_TEAM_DM_LAST_WINNING_TEAM_NUMBER: usize = 0xEA4; // int32
                pub const M_TEAM_DM_LAST_THINK_TIME: usize = 0xEA8; // float32
                pub const M_FL_TEAM_DM_LAST_ANNOUNCEMENT_TIME: usize = 0xEAC; // float32
                pub const M_I_ACCOUNT_TERRORIST: usize = 0xEB0; // int32
                pub const M_I_ACCOUNT_CT: usize = 0xEB4; // int32
                pub const M_I_SPAWN_POINT_COUNT_TERRORIST: usize = 0xEB8; // int32
                pub const M_I_SPAWN_POINT_COUNT_CT: usize = 0xEBC; // int32
                pub const M_I_MAX_NUM_TERRORISTS: usize = 0xEC0; // int32
                pub const M_I_MAX_NUM_C_TS: usize = 0xEC4; // int32
                pub const M_I_LOSER_BONUS: usize = 0xEC8; // int32
                pub const M_I_LOSER_BONUS_MOST_RECENT_TEAM: usize = 0xECC; // int32
                pub const M_TM_NEXT_PERIODIC_THINK: usize = 0xED0; // float32
                pub const M_B_VOICE_WON_MATCH_BRAG_FIRED: usize = 0xED4; // bool
                pub const M_F_WARMUP_NEXT_CHAT_NOTICE_TIME: usize = 0xED8; // float32
                pub const M_I_HOSTAGES_RESCUED: usize = 0xEE0; // int32
                pub const M_I_HOSTAGES_TOUCHED: usize = 0xEE4; // int32
                pub const M_FL_NEXT_HOSTAGE_ANNOUNCEMENT: usize = 0xEE8; // float32
                pub const M_B_NO_TERRORISTS_KILLED: usize = 0xEEC; // bool
                pub const M_B_NO_C_TS_KILLED: usize = 0xEED; // bool
                pub const M_B_NO_ENEMIES_KILLED: usize = 0xEEE; // bool
                pub const M_B_CAN_DONATE_WEAPONS: usize = 0xEEF; // bool
                pub const M_FIRST_KILL_TIME: usize = 0xEF4; // float32
                pub const M_FIRST_BLOOD_TIME: usize = 0xEFC; // float32
                pub const M_HOSTAGE_WAS_INJURED: usize = 0xF18; // bool
                pub const M_HOSTAGE_WAS_KILLED: usize = 0xF19; // bool
                pub const M_B_VOTE_CALLED: usize = 0xF28; // bool
                pub const M_B_SERVER_VOTE_ON_RESET: usize = 0xF29; // bool
                pub const M_FL_VOTE_CHECK_THROTTLE: usize = 0xF2C; // float32
                pub const M_B_BUY_TIME_ENDED: usize = 0xF30; // bool
                pub const M_N_LAST_FREEZE_END_BEEP: usize = 0xF34; // int32
                pub const M_B_TARGET_BOMBED: usize = 0xF38; // bool
                pub const M_B_BOMB_DEFUSED: usize = 0xF39; // bool
                pub const M_B_MAP_HAS_BOMB_ZONE: usize = 0xF3A; // bool
                pub const M_VEC_MAIN_CT_SPAWN_POS: usize = 0xF58; // 
                pub const M_CT_SPAWN_POINTS_MASTER_LIST: usize = 0xF68; // CUtlVector< SpawnPoint* >
                pub const M_TERRORIST_SPAWN_POINTS_MASTER_LIST: usize = 0xF80; // CUtlVector< SpawnPoint* >
                pub const M_B_RESPAWNING_ALL_RESPAWNABLE_PLAYERS: usize = 0xF98; // bool
                pub const M_I_NEXT_CT_SPAWN_POINT: usize = 0xF9C; // int32
                pub const M_FL_CT_SPAWN_POINT_USED_TIME: usize = 0xFA0; // float32
                pub const M_I_NEXT_TERRORIST_SPAWN_POINT: usize = 0xFA4; // int32
                pub const M_FL_TERRORIST_SPAWN_POINT_USED_TIME: usize = 0xFA8; // float32
                pub const M_CT_SPAWN_POINTS: usize = 0xFB0; // CUtlVector< SpawnPoint* >
                pub const M_TERRORIST_SPAWN_POINTS: usize = 0xFC8; // CUtlVector< SpawnPoint* >
                pub const M_B_IS_UNRESERVED_GAME_SERVER: usize = 0xFE0; // bool
                pub const M_F_AUTOBALANCE_DISPLAY_TIME: usize = 0xFE4; // float32
                pub const M_B_ALLOW_WEAPON_SWITCH: usize = 0x1250; // bool
                pub const M_B_ROUND_TIME_WARNING_TRIGGERED: usize = 0x1251; // bool
                pub const M_PHASE_CHANGE_ANNOUNCEMENT_TIME: usize = 0x1254; // GameTime_t
                pub const M_F_NEXT_UPDATE_TEAM_CLAN_NAMES_TIME: usize = 0x1258; // float32
                pub const M_FL_LAST_THINK_TIME: usize = 0x125C; // GameTime_t
                pub const M_F_ACCUMULATED_ROUND_OFF_DAMAGE: usize = 0x1260; // float32
                pub const M_N_SHORTHANDED_BONUS_LAST_EVAL_ROUND: usize = 0x1264; // int32
                pub const M_N_MATCH_ABORTED_EARLY_REASON: usize = 0x14E0; // int32
                pub const M_B_HAS_TRIGGERED_ROUND_START_MUSIC: usize = 0x14E4; // bool
                pub const M_B_HAS_TRIGGERED_COOP_SPAWN_RESET: usize = 0x14E5; // bool
                pub const M_B_SWITCHING_TEAMS_AT_ROUND_RESET: usize = 0x14E6; // bool
                pub const M_P_GAME_MODE_RULES: usize = 0x1500; // CCSGameModeRules*
                pub const M_BT_GLOBAL_BLACKBOARD: usize = 0x1508; // KeyValues3
                pub const M_H_PLAYER_RESOURCE: usize = 0x1570; // CHandle< CBaseEntity >
                pub const M_RETAKE_RULES: usize = 0x1578; // CRetakeGameRules
                pub const M_GUARDIAN_BOT_SKILL_LEVEL_MAX: usize = 0x175C; // int32
                pub const M_GUARDIAN_BOT_SKILL_LEVEL_MIN: usize = 0x1760; // int32
                pub const M_ARR_TEAM_UNIQUE_KILL_WEAPONS_MATCH: usize = 0x1768; // CUtlVector< int32 >[4]
                pub const M_B_TEAM_LAST_KILL_USED_UNIQUE_WEAPON_MATCH: usize = 0x17C8; // bool[4]
                pub const M_N_MATCH_END_COUNT: usize = 0x17F0; // uint8
                pub const M_N_T_TEAM_INTRO_VARIANT: usize = 0x17F4; // int32
                pub const M_N_CT_TEAM_INTRO_VARIANT: usize = 0x17F8; // int32
                pub const M_B_TEAM_INTRO_PERIOD: usize = 0x17FC; // bool
                pub const M_F_TEAM_INTRO_PERIOD_END: usize = 0x1800; // GameTime_t
                pub const M_B_PLAYED_TEAM_INTRO_VO: usize = 0x1804; // bool
                pub const M_I_ROUND_END_WINNER_TEAM: usize = 0x1808; // int32
                pub const M_E_ROUND_END_REASON: usize = 0x180C; // int32
                pub const M_B_ROUND_END_SHOW_TIMER_DEFEND: usize = 0x1810; // bool
                pub const M_I_ROUND_END_TIMER_TIME: usize = 0x1814; // int32
                pub const M_S_ROUND_END_FUN_FACT_TOKEN: usize = 0x1818; // CUtlString
                pub const M_I_ROUND_END_FUN_FACT_PLAYER_SLOT: usize = 0x1820; // CPlayerSlot
                pub const M_I_ROUND_END_FUN_FACT_DATA1: usize = 0x1824; // int32
                pub const M_I_ROUND_END_FUN_FACT_DATA2: usize = 0x1828; // int32
                pub const M_I_ROUND_END_FUN_FACT_DATA3: usize = 0x182C; // int32
                pub const M_S_ROUND_END_MESSAGE: usize = 0x1830; // CUtlString
                pub const M_I_ROUND_END_PLAYER_COUNT: usize = 0x1838; // int32
                pub const M_B_ROUND_END_NO_MUSIC: usize = 0x183C; // bool
                pub const M_I_ROUND_END_LEGACY: usize = 0x1840; // int32
                pub const M_N_ROUND_END_COUNT: usize = 0x1844; // uint8
                pub const M_I_ROUND_START_ROUND_NUMBER: usize = 0x1848; // int32
                pub const M_N_ROUND_START_COUNT: usize = 0x184C; // uint8
                pub const M_FL_LAST_PERF_SAMPLE_TIME: usize = 0x5858; // float64
                pub const M_B_SKIP_NEXT_SERVER_PERF_SAMPLE: usize = 0x5860; // bool
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod ccs_take_damage_info_api {
            }
            // Parent: None
            // Fields count: 1
            pub mod ccs_game_mode_rules {
                pub const M_P_CHAIN_ENTITY: usize = 0x8; // CNetworkVarChainer
            }
            // Parent: ccs_game_mode_rules
            // Fields count: 0
            pub mod ccs_game_mode_rules_noop {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod ccs_game_mode_script {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            pub mod ccs_game_mode_script_con_vars {
            }
            // Parent: ccs_game_mode_rules
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_WeaponSequence (CUtlString)
            pub mod ccs_game_mode_rules_arms_race {
                pub const M_WEAPON_SEQUENCE: usize = 0x38; // CNetworkUtlVectorBase< CUtlString >
            }
            // Parent: ccs_game_mode_script
            // Fields count: 1
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod ccs_arms_race_script {
                pub const M_P_OUTER: usize = 0xD8; // CCSGameModeRules_ArmsRace*
            }
            // Parent: ccs_game_mode_rules
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_flDMBonusStartTime (GameTime_t)
            // NetworkVarNames: m_flDMBonusTimeLength (float)
            // NetworkVarNames: m_nDMBonusWeaponLoadoutSlot (int16)
            pub mod ccs_game_mode_rules_deathmatch {
                pub const M_FL_DM_BONUS_START_TIME: usize = 0x38; // GameTime_t
                pub const M_FL_DM_BONUS_TIME_LENGTH: usize = 0x3C; // float32
                pub const M_N_DM_BONUS_WEAPON_LOADOUT_SLOT: usize = 0x40; // int16
            }
            // Parent: ccs_game_mode_script
            // Fields count: 1
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod ccs_deathmatch_script {
                pub const M_P_OUTER: usize = 0xD8; // CCSGameModeRules_Deathmatch*
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            pub mod ccs_deathmatch_script_con_vars {
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_nMatchSeed (int)
            // NetworkVarNames: m_bBlockersPresent (bool)
            // NetworkVarNames: m_bRoundInProgress (bool)
            // NetworkVarNames: m_iFirstSecondHalfRound (int)
            // NetworkVarNames: m_iBombSite (int)
            pub mod c_retake_game_rules {
                pub const M_N_MATCH_SEED: usize = 0xF8; // int32
                pub const M_B_BLOCKERS_PRESENT: usize = 0xFC; // bool
                pub const M_B_ROUND_IN_PROGRESS: usize = 0xFD; // bool
                pub const M_I_FIRST_SECOND_HALF_ROUND: usize = 0x100; // int32
                pub const M_I_BOMB_SITE: usize = 0x104; // int32
            }
            // Parent: None
            // Fields count: 0
            pub mod quest_progress {
            }
            // Parent: None
            // Fields count: 13
            //
            // Metadata:
            // NetworkVarNames: m_iKills (int)
            // NetworkVarNames: m_iDeaths (int)
            // NetworkVarNames: m_iAssists (int)
            // NetworkVarNames: m_iDamage (int)
            // NetworkVarNames: m_iEquipmentValue (int)
            // NetworkVarNames: m_iMoneySaved (int)
            // NetworkVarNames: m_iKillReward (int)
            // NetworkVarNames: m_iLiveTime (int)
            // NetworkVarNames: m_iHeadShotKills (int)
            // NetworkVarNames: m_iObjective (int)
            // NetworkVarNames: m_iCashEarned (int)
            // NetworkVarNames: m_iUtilityDamage (int)
            // NetworkVarNames: m_iEnemiesFlashed (int)
            pub mod cs_per_round_stats_t {
                pub const M_I_KILLS: usize = 0x30; // int32
                pub const M_I_DEATHS: usize = 0x34; // int32
                pub const M_I_ASSISTS: usize = 0x38; // int32
                pub const M_I_DAMAGE: usize = 0x3C; // int32
                pub const M_I_EQUIPMENT_VALUE: usize = 0x40; // int32
                pub const M_I_MONEY_SAVED: usize = 0x44; // int32
                pub const M_I_KILL_REWARD: usize = 0x48; // int32
                pub const M_I_LIVE_TIME: usize = 0x4C; // int32
                pub const M_I_HEAD_SHOT_KILLS: usize = 0x50; // int32
                pub const M_I_OBJECTIVE: usize = 0x54; // int32
                pub const M_I_CASH_EARNED: usize = 0x58; // int32
                pub const M_I_UTILITY_DAMAGE: usize = 0x5C; // int32
                pub const M_I_ENEMIES_FLASHED: usize = 0x60; // int32
            }
            // Parent: cs_per_round_stats_t
            // Fields count: 21
            //
            // Metadata:
            // NetworkVarNames: m_iEnemy5Ks (int)
            // NetworkVarNames: m_iEnemy4Ks (int)
            // NetworkVarNames: m_iEnemy3Ks (int)
            // NetworkVarNames: m_iEnemyKnifeKills (int)
            // NetworkVarNames: m_iEnemyTaserKills (int)
            pub mod cs_match_stats_t {
                pub const M_I_ENEMY5_KS: usize = 0x68; // int32
                pub const M_I_ENEMY4_KS: usize = 0x6C; // int32
                pub const M_I_ENEMY3_KS: usize = 0x70; // int32
                pub const M_I_ENEMY_KNIFE_KILLS: usize = 0x74; // int32
                pub const M_I_ENEMY_TASER_KILLS: usize = 0x78; // int32
                pub const M_I_ENEMY2_KS: usize = 0x7C; // int32
                pub const M_I_UTILITY_COUNT: usize = 0x80; // int32
                pub const M_I_UTILITY_SUCCESSES: usize = 0x84; // int32
                pub const M_I_UTILITY_ENEMIES: usize = 0x88; // int32
                pub const M_I_FLASH_COUNT: usize = 0x8C; // int32
                pub const M_I_FLASH_SUCCESSES: usize = 0x90; // int32
                pub const M_N_HEALTH_POINTS_REMOVED_TOTAL: usize = 0x94; // int32
                pub const M_N_HEALTH_POINTS_DEALT_TOTAL: usize = 0x98; // int32
                pub const M_N_SHOTS_FIRED_TOTAL: usize = 0x9C; // int32
                pub const M_N_SHOTS_ON_TARGET_TOTAL: usize = 0xA0; // int32
                pub const M_I1V1_COUNT: usize = 0xA4; // int32
                pub const M_I1V1_WINS: usize = 0xA8; // int32
                pub const M_I1V2_COUNT: usize = 0xAC; // int32
                pub const M_I1V2_WINS: usize = 0xB0; // int32
                pub const M_I_ENTRY_COUNT: usize = 0xB4; // int32
                pub const M_I_ENTRY_WINS: usize = 0xB8; // int32
            }
            // Parent: c_base_entity
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_nVariant (int)
            // NetworkVarNames: m_nRandom (int)
            // NetworkVarNames: m_nOrdinal (int)
            // NetworkVarNames: m_sWeaponName (CUtlString)
            // NetworkVarNames: m_xuid (XUID)
            // NetworkVarNames: m_agentItem (CEconItemView)
            // NetworkVarNames: m_glovesItem (CEconItemView)
            // NetworkVarNames: m_weaponItem (CEconItemView)
            pub mod ccsgo_team_preview_character_position {
                pub const M_N_VARIANT: usize = 0x4C0; // int32
                pub const M_N_RANDOM: usize = 0x4C4; // int32
                pub const M_N_ORDINAL: usize = 0x4C8; // int32
                pub const M_S_WEAPON_NAME: usize = 0x4D0; // CUtlString
                pub const M_XUID: usize = 0x4D8; // uint64
                pub const M_AGENT_ITEM: usize = 0x4E0; // CEconItemView
                pub const M_GLOVES_ITEM: usize = 0x758; // CEconItemView
                pub const M_WEAPON_ITEM: usize = 0x9D0; // CEconItemView
            }
            // Parent: ccsgo_team_preview_character_position
            // Fields count: 0
            pub mod ccsgo_team_select_character_position {
            }
            // Parent: ccsgo_team_select_character_position
            // Fields count: 0
            pub mod ccsgo_team_select_terrorist_position {
            }
            // Parent: ccsgo_team_select_character_position
            // Fields count: 0
            pub mod ccsgo_team_select_counter_terrorist_position {
            }
            // Parent: ccsgo_team_preview_character_position
            // Fields count: 0
            pub mod ccsgo_team_intro_character_position {
            }
            // Parent: ccsgo_team_intro_character_position
            // Fields count: 0
            pub mod ccsgo_team_intro_terrorist_position {
            }
            // Parent: ccsgo_team_intro_character_position
            // Fields count: 0
            pub mod ccsgo_team_intro_counter_terrorist_position {
            }
            // Parent: ccsgo_team_intro_character_position
            // Fields count: 0
            pub mod ccsgo_wingman_intro_character_position {
            }
            // Parent: ccsgo_wingman_intro_character_position
            // Fields count: 0
            pub mod ccsgo_wingman_intro_terrorist_position {
            }
            // Parent: ccsgo_wingman_intro_character_position
            // Fields count: 0
            pub mod ccsgo_wingman_intro_counter_terrorist_position {
            }
            // Parent: c_base_entity
            // Fields count: 0
            pub mod ccs_minimap_boundary {
            }
            // Parent: None
            // Fields count: 0
            pub mod ccsgo_player_anim_graph_state {
            }
            // Parent: c_base_entity
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_hPlayer (CHandle< CCSPlayerPawn>)
            // NetworkVarNames: m_hPingedEntity (CHandle< CBaseEntity>)
            // NetworkVarNames: m_iType (int)
            // NetworkVarNames: m_bUrgent (bool)
            // NetworkVarNames: m_szPlaceName (char)
            pub mod c_player_ping {
                pub const M_H_PLAYER: usize = 0x4C8; // CHandle< CCSPlayerPawn >
                pub const M_H_PINGED_ENTITY: usize = 0x4CC; // CHandle< CBaseEntity >
                pub const M_I_TYPE: usize = 0x4D0; // int32
                pub const M_B_URGENT: usize = 0x4D4; // bool
                pub const M_SZ_PLACE_NAME: usize = 0x4D5; // char[18]
            }
            // Parent: c_player_pawn_component
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hPlayerPing (CHandle< CBaseEntity>)
            pub mod ccs_player_ping_services {
                pub const M_FL_PLAYER_PING_TOKENS: usize = 0x40; // GameTime_t[5]
                pub const M_H_PLAYER_PING: usize = 0x54; // CHandle< CBaseEntity >
            }
            // Parent: c_base_entity
            // Fields count: 10
            //
            // Metadata:
            // NetworkVarNames: m_bHostageAlive (bool)
            // NetworkVarNames: m_isHostageFollowingSomeone (bool)
            // NetworkVarNames: m_iHostageEntityIDs (CEntityIndex)
            // NetworkVarNames: m_bombsiteCenterA (Vector)
            // NetworkVarNames: m_bombsiteCenterB (Vector)
            // NetworkVarNames: m_hostageRescueX (int)
            // NetworkVarNames: m_hostageRescueY (int)
            // NetworkVarNames: m_hostageRescueZ (int)
            // NetworkVarNames: m_bEndMatchNextMapAllVoted (bool)
            pub mod ccs_player_resource {
                pub const M_B_HOSTAGE_ALIVE: usize = 0x4C0; // bool[12]
                pub const M_IS_HOSTAGE_FOLLOWING_SOMEONE: usize = 0x4CC; // bool[12]
                pub const M_I_HOSTAGE_ENTITY_I_DS: usize = 0x4D8; // CEntityIndex[12]
                pub const M_BOMBSITE_CENTER_A: usize = 0x508; // 
                pub const M_BOMBSITE_CENTER_B: usize = 0x514; // 
                pub const M_HOSTAGE_RESCUE_X: usize = 0x520; // int32[4]
                pub const M_HOSTAGE_RESCUE_Y: usize = 0x530; // int32[4]
                pub const M_HOSTAGE_RESCUE_Z: usize = 0x540; // int32[4]
                pub const M_B_END_MATCH_NEXT_MAP_ALL_VOTED: usize = 0x550; // bool
                pub const M_FOUND_GOAL_POSITIONS: usize = 0x551; // bool
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod ccs_player_controller_api {
            }
            // Parent: c_player_pawn_component
            // Fields count: 0
            pub mod c_player_view_model_services {
            }
        }
        // Module: soundsystem.dll
        // Classes count: 50
        // Enums count: 13
        pub mod soundsystem {
            // Alignment: 4
            // Members count: 30
            #[repr(i32)]
            pub enum SoundlevelT {
                SndlvlNone = 0,
                Sndlvl20dB = 20,
                Sndlvl25dB = 25,
                Sndlvl30dB = 30,
                Sndlvl35dB = 35,
                Sndlvl40dB = 40,
                Sndlvl45dB = 45,
                Sndlvl50dB = 50,
                Sndlvl55dB = 55,
                SndlvlIdle = 60,
                Sndlvl60dB = 60,
                Sndlvl65dB = 65,
                SndlvlStatic = 66,
                Sndlvl70dB = 70,
                SndlvlNorm = 75,
                Sndlvl75dB = 75,
                Sndlvl80dB = 80,
                SndlvlTalking = 80,
                Sndlvl85dB = 85,
                Sndlvl90dB = 90,
                Sndlvl95dB = 95,
                Sndlvl100dB = 100,
                Sndlvl105dB = 105,
                Sndlvl110dB = 110,
                Sndlvl120dB = 120,
                Sndlvl130dB = 130,
                SndlvlGunfire = 140,
                Sndlvl140dB = 140,
                Sndlvl150dB = 150,
                Sndlvl180dB = 180
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum ActionTypeT {
                SosActionNone = 0,
                SosActionLimiter = 1,
                SosActionTimeLimit = 2,
                SosActionTimeBlockLimiter = 3,
                SosActionSetSoundeventParam = 4,
                SosActionSoundeventCluster = 5
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum SosActionStopTypeT {
                SosStoptypeNone = 0,
                SosStoptypeTime = 1,
                SosStoptypeOpvar = 2
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum SosActionSortTypeT {
                SosSorttypeHighest = 0,
                SosSorttypeLowest = 1
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum SosGroupTypeT {
                SosGrouptypeDynamic = 0,
                SosGrouptypeStatic = 1
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum SosEditItemTypeT {
                SosEditItemTypeSoundevents = 0,
                SosEditItemTypeSoundevent = 1,
                SosEditItemTypeLibrarystacks = 2,
                SosEditItemTypeStack = 3,
                SosEditItemTypeOperator = 4,
                SosEditItemTypeField = 5
            }
            // Alignment: 2
            // Members count: 10
            #[repr(i16)]
            pub enum VMixFilterTypeT {
                FilterUnknown = -1,
                FilterLowpass = 0,
                FilterHighpass = 1,
                FilterBandpass = 2,
                FilterNotch = 3,
                FilterPeakingEq = 4,
                FilterLowShelf = 5,
                FilterHighShelf = 6,
                FilterAllpass = 7,
                FilterPassthrough = 8
            }
            // Alignment: 1
            // Members count: 9
            #[repr(i8)]
            pub enum VMixFilterSlopeT {
                FilterSlope1pole6dB = 0,
                FilterSlope1pole12dB = 1,
                FilterSlope1pole18dB = 2,
                FilterSlope1pole24dB = 3,
                FilterSlope12dB = 4,
                FilterSlope24dB = 5,
                FilterSlope36dB = 6,
                FilterSlope48dB = 7,
                FilterSlopeMax = 7
            }
            // Alignment: 2
            // Members count: 30
            #[repr(i16)]
            pub enum VMixProcessorTypeT {
                VprocessorUnknown = 0,
                VprocessorSteamaudioReverb = 1,
                VprocessorRtPitch = 2,
                VprocessorSteamaudioHrtf = 3,
                VprocessorDynamics = 4,
                VprocessorPresetdsp = 5,
                VprocessorDelay = 6,
                VprocessorModDelay = 7,
                VprocessorDiffusor = 8,
                VprocessorBoxverb = 9,
                VprocessorFreeverb = 10,
                VprocessorPlateverb = 11,
                VprocessorFullwaveIntegrator = 12,
                VprocessorFilter = 13,
                VprocessorSteamaudioPathing = 14,
                VprocessorEq8 = 15,
                VprocessorEnvelope = 16,
                VprocessorVocoder = 17,
                VprocessorConvolution = 18,
                VprocessorDynamics3band = 19,
                VprocessorDynamicsCompressor = 20,
                VprocessorShaper = 21,
                VprocessorPanner = 22,
                VprocessorUtility = 23,
                VprocessorAutofilter = 24,
                VprocessorOsc = 25,
                VprocessorStereodelay = 26,
                VprocessorEffectChain = 27,
                VprocessorSubgraphSwitch = 28,
                VprocessorSteamaudioDirect = 29
            }
            // Alignment: 4
            // Members count: 5
            #[repr(i32)]
            pub enum VMixLfoShapeT {
                LfoShapeSine = 0,
                LfoShapeSquare = 1,
                LfoShapeTri = 2,
                LfoShapeSaw = 3,
                LfoShapeNoise = 4
            }
            // Alignment: 4
            // Members count: 2
            #[repr(i32)]
            pub enum VMixPannerTypeT {
                PannerTypeLinear = 0,
                PannerTypeEqualPower = 1
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum VMixSubgraphSwitchInterpolationTypeT {
                SubgraphInterpolationTemporalCrossfade = 0,
                SubgraphInterpolationTemporalFadeOut = 1,
                SubgraphInterpolationKeepLastSubgraphRunning = 2
            }
            // Alignment: 4
            // Members count: 6
            #[repr(i32)]
            pub enum VMixChannelOperationT {
                VmixChanStereo = 0,
                VmixChanLeft = 1,
                VmixChanRight = 2,
                VmixChanSwap = 3,
                VmixChanMono = 4,
                VmixChanMidSide = 5
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyElementNameFn
            pub mod c_sos_group_action_schema {
                pub const M_NAME: usize = 0x8; // CUtlString
                pub const M_ACTION_TYPE: usize = 0x10; // ActionType_t
                pub const M_ACTION_INSTANCE_TYPE: usize = 0x14; // ActionType_t
            }
            // Parent: c_sos_group_action_schema
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_sos_group_action_limit_schema {
                pub const M_N_MAX_COUNT: usize = 0x18; // int32
                pub const M_N_STOP_TYPE: usize = 0x1C; // SosActionStopType_t
                pub const M_N_SORT_TYPE: usize = 0x20; // SosActionSortType_t
            }
            // Parent: c_sos_group_action_schema
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_sos_group_action_time_limit_schema {
                pub const M_FL_MAX_DURATION: usize = 0x18; // float32
            }
            // Parent: c_sos_group_action_schema
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_sos_group_action_time_block_limit_schema {
                pub const M_N_MAX_COUNT: usize = 0x18; // int32
                pub const M_FL_MAX_DURATION: usize = 0x1C; // float32
            }
            // Parent: c_sos_group_action_schema
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_sos_group_action_set_soundevent_parameter_schema {
                pub const M_N_MAX_COUNT: usize = 0x18; // int32
                pub const M_FL_MIN_VALUE: usize = 0x1C; // float32
                pub const M_FL_MAX_VALUE: usize = 0x20; // float32
                pub const M_OPVAR_NAME: usize = 0x28; // CUtlString
                pub const M_N_SORT_TYPE: usize = 0x30; // SosActionSortType_t
            }
            // Parent: c_sos_group_action_schema
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_sos_group_action_soundevent_cluster_schema {
                pub const M_N_MIN_NEARBY: usize = 0x18; // int32
                pub const M_FL_CLUSTER_EPSILON: usize = 0x1C; // float32
                pub const M_SHOULD_PLAY_OPVAR: usize = 0x20; // CUtlString
                pub const M_SHOULD_PLAY_CLUSTER_CHILD: usize = 0x28; // CUtlString
                pub const M_CLUSTER_SIZE_OPVAR: usize = 0x30; // CUtlString
                pub const M_GROUP_BOUNDING_BOX_MINS_OPVAR: usize = 0x38; // CUtlString
                pub const M_GROUP_BOUNDING_BOX_MAXS_OPVAR: usize = 0x40; // CUtlString
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_sos_group_branch_pattern {
                pub const M_B_MATCH_EVENT_NAME: usize = 0x8; // bool
                pub const M_B_MATCH_EVENT_SUB_STRING: usize = 0x9; // bool
                pub const M_B_MATCH_ENT_INDEX: usize = 0xA; // bool
                pub const M_B_MATCH_OPVAR: usize = 0xB; // bool
            }
            // Parent: c_sos_group_branch_pattern
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_sos_group_match_pattern {
                pub const M_MATCH_SOUND_EVENT_NAME: usize = 0x10; // CUtlString
                pub const M_MATCH_SOUND_EVENT_SUB_STRING: usize = 0x18; // CUtlString
                pub const M_FL_ENT_INDEX: usize = 0x20; // float32
                pub const M_FL_OPVAR: usize = 0x24; // float32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyElementNameFn
            pub mod c_sos_sound_event_group_schema {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_N_TYPE: usize = 0x8; // SosGroupType_t
                pub const M_B_IS_BLOCKING: usize = 0xC; // bool
                pub const M_N_BLOCK_MAX_COUNT: usize = 0x10; // int32
                pub const M_B_INVERT_MATCH: usize = 0x14; // bool
                pub const M_MATCH_PATTERN: usize = 0x18; // CSosGroupMatchPattern
                pub const M_BRANCH_PATTERN: usize = 0x40; // CSosGroupBranchPattern
                pub const M_V_ACTIONS: usize = 0xB0; // CSosGroupActionSchema*[4]
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_sos_sound_event_group_list_schema {
                pub const M_GROUP_LIST: usize = 0x0; // CUtlVector< CSosSoundEventGroupSchema >
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod sos_edit_item_info_t {
                pub const ITEM_TYPE: usize = 0x0; // SosEditItemType_t
                pub const ITEM_NAME: usize = 0x8; // CUtlString
                pub const ITEM_TYPE_NAME: usize = 0x10; // CUtlString
                pub const ITEM_KV_STRING: usize = 0x20; // CUtlString
                pub const ITEM_POS: usize = 0x28; // Vector2D
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod selected_edit_item_info_t {
                pub const M_EDIT_ITEMS: usize = 0x0; // CUtlVector< SosEditItemInfo_t >
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_sound_event_meta_data {
                pub const M_SOUND_EVENT_V_MIX: usize = 0x0; // CStrongHandle< InfoForResourceTypeCVMixListResource >
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cdsp_mixgroup_modifier {
                pub const M_MIXGROUP: usize = 0x0; // CUtlString
                pub const M_FL_MODIFIER: usize = 0x8; // float32
                pub const M_FL_MODIFIER_MIN: usize = 0xC; // float32
                pub const M_FL_SOURCE_MODIFIER: usize = 0x10; // float32
                pub const M_FL_SOURCE_MODIFIER_MIN: usize = 0x14; // float32
                pub const M_FL_LISTENER_REVERB_MODIFIER_WHEN_SOURCE_REVERB_IS_ACTIVE: usize = 0x18; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_dsp_preset_modifier_list {
                pub const M_DSP_NAME: usize = 0x0; // CUtlString
                pub const M_MODIFIERS: usize = 0x8; // CUtlVector< CDSPMixgroupModifier >
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_base {
                pub const M_CURVES: usize = 0x20; // CUtlDict< CPiecewiseCurve, F( size=1 ) >
            }
            // Parent: c_voice_container_base
            // Fields count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_default {
            }
            // Parent: c_voice_container_base
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_wav_file_reader {
                pub const M_WAV_FILE_PATH: usize = 0x60; // CUtlString
            }
            // Parent: c_voice_container_wav_file_reader
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_test_constant {
                pub const M_FL_TEST_CONSTANT_PARAM: usize = 0x68; // bool
            }
            // Parent: c_voice_container_base
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_test_nested_dynamic {
                pub const M_SOUND_TO_PLAY: usize = 0x60; // CStrongHandle< InfoForResourceTypeCVoiceContainerBase >
                pub const M_FL_TEST_CONSTANT_PARAM: usize = 0x68; // float32
            }
            // Parent: c_voice_container_base
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_decaying_sine_wave {
                pub const M_FL_FREQUENCY: usize = 0x60; // float32
                pub const M_FL_DECAY_TIME: usize = 0x64; // float32
            }
            // Parent: c_voice_container_decaying_sine_wave
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_amped_decaying_sine_wave {
                pub const M_FL_GAIN_AMOUNT: usize = 0x68; // float32
            }
            // Parent: c_voice_container_base
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_realtime_fm_sine_wave {
                pub const M_FL_CARRIER_FREQUENCY: usize = 0x60; // float32
                pub const M_FL_MODULATOR_FREQUENCY: usize = 0x64; // float32
                pub const M_FL_MODULATOR_AMOUNT: usize = 0x68; // float32
            }
            // Parent: c_voice_container_base
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_blend {
                pub const M_H_SOUND_ONE: usize = 0x60; // CStrongHandle< InfoForResourceTypeCVoiceContainerBase >
                pub const M_H_SOUND_TWO: usize = 0x68; // CStrongHandle< InfoForResourceTypeCVoiceContainerBase >
                pub const M_FL_BLEND_AMOUNT: usize = 0x70; // float32
            }
            // Parent: c_voice_container_base
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_engine_sound {
                pub const M_SOUND_TO_PLAY: usize = 0x60; // CStrongHandle< InfoForResourceTypeCVoiceContainerBase >
                pub const M_FL_TEST_CONSTANT_PARAM: usize = 0x68; // float32
                pub const M_FL_TEST_SOUND_EVENT_BOUND_PARAM: usize = 0x6C; // float32
                pub const M_FL_ENGINE_RPM: usize = 0x70; // float32
            }
            // Parent: c_voice_container_wav_file_reader
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_envelope_analyzer {
                pub const M_ENV_BUFFER: usize = 0x68; // CUtlVector< float32 >
            }
            // Parent: c_voice_container_base
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voice_container_random_sampler {
                pub const M_FL_LOUD_AMPLITUDE: usize = 0x60; // float32
                pub const M_FL_LOUD_AMPLITUDE_JITTER: usize = 0x64; // float32
                pub const M_FL_SOFT_AMPLITUDE: usize = 0x68; // float32
                pub const M_FL_SOFT_AMPLITUDE_JITTER: usize = 0x6C; // float32
                pub const M_FL_LOUD_TIME_JITTER: usize = 0x70; // float32
                pub const M_FL_SOFT_TIME_JITTER: usize = 0x74; // float32
                pub const M_GRAIN_RESOURCES: usize = 0x78; // CUtlVector< CStrongHandle< InfoForResourceTypeCVoiceContainerBase > >
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_filter_desc_t {
                pub const M_N_FILTER_TYPE: usize = 0x0; // VMixFilterType_t
                pub const M_N_FILTER_SLOPE: usize = 0x2; // VMixFilterSlope_t
                pub const M_B_ENABLED: usize = 0x3; // bool
                pub const M_FLDB_GAIN: usize = 0x4; // float32
                pub const M_FL_CUTOFF_FREQ: usize = 0x8; // float32
                pub const M_FL_Q: usize = 0xC; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_eq8_desc_t {
                pub const M_STAGES: usize = 0x0; // VMixFilterDesc_t[8]
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_delay_desc_t {
                pub const M_FEEDBACK_FILTER: usize = 0x0; // VMixFilterDesc_t
                pub const M_B_ENABLE_FILTER: usize = 0x10; // bool
                pub const M_FL_DELAY: usize = 0x14; // float32
                pub const M_FL_DIRECT_GAIN: usize = 0x18; // float32
                pub const M_FL_DELAY_GAIN: usize = 0x1C; // float32
                pub const M_FL_FEEDBACK_GAIN: usize = 0x20; // float32
                pub const M_FL_WIDTH: usize = 0x24; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_panner_desc_t {
                pub const M_TYPE: usize = 0x0; // VMixPannerType_t
                pub const M_FL_STRENGTH: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_mod_delay_desc_t {
                pub const M_FEEDBACK_FILTER: usize = 0x0; // VMixFilterDesc_t
                pub const M_B_PHASE_INVERT: usize = 0x10; // bool
                pub const M_FL_GLIDE_TIME: usize = 0x14; // float32
                pub const M_FL_DELAY: usize = 0x18; // float32
                pub const M_FL_OUTPUT_GAIN: usize = 0x1C; // float32
                pub const M_FL_FEEDBACK_GAIN: usize = 0x20; // float32
                pub const M_FL_MOD_RATE: usize = 0x24; // float32
                pub const M_FL_MOD_DEPTH: usize = 0x28; // float32
                pub const M_B_APPLY_ANTIALIASING: usize = 0x2C; // bool
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_diffusor_desc_t {
                pub const M_FL_SIZE: usize = 0x0; // float32
                pub const M_FL_COMPLEXITY: usize = 0x4; // float32
                pub const M_FL_FEEDBACK: usize = 0x8; // float32
                pub const M_FL_OUTPUT_GAIN: usize = 0xC; // float32
            }
            // Parent: None
            // Fields count: 17
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_boxverb_desc_t {
                pub const M_FL_SIZE_MAX: usize = 0x0; // float32
                pub const M_FL_SIZE_MIN: usize = 0x4; // float32
                pub const M_FL_COMPLEXITY: usize = 0x8; // float32
                pub const M_FL_DIFFUSION: usize = 0xC; // float32
                pub const M_FL_MOD_DEPTH: usize = 0x10; // float32
                pub const M_FL_MOD_RATE: usize = 0x14; // float32
                pub const M_B_PARALLEL: usize = 0x18; // bool
                pub const M_FILTER_TYPE: usize = 0x1C; // VMixFilterDesc_t
                pub const M_FL_WIDTH: usize = 0x2C; // float32
                pub const M_FL_HEIGHT: usize = 0x30; // float32
                pub const M_FL_DEPTH: usize = 0x34; // float32
                pub const M_FL_FEEDBACK_SCALE: usize = 0x38; // float32
                pub const M_FL_FEEDBACK_WIDTH: usize = 0x3C; // float32
                pub const M_FL_FEEDBACK_HEIGHT: usize = 0x40; // float32
                pub const M_FL_FEEDBACK_DEPTH: usize = 0x44; // float32
                pub const M_FL_OUTPUT_GAIN: usize = 0x48; // float32
                pub const M_FL_TAPS: usize = 0x4C; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_freeverb_desc_t {
                pub const M_FL_ROOM_SIZE: usize = 0x0; // float32
                pub const M_FL_DAMP: usize = 0x4; // float32
                pub const M_FL_WIDTH: usize = 0x8; // float32
                pub const M_FL_LATE_REFLECTIONS: usize = 0xC; // float32
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_plateverb_desc_t {
                pub const M_FL_PREFILTER: usize = 0x0; // float32
                pub const M_FL_INPUT_DIFFUSION1: usize = 0x4; // float32
                pub const M_FL_INPUT_DIFFUSION2: usize = 0x8; // float32
                pub const M_FL_DECAY: usize = 0xC; // float32
                pub const M_FL_DAMP: usize = 0x10; // float32
                pub const M_FL_FEEDBACK_DIFFUSION1: usize = 0x14; // float32
                pub const M_FL_FEEDBACK_DIFFUSION2: usize = 0x18; // float32
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_dynamics_desc_t {
                pub const M_FLDB_GAIN: usize = 0x0; // float32
                pub const M_FLDB_NOISE_GATE_THRESHOLD: usize = 0x4; // float32
                pub const M_FLDB_COMPRESSION_THRESHOLD: usize = 0x8; // float32
                pub const M_FLDB_LIMITER_THRESHOLD: usize = 0xC; // float32
                pub const M_FLDB_KNEE_WIDTH: usize = 0x10; // float32
                pub const M_FL_RATIO: usize = 0x14; // float32
                pub const M_FL_LIMITER_RATIO: usize = 0x18; // float32
                pub const M_FL_ATTACK_TIME_MS: usize = 0x1C; // float32
                pub const M_FL_RELEASE_TIME_MS: usize = 0x20; // float32
                pub const M_FL_RMS_TIME_MS: usize = 0x24; // float32
                pub const M_FL_WET_MIX: usize = 0x28; // float32
                pub const M_B_PEAK_MODE: usize = 0x2C; // bool
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_dynamics_compressor_desc_t {
                pub const M_FLDB_OUTPUT_GAIN: usize = 0x0; // float32
                pub const M_FLDB_COMPRESSION_THRESHOLD: usize = 0x4; // float32
                pub const M_FLDB_KNEE_WIDTH: usize = 0x8; // float32
                pub const M_FL_COMPRESSION_RATIO: usize = 0xC; // float32
                pub const M_FL_ATTACK_TIME_MS: usize = 0x10; // float32
                pub const M_FL_RELEASE_TIME_MS: usize = 0x14; // float32
                pub const M_FL_RMS_TIME_MS: usize = 0x18; // float32
                pub const M_FL_WET_MIX: usize = 0x1C; // float32
                pub const M_B_PEAK_MODE: usize = 0x20; // bool
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_dynamics_band_t {
                pub const M_FLDB_GAIN_INPUT: usize = 0x0; // float32
                pub const M_FLDB_GAIN_OUTPUT: usize = 0x4; // float32
                pub const M_FLDB_THRESHOLD_BELOW: usize = 0x8; // float32
                pub const M_FLDB_THRESHOLD_ABOVE: usize = 0xC; // float32
                pub const M_FL_RATIO_BELOW: usize = 0x10; // float32
                pub const M_FL_RATIO_ABOVE: usize = 0x14; // float32
                pub const M_FL_ATTACK_TIME_MS: usize = 0x18; // float32
                pub const M_FL_RELEASE_TIME_MS: usize = 0x1C; // float32
                pub const M_B_ENABLE: usize = 0x20; // bool
                pub const M_B_SOLO: usize = 0x21; // bool
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_dynamics3_band_desc_t {
                pub const M_FLDB_GAIN_OUTPUT: usize = 0x0; // float32
                pub const M_FL_RMS_TIME_MS: usize = 0x4; // float32
                pub const M_FLDB_KNEE_WIDTH: usize = 0x8; // float32
                pub const M_FL_DEPTH: usize = 0xC; // float32
                pub const M_FL_WET_MIX: usize = 0x10; // float32
                pub const M_FL_TIME_SCALE: usize = 0x14; // float32
                pub const M_FL_LOW_CUTOFF_FREQ: usize = 0x18; // float32
                pub const M_FL_HIGH_CUTOFF_FREQ: usize = 0x1C; // float32
                pub const M_B_PEAK_MODE: usize = 0x20; // bool
                pub const M_BAND_DESC: usize = 0x24; // VMixDynamicsBand_t[3]
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_envelope_desc_t {
                pub const M_FL_ATTACK_TIME_MS: usize = 0x0; // float32
                pub const M_FL_HOLD_TIME_MS: usize = 0x4; // float32
                pub const M_FL_RELEASE_TIME_MS: usize = 0x8; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_pitch_shift_desc_t {
                pub const M_N_GRAIN_SAMPLE_COUNT: usize = 0x0; // int32
                pub const M_FL_PITCH_SHIFT: usize = 0x4; // float32
                pub const M_N_QUALITY: usize = 0x8; // int32
                pub const M_N_PROC_TYPE: usize = 0xC; // int32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_convolution_desc_t {
                pub const M_FLDB_GAIN: usize = 0x0; // float32
                pub const M_FL_PRE_DELAY_MS: usize = 0x4; // float32
                pub const M_FL_WET_MIX: usize = 0x8; // float32
                pub const M_FLDB_LOW: usize = 0xC; // float32
                pub const M_FLDB_MID: usize = 0x10; // float32
                pub const M_FLDB_HIGH: usize = 0x14; // float32
                pub const M_FL_LOW_CUTOFF_FREQ: usize = 0x18; // float32
                pub const M_FL_HIGH_CUTOFF_FREQ: usize = 0x1C; // float32
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_vocoder_desc_t {
                pub const M_N_BAND_COUNT: usize = 0x0; // int32
                pub const M_FL_BANDWIDTH: usize = 0x4; // float32
                pub const M_FLD_B_MOD_GAIN: usize = 0x8; // float32
                pub const M_FL_FREQ_RANGE_START: usize = 0xC; // float32
                pub const M_FL_FREQ_RANGE_END: usize = 0x10; // float32
                pub const M_FLD_B_UNVOICED_GAIN: usize = 0x14; // float32
                pub const M_FL_ATTACK_TIME_MS: usize = 0x18; // float32
                pub const M_FL_RELEASE_TIME_MS: usize = 0x1C; // float32
                pub const M_N_DEBUG_BAND: usize = 0x20; // int32
                pub const M_B_PEAK_MODE: usize = 0x24; // bool
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_shaper_desc_t {
                pub const M_N_SHAPE: usize = 0x0; // int32
                pub const M_FLDB_DRIVE: usize = 0x4; // float32
                pub const M_FLDB_OUTPUT_GAIN: usize = 0x8; // float32
                pub const M_FL_WET_MIX: usize = 0xC; // float32
                pub const M_N_OVERSAMPLE_FACTOR: usize = 0x10; // int32
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_utility_desc_t {
                pub const M_N_OP: usize = 0x0; // VMixChannelOperation_t
                pub const M_FL_INPUT_PAN: usize = 0x4; // float32
                pub const M_FL_OUTPUT_BALANCE: usize = 0x8; // float32
                pub const M_FLDB_OUTPUT_GAIN: usize = 0xC; // float32
                pub const M_B_BASS_MONO: usize = 0x10; // bool
                pub const M_FL_BASS_FREQ: usize = 0x14; // float32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_auto_filter_desc_t {
                pub const M_FL_ENVELOPE_AMOUNT: usize = 0x0; // float32
                pub const M_FL_ATTACK_TIME_MS: usize = 0x4; // float32
                pub const M_FL_RELEASE_TIME_MS: usize = 0x8; // float32
                pub const M_FILTER: usize = 0xC; // VMixFilterDesc_t
                pub const M_FL_LFO_AMOUNT: usize = 0x1C; // float32
                pub const M_FL_LFO_RATE: usize = 0x20; // float32
                pub const M_FL_PHASE: usize = 0x24; // float32
                pub const M_N_LFO_SHAPE: usize = 0x28; // VMixLFOShape_t
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_osc_desc_t {
                pub const OSC_TYPE: usize = 0x0; // VMixLFOShape_t
                pub const M_FREQ: usize = 0x4; // float32
                pub const M_FL_PHASE: usize = 0x8; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_effect_chain_desc_t {
                pub const M_FL_CROSSFADE_TIME: usize = 0x0; // float32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod v_mix_subgraph_switch_desc_t {
                pub const M_INTERPOLATION_MODE: usize = 0x0; // VMixSubgraphSwitchInterpolationType_t
                pub const M_B_ONLY_TAILS_ON_FADE_OUT: usize = 0x4; // bool
                pub const M_FL_INTERPOLATION_TIME: usize = 0x8; // float32
            }
        }
        // Module: vphysics2.dll
        // Classes count: 88
        // Enums count: 2
        pub mod vphysics2 {
            // Alignment: 4
            // Members count: 4
            #[repr(i32)]
            pub enum JointAxisT {
                JointAxisX = 0,
                JointAxisY = 1,
                JointAxisZ = 2,
                JointAxisCount = 3
            }
            // Alignment: 4
            // Members count: 3
            #[repr(i32)]
            pub enum JointMotionT {
                JointMotionFree = 0,
                JointMotionLocked = 1,
                JointMotionCount = 2
            }
            // Parent: None
            // Fields count: 5
            pub mod constraint_breakableparams_t {
                pub const STRENGTH: usize = 0x0; // float32
                pub const FORCE_LIMIT: usize = 0x4; // float32
                pub const TORQUE_LIMIT: usize = 0x8; // float32
                pub const BODY_MASS_SCALE: usize = 0xC; // float32[2]
                pub const IS_ACTIVE: usize = 0x14; // bool
            }
            // Parent: None
            // Fields count: 4
            pub mod constraint_axislimit_t {
                pub const FL_MIN_ROTATION: usize = 0x0; // float32
                pub const FL_MAX_ROTATION: usize = 0x4; // float32
                pub const FL_MOTOR_TARGET_ANG_SPEED: usize = 0x8; // float32
                pub const FL_MOTOR_MAX_TORQUE: usize = 0xC; // float32
            }
            // Parent: None
            // Fields count: 4
            pub mod constraint_hingeparams_t {
                pub const WORLD_POSITION: usize = 0x0; // 
                pub const WORLD_AXIS_DIRECTION: usize = 0xC; // 
                pub const HINGE_AXIS: usize = 0x18; // constraint_axislimit_t
                pub const CONSTRAINT: usize = 0x28; // constraint_breakableparams_t
            }
            // Parent: None
            // Fields count: 0
            pub mod i_physics_player_controller {
            }
            // Parent: None
            // Fields count: 35
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_fe_jiggle_bone {
                pub const M_N_FLAGS: usize = 0x0; // uint32
                pub const M_FL_LENGTH: usize = 0x4; // float32
                pub const M_FL_TIP_MASS: usize = 0x8; // float32
                pub const M_FL_YAW_STIFFNESS: usize = 0xC; // float32
                pub const M_FL_YAW_DAMPING: usize = 0x10; // float32
                pub const M_FL_PITCH_STIFFNESS: usize = 0x14; // float32
                pub const M_FL_PITCH_DAMPING: usize = 0x18; // float32
                pub const M_FL_ALONG_STIFFNESS: usize = 0x1C; // float32
                pub const M_FL_ALONG_DAMPING: usize = 0x20; // float32
                pub const M_FL_ANGLE_LIMIT: usize = 0x24; // float32
                pub const M_FL_MIN_YAW: usize = 0x28; // float32
                pub const M_FL_MAX_YAW: usize = 0x2C; // float32
                pub const M_FL_YAW_FRICTION: usize = 0x30; // float32
                pub const M_FL_YAW_BOUNCE: usize = 0x34; // float32
                pub const M_FL_MIN_PITCH: usize = 0x38; // float32
                pub const M_FL_MAX_PITCH: usize = 0x3C; // float32
                pub const M_FL_PITCH_FRICTION: usize = 0x40; // float32
                pub const M_FL_PITCH_BOUNCE: usize = 0x44; // float32
                pub const M_FL_BASE_MASS: usize = 0x48; // float32
                pub const M_FL_BASE_STIFFNESS: usize = 0x4C; // float32
                pub const M_FL_BASE_DAMPING: usize = 0x50; // float32
                pub const M_FL_BASE_MIN_LEFT: usize = 0x54; // float32
                pub const M_FL_BASE_MAX_LEFT: usize = 0x58; // float32
                pub const M_FL_BASE_LEFT_FRICTION: usize = 0x5C; // float32
                pub const M_FL_BASE_MIN_UP: usize = 0x60; // float32
                pub const M_FL_BASE_MAX_UP: usize = 0x64; // float32
                pub const M_FL_BASE_UP_FRICTION: usize = 0x68; // float32
                pub const M_FL_BASE_MIN_FORWARD: usize = 0x6C; // float32
                pub const M_FL_BASE_MAX_FORWARD: usize = 0x70; // float32
                pub const M_FL_BASE_FORWARD_FRICTION: usize = 0x74; // float32
                pub const M_FL_RADIUS0: usize = 0x78; // float32
                pub const M_FL_RADIUS1: usize = 0x7C; // float32
                pub const M_V_POINT0: usize = 0x80; // 
                pub const M_V_POINT1: usize = 0x8C; // 
                pub const M_N_COLLISION_MASK: usize = 0x98; // uint16
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_fe_named_jiggle_bone {
                pub const M_STR_PARENT_BONE: usize = 0x0; // CUtlString
                pub const M_TRANSFORM: usize = 0x10; // CTransform
                pub const M_N_JIGGLE_PARENT: usize = 0x30; // uint32
                pub const M_JIGGLE_BONE: usize = 0x34; // CFeJiggleBone
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_fe_indexed_jiggle_bone {
                pub const M_N_NODE: usize = 0x0; // uint32
                pub const M_N_JIGGLE_PARENT: usize = 0x4; // uint32
                pub const M_JIGGLE_BONE: usize = 0x8; // CFeJiggleBone
            }
            // Parent: None
            // Fields count: 100
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod phys_fe_model_desc_t {
                pub const M_CTRL_HASH: usize = 0x0; // CUtlVector< uint32 >
                pub const M_CTRL_NAME: usize = 0x18; // CUtlVector< CUtlString >
                pub const M_N_STATIC_NODE_FLAGS: usize = 0x30; // uint32
                pub const M_N_DYNAMIC_NODE_FLAGS: usize = 0x34; // uint32
                pub const M_FL_LOCAL_FORCE: usize = 0x38; // float32
                pub const M_FL_LOCAL_ROTATION: usize = 0x3C; // float32
                pub const M_N_NODE_COUNT: usize = 0x40; // uint16
                pub const M_N_STATIC_NODES: usize = 0x42; // uint16
                pub const M_N_ROT_LOCK_STATIC_NODES: usize = 0x44; // uint16
                pub const M_N_FIRST_POSITION_DRIVEN_NODE: usize = 0x46; // uint16
                pub const M_N_SIMD_TRI_COUNT1: usize = 0x48; // uint16
                pub const M_N_SIMD_TRI_COUNT2: usize = 0x4A; // uint16
                pub const M_N_SIMD_QUAD_COUNT1: usize = 0x4C; // uint16
                pub const M_N_SIMD_QUAD_COUNT2: usize = 0x4E; // uint16
                pub const M_N_QUAD_COUNT1: usize = 0x50; // uint16
                pub const M_N_QUAD_COUNT2: usize = 0x52; // uint16
                pub const M_N_TREE_DEPTH: usize = 0x54; // uint16
                pub const M_N_NODE_BASE_JIGGLEBONE_DEPENDS_COUNT: usize = 0x56; // uint16
                pub const M_N_ROPE_COUNT: usize = 0x58; // uint16
                pub const M_ROPES: usize = 0x60; // CUtlVector< uint16 >
                pub const M_NODE_BASES: usize = 0x78; // CUtlVector< FeNodeBase_t >
                pub const M_SIMD_NODE_BASES: usize = 0x90; // CUtlVector< FeSimdNodeBase_t >
                pub const M_QUADS: usize = 0xA8; // CUtlVector< FeQuad_t >
                pub const M_SIMD_QUADS: usize = 0xC0; // CUtlVector< FeSimdQuad_t >
                pub const M_SIMD_TRIS: usize = 0xD8; // CUtlVector< FeSimdTri_t >
                pub const M_SIMD_RODS: usize = 0xF0; // CUtlVector< FeSimdRodConstraint_t >
                pub const M_SIMD_RODS_ANIM: usize = 0x108; // CUtlVector< FeSimdRodConstraintAnim_t >
                pub const M_INIT_POSE: usize = 0x120; // CUtlVector< CTransform >
                pub const M_RODS: usize = 0x138; // CUtlVector< FeRodConstraint_t >
                pub const M_TWISTS: usize = 0x150; // CUtlVector< FeTwistConstraint_t >
                pub const M_AXIAL_EDGES: usize = 0x168; // CUtlVector< FeAxialEdgeBend_t >
                pub const M_NODE_INV_MASSES: usize = 0x180; // CUtlVector< float32 >
                pub const M_CTRL_OFFSETS: usize = 0x198; // CUtlVector< FeCtrlOffset_t >
                pub const M_CTRL_OS_OFFSETS: usize = 0x1B0; // CUtlVector< FeCtrlOsOffset_t >
                pub const M_FOLLOW_NODES: usize = 0x1C8; // CUtlVector< FeFollowNode_t >
                pub const M_COLLISION_PLANES: usize = 0x1E0; // CUtlVector< FeCollisionPlane_t >
                pub const M_NODE_INTEGRATOR: usize = 0x1F8; // CUtlVector< FeNodeIntegrator_t >
                pub const M_SPRING_INTEGRATOR: usize = 0x210; // CUtlVector< FeSpringIntegrator_t >
                pub const M_SIMD_SPRING_INTEGRATOR: usize = 0x228; // CUtlVector< FeSimdSpringIntegrator_t >
                pub const M_WORLD_COLLISION_PARAMS: usize = 0x240; // CUtlVector< FeWorldCollisionParams_t >
                pub const M_LEGACY_STRETCH_FORCE: usize = 0x258; // CUtlVector< float32 >
                pub const M_NODE_COLLISION_RADII: usize = 0x270; // CUtlVector< float32 >
                pub const M_DYN_NODE_FRICTION: usize = 0x288; // CUtlVector< float32 >
                pub const M_LOCAL_ROTATION: usize = 0x2A0; // CUtlVector< float32 >
                pub const M_LOCAL_FORCE: usize = 0x2B8; // CUtlVector< float32 >
                pub const M_TAPERED_CAPSULE_STRETCHES: usize = 0x2D0; // CUtlVector< FeTaperedCapsuleStretch_t >
                pub const M_TAPERED_CAPSULE_RIGIDS: usize = 0x2E8; // CUtlVector< FeTaperedCapsuleRigid_t >
                pub const M_SPHERE_RIGIDS: usize = 0x300; // CUtlVector< FeSphereRigid_t >
                pub const M_WORLD_COLLISION_NODES: usize = 0x318; // CUtlVector< uint16 >
                pub const M_TREE_PARENTS: usize = 0x330; // CUtlVector< uint16 >
                pub const M_TREE_COLLISION_MASKS: usize = 0x348; // CUtlVector< uint16 >
                pub const M_TREE_CHILDREN: usize = 0x360; // CUtlVector< FeTreeChildren_t >
                pub const M_FREE_NODES: usize = 0x378; // CUtlVector< uint16 >
                pub const M_FIT_MATRICES: usize = 0x390; // CUtlVector< FeFitMatrix_t >
                pub const M_FIT_WEIGHTS: usize = 0x3A8; // CUtlVector< FeFitWeight_t >
                pub const M_REVERSE_OFFSETS: usize = 0x3C0; // CUtlVector< FeNodeReverseOffset_t >
                pub const M_ANIM_STRAY_RADII: usize = 0x3D8; // CUtlVector< FeAnimStrayRadius_t >
                pub const M_SIMD_ANIM_STRAY_RADII: usize = 0x3F0; // CUtlVector< FeSimdAnimStrayRadius_t >
                pub const M_KELAGER_BENDS: usize = 0x408; // CUtlVector< FeKelagerBend2_t >
                pub const M_CTRL_SOFT_OFFSETS: usize = 0x420; // CUtlVector< FeCtrlSoftOffset_t >
                pub const M_JIGGLE_BONES: usize = 0x438; // CUtlVector< CFeIndexedJiggleBone >
                pub const M_SOURCE_ELEMS: usize = 0x450; // CUtlVector< uint16 >
                pub const M_GOAL_DAMPED_SPRING_INTEGRATORS: usize = 0x468; // CUtlVector< uint32 >
                pub const M_TRIS: usize = 0x480; // CUtlVector< FeTri_t >
                pub const M_N_TRI_COUNT1: usize = 0x498; // uint16
                pub const M_N_TRI_COUNT2: usize = 0x49A; // uint16
                pub const M_N_RESERVED_UINT8: usize = 0x49C; // uint8
                pub const M_N_EXTRA_PRESSURE_ITERATIONS: usize = 0x49D; // uint8
                pub const M_N_EXTRA_GOAL_ITERATIONS: usize = 0x49E; // uint8
                pub const M_N_EXTRA_ITERATIONS: usize = 0x49F; // uint8
                pub const M_BOX_RIGIDS: usize = 0x4A0; // CUtlVector< FeBoxRigid_t >
                pub const M_DYN_NODE_VERTEX_SET: usize = 0x4B8; // CUtlVector< uint8 >
                pub const M_VERTEX_SET_NAMES: usize = 0x4D0; // CUtlVector< uint32 >
                pub const M_RIGID_COLLIDER_PRIORITIES: usize = 0x4E8; // CUtlVector< FeRigidColliderIndices_t >
                pub const M_MORPH_LAYERS: usize = 0x500; // CUtlVector< FeMorphLayerDepr_t >
                pub const M_MORPH_SET_DATA: usize = 0x518; // CUtlVector< uint8 >
                pub const M_VERTEX_MAPS: usize = 0x530; // CUtlVector< FeVertexMapDesc_t >
                pub const M_VERTEX_MAP_VALUES: usize = 0x548; // CUtlVector< uint8 >
                pub const M_EFFECTS: usize = 0x560; // CUtlVector< FeEffectDesc_t >
                pub const M_LOCK_TO_PARENT: usize = 0x578; // CUtlVector< FeCtrlOffset_t >
                pub const M_LOCK_TO_GOAL: usize = 0x590; // CUtlVector< uint16 >
                pub const M_SKEL_PARENTS: usize = 0x5A8; // CUtlVector< int16 >
                pub const M_DYN_NODE_WIND_BASES: usize = 0x5C0; // CUtlVector< FeNodeWindBase_t >
                pub const M_FL_INTERNAL_PRESSURE: usize = 0x5D8; // float32
                pub const M_FL_DEFAULT_TIME_DILATION: usize = 0x5DC; // float32
                pub const M_FL_WINDAGE: usize = 0x5E0; // float32
                pub const M_FL_WIND_DRAG: usize = 0x5E4; // float32
                pub const M_FL_DEFAULT_SURFACE_STRETCH: usize = 0x5E8; // float32
                pub const M_FL_DEFAULT_THREAD_STRETCH: usize = 0x5EC; // float32
                pub const M_FL_DEFAULT_GRAVITY_SCALE: usize = 0x5F0; // float32
                pub const M_FL_DEFAULT_VEL_AIR_DRAG: usize = 0x5F4; // float32
                pub const M_FL_DEFAULT_EXP_AIR_DRAG: usize = 0x5F8; // float32
                pub const M_FL_DEFAULT_VEL_QUAD_AIR_DRAG: usize = 0x5FC; // float32
                pub const M_FL_DEFAULT_EXP_QUAD_AIR_DRAG: usize = 0x600; // float32
                pub const M_FL_ROD_VELOCITY_SMOOTH_RATE: usize = 0x604; // float32
                pub const M_FL_QUAD_VELOCITY_SMOOTH_RATE: usize = 0x608; // float32
                pub const M_FL_ADD_WORLD_COLLISION_RADIUS: usize = 0x60C; // float32
                pub const M_FL_DEFAULT_VOLUMETRIC_SOLVE_AMOUNT: usize = 0x610; // float32
                pub const M_N_ROD_VELOCITY_SMOOTH_ITERATIONS: usize = 0x614; // uint16
                pub const M_N_QUAD_VELOCITY_SMOOTH_ITERATIONS: usize = 0x616; // uint16
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod four_vectors2_d {
                pub const X: usize = 0x0; // fltx4
                pub const Y: usize = 0x10; // fltx4
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_edge_desc_t {
                pub const N_EDGE: usize = 0x0; // uint16[2]
                pub const N_SIDE: usize = 0x4; // uint16[2][2]
                pub const N_VIRT_ELEM: usize = 0xC; // uint16[2]
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod old_fe_edge_t {
                pub const M_FL_K: usize = 0x0; // float32[3]
                pub const INV_A: usize = 0xC; // float32
                pub const T: usize = 0x10; // float32
                pub const FL_THETA_RELAXED: usize = 0x14; // float32
                pub const FL_THETA_FACTOR: usize = 0x18; // float32
                pub const C01: usize = 0x1C; // float32
                pub const C02: usize = 0x20; // float32
                pub const C03: usize = 0x24; // float32
                pub const C04: usize = 0x28; // float32
                pub const FL_AXIAL_MODEL_DIST: usize = 0x2C; // float32
                pub const FL_AXIAL_MODEL_WEIGHTS: usize = 0x30; // float32[4]
                pub const M_N_NODE: usize = 0x40; // uint16[4]
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_weighted_node_t {
                pub const N_NODE: usize = 0x0; // uint16
                pub const N_WEIGHT: usize = 0x2; // uint16
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_kelager_bend2_t {
                pub const FL_WEIGHT: usize = 0x0; // float32[3]
                pub const FL_HEIGHT0: usize = 0xC; // float32
                pub const N_NODE: usize = 0x10; // uint16[3]
                pub const N_RESERVED: usize = 0x16; // uint16
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_stiff_hinge_build_t {
                pub const FL_MAX_ANGLE: usize = 0x0; // float32
                pub const FL_STRENGTH: usize = 0x4; // float32
                pub const FL_MOTION_BIAS: usize = 0x8; // float32[3]
                pub const N_NODE: usize = 0x14; // uint16[3]
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_tri_t {
                pub const N_NODE: usize = 0x0; // uint16[3]
                pub const W1: usize = 0x8; // float32
                pub const W2: usize = 0xC; // float32
                pub const V1X: usize = 0x10; // float32
                pub const V2: usize = 0x14; // Vector2D
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_simd_tri_t {
                pub const N_NODE: usize = 0x0; // uint32[4][3]
                pub const W1: usize = 0x30; // fltx4
                pub const W2: usize = 0x40; // fltx4
                pub const V1X: usize = 0x50; // fltx4
                pub const V2: usize = 0x60; // FourVectors2D
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_quad_t {
                pub const N_NODE: usize = 0x0; // uint16[4]
                pub const FL_SLACK: usize = 0x8; // float32
                pub const V_SHAPE: usize = 0xC; // Vector4D[4]
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_node_base_t {
                pub const N_NODE: usize = 0x0; // uint16
                pub const N_DUMMY: usize = 0x2; // uint16[3]
                pub const N_NODE_X0: usize = 0x8; // uint16
                pub const N_NODE_X1: usize = 0xA; // uint16
                pub const N_NODE_Y0: usize = 0xC; // uint16
                pub const N_NODE_Y1: usize = 0xE; // uint16
                pub const Q_ADJUST: usize = 0x10; // QuaternionStorage
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_node_wind_base_t {
                pub const N_NODE_X0: usize = 0x0; // uint16
                pub const N_NODE_X1: usize = 0x2; // uint16
                pub const N_NODE_Y0: usize = 0x4; // uint16
                pub const N_NODE_Y1: usize = 0x6; // uint16
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_node_reverse_offset_t {
                pub const V_OFFSET: usize = 0x0; // 
                pub const N_BONE_CTRL: usize = 0xC; // uint16
                pub const N_TARGET_NODE: usize = 0xE; // uint16
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_simd_quad_t {
                pub const N_NODE: usize = 0x0; // uint16[4][4]
                pub const F4_SLACK: usize = 0x20; // fltx4
                pub const V_SHAPE: usize = 0x30; // FourVectors[4]
                pub const F4_WEIGHTS: usize = 0xF0; // fltx4[4]
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_axial_edge_bend_t {
                pub const TE: usize = 0x0; // float32
                pub const TV: usize = 0x4; // float32
                pub const FL_DIST: usize = 0x8; // float32
                pub const FL_WEIGHT: usize = 0xC; // float32[4]
                pub const N_NODE: usize = 0x1C; // uint16[6]
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_band_bend_limit_t {
                pub const FL_DIST_MIN: usize = 0x0; // float32
                pub const FL_DIST_MAX: usize = 0x4; // float32
                pub const N_NODE: usize = 0x8; // uint16[6]
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_rod_constraint_t {
                pub const N_NODE: usize = 0x0; // uint16[2]
                pub const FL_MAX_DIST: usize = 0x4; // float32
                pub const FL_MIN_DIST: usize = 0x8; // float32
                pub const FL_WEIGHT0: usize = 0xC; // float32
                pub const FL_RELAXATION_FACTOR: usize = 0x10; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_twist_constraint_t {
                pub const N_NODE_ORIENT: usize = 0x0; // uint16
                pub const N_NODE_END: usize = 0x2; // uint16
                pub const FL_TWIST_RELAX: usize = 0x4; // float32
                pub const FL_SWING_RELAX: usize = 0x8; // float32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_simd_rod_constraint_t {
                pub const N_NODE: usize = 0x0; // uint16[4][2]
                pub const F4_MAX_DIST: usize = 0x10; // fltx4
                pub const F4_MIN_DIST: usize = 0x20; // fltx4
                pub const F4_WEIGHT0: usize = 0x30; // fltx4
                pub const F4_RELAXATION_FACTOR: usize = 0x40; // fltx4
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_simd_rod_constraint_anim_t {
                pub const N_NODE: usize = 0x0; // uint16[4][2]
                pub const F4_WEIGHT0: usize = 0x10; // fltx4
                pub const F4_RELAXATION_FACTOR: usize = 0x20; // fltx4
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_anim_stray_radius_t {
                pub const N_NODE: usize = 0x0; // uint16[2]
                pub const FL_MAX_DIST: usize = 0x4; // float32
                pub const FL_RELAXATION_FACTOR: usize = 0x8; // float32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_simd_anim_stray_radius_t {
                pub const N_NODE: usize = 0x0; // uint16[4][2]
                pub const FL_MAX_DIST: usize = 0x10; // fltx4
                pub const FL_RELAXATION_FACTOR: usize = 0x20; // fltx4
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_simd_node_base_t {
                pub const N_NODE: usize = 0x0; // uint16[4]
                pub const N_NODE_X0: usize = 0x8; // uint16[4]
                pub const N_NODE_X1: usize = 0x10; // uint16[4]
                pub const N_NODE_Y0: usize = 0x18; // uint16[4]
                pub const N_NODE_Y1: usize = 0x20; // uint16[4]
                pub const N_DUMMY: usize = 0x28; // uint16[4]
                pub const Q_ADJUST: usize = 0x30; // FourQuaternions
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_node_integrator_t {
                pub const FL_POINT_DAMPING: usize = 0x0; // float32
                pub const FL_ANIMATION_FORCE_ATTRACTION: usize = 0x4; // float32
                pub const FL_ANIMATION_VERTEX_ATTRACTION: usize = 0x8; // float32
                pub const FL_GRAVITY: usize = 0xC; // float32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_spring_integrator_t {
                pub const N_NODE: usize = 0x0; // uint16[2]
                pub const FL_SPRING_REST_LENGTH: usize = 0x4; // float32
                pub const FL_SPRING_CONSTANT: usize = 0x8; // float32
                pub const FL_SPRING_DAMPING: usize = 0xC; // float32
                pub const FL_NODE_WEIGHT0: usize = 0x10; // float32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_simd_spring_integrator_t {
                pub const N_NODE: usize = 0x0; // uint16[4][2]
                pub const FL_SPRING_REST_LENGTH: usize = 0x10; // fltx4
                pub const FL_SPRING_CONSTANT: usize = 0x20; // fltx4
                pub const FL_SPRING_DAMPING: usize = 0x30; // fltx4
                pub const FL_NODE_WEIGHT0: usize = 0x40; // fltx4
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_ctrl_offset_t {
                pub const V_OFFSET: usize = 0x0; // 
                pub const N_CTRL_PARENT: usize = 0xC; // uint16
                pub const N_CTRL_CHILD: usize = 0xE; // uint16
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_soft_parent_t {
                pub const N_PARENT: usize = 0x0; // int32
                pub const FL_ALPHA: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_ctrl_soft_offset_t {
                pub const N_CTRL_PARENT: usize = 0x0; // uint16
                pub const N_CTRL_CHILD: usize = 0x2; // uint16
                pub const V_OFFSET: usize = 0x4; // 
                pub const FL_ALPHA: usize = 0x10; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_ctrl_os_offset_t {
                pub const N_CTRL_PARENT: usize = 0x0; // uint16
                pub const N_CTRL_CHILD: usize = 0x2; // uint16
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_follow_node_t {
                pub const N_PARENT_NODE: usize = 0x0; // uint16
                pub const N_CHILD_NODE: usize = 0x2; // uint16
                pub const FL_WEIGHT: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_collision_plane_t {
                pub const N_CTRL_PARENT: usize = 0x0; // uint16
                pub const N_CHILD_NODE: usize = 0x2; // uint16
                pub const M_PLANE: usize = 0x4; // RnPlane_t
                pub const FL_STRENGTH: usize = 0x14; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_world_collision_params_t {
                pub const FL_WORLD_FRICTION: usize = 0x0; // float32
                pub const FL_GROUND_FRICTION: usize = 0x4; // float32
                pub const N_LIST_BEGIN: usize = 0x8; // uint16
                pub const N_LIST_END: usize = 0xA; // uint16
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_tree_children_t {
                pub const N_CHILD: usize = 0x0; // uint16[2]
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_tapered_capsule_rigid_t {
                pub const V_SPHERE: usize = 0x0; // fltx4[2]
                pub const N_NODE: usize = 0x20; // uint16
                pub const N_COLLISION_MASK: usize = 0x22; // uint16
                pub const N_VERTEX_MAP_INDEX: usize = 0x24; // uint16
                pub const N_FLAGS: usize = 0x26; // uint16
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_sphere_rigid_t {
                pub const V_SPHERE: usize = 0x0; // fltx4
                pub const N_NODE: usize = 0x10; // uint16
                pub const N_COLLISION_MASK: usize = 0x12; // uint16
                pub const N_VERTEX_MAP_INDEX: usize = 0x14; // uint16
                pub const N_FLAGS: usize = 0x16; // uint16
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_tapered_capsule_stretch_t {
                pub const N_NODE: usize = 0x0; // uint16[2]
                pub const N_COLLISION_MASK: usize = 0x4; // uint16
                pub const N_DUMMY: usize = 0x6; // uint16
                pub const FL_RADIUS: usize = 0x8; // float32[2]
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_box_rigid_t {
                pub const TM_FRAME2: usize = 0x0; // CTransform
                pub const N_NODE: usize = 0x20; // uint16
                pub const N_COLLISION_MASK: usize = 0x22; // uint16
                pub const V_SIZE: usize = 0x24; // 
                pub const N_VERTEX_MAP_INDEX: usize = 0x30; // uint16
                pub const N_FLAGS: usize = 0x32; // uint16
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cov_matrix3 {
                pub const M_V_DIAG: usize = 0x0; // 
                pub const M_FL_XY: usize = 0xC; // float32
                pub const M_FL_XZ: usize = 0x10; // float32
                pub const M_FL_YZ: usize = 0x14; // float32
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod four_cov_matrices3 {
                pub const M_V_DIAG: usize = 0x0; // FourVectors
                pub const M_FL_XY: usize = 0x30; // fltx4
                pub const M_FL_XZ: usize = 0x40; // fltx4
                pub const M_FL_YZ: usize = 0x50; // fltx4
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_fit_weight_t {
                pub const FL_WEIGHT: usize = 0x0; // float32
                pub const N_NODE: usize = 0x4; // uint16
                pub const N_DUMMY: usize = 0x6; // uint16
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_fit_influence_t {
                pub const N_VERTEX_NODE: usize = 0x0; // uint32
                pub const FL_WEIGHT: usize = 0x4; // float32
                pub const N_MATRIX_NODE: usize = 0x8; // uint32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_fit_matrix_t {
                pub const BONE: usize = 0x0; // CTransform
                pub const V_CENTER: usize = 0x20; // 
                pub const N_END: usize = 0x2C; // uint16
                pub const N_NODE: usize = 0x2E; // uint16
                pub const N_BEGIN_DYNAMIC: usize = 0x30; // uint16
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_rigid_collider_indices_t {
                pub const M_N_TAPERED_CAPSULE_RIGID_INDEX: usize = 0x0; // uint16
                pub const M_N_SPHERE_RIGID_INDEX: usize = 0x2; // uint16
                pub const M_N_BOX_RIGID_INDEX: usize = 0x4; // uint16
                pub const M_N_COLLISION_PLANE_INDEX: usize = 0x6; // uint16
            }
            // Parent: fe_tapered_capsule_rigid_t
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_build_tapered_capsule_rigid_t {
                pub const M_N_PRIORITY: usize = 0x30; // int32
                pub const M_N_VERTEX_MAP_HASH: usize = 0x34; // uint32
            }
            // Parent: fe_box_rigid_t
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_build_box_rigid_t {
                pub const M_N_PRIORITY: usize = 0x40; // int32
                pub const M_N_VERTEX_MAP_HASH: usize = 0x44; // uint32
            }
            // Parent: fe_sphere_rigid_t
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_build_sphere_rigid_t {
                pub const M_N_PRIORITY: usize = 0x20; // int32
                pub const M_N_VERTEX_MAP_HASH: usize = 0x24; // uint32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_source_edge_t {
                pub const N_NODE: usize = 0x0; // uint16[2]
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_effect_desc_t {
                pub const S_NAME: usize = 0x0; // CUtlString
                pub const N_NAME_HASH: usize = 0x8; // uint32
                pub const N_TYPE: usize = 0xC; // int32
                pub const M_PARAMS: usize = 0x10; // KeyValues3
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_vertex_map_build_t {
                pub const M_VERTEX_MAP_NAME: usize = 0x0; // CUtlString
                pub const M_N_NAME_HASH: usize = 0x8; // uint32
                pub const M_COLOR: usize = 0xC; // Color
                pub const M_FL_VOLUMETRIC_SOLVE_STRENGTH: usize = 0x10; // float32
                pub const M_N_SCALE_SOURCE_NODE: usize = 0x14; // int32
                pub const M_WEIGHTS: usize = 0x18; // CUtlVector< float32 >
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_fe_vertex_map_build_array {
                pub const M_ARRAY: usize = 0x0; // CUtlVector< FeVertexMapBuild_t* >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_proxy_vertex_map_t {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_FL_WEIGHT: usize = 0x8; // float32
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_vertex_map_desc_t {
                pub const S_NAME: usize = 0x0; // CUtlString
                pub const N_NAME_HASH: usize = 0x8; // uint32
                pub const N_COLOR: usize = 0xC; // uint32
                pub const N_FLAGS: usize = 0x10; // uint32
                pub const N_VERTEX_BASE: usize = 0x14; // uint16
                pub const N_VERTEX_COUNT: usize = 0x16; // uint16
                pub const N_MAP_OFFSET: usize = 0x18; // uint32
                pub const N_NODE_LIST_OFFSET: usize = 0x1C; // uint32
                pub const V_CENTER_OF_MASS: usize = 0x20; // 
                pub const FL_VOLUMETRIC_SOLVE_STRENGTH: usize = 0x2C; // float32
                pub const N_SCALE_SOURCE_NODE: usize = 0x30; // int16
                pub const N_NODE_LIST_COUNT: usize = 0x32; // uint16
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod fe_morph_layer_depr_t {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_N_NAME_HASH: usize = 0x8; // uint32
                pub const M_NODES: usize = 0x10; // CUtlVector< uint16 >
                pub const M_INIT_POS: usize = 0x28; // CUtlVector< Vector >
                pub const M_GRAVITY: usize = 0x40; // CUtlVector< float32 >
                pub const M_GOAL_STRENGTH: usize = 0x58; // CUtlVector< float32 >
                pub const M_GOAL_DAMPING: usize = 0x70; // CUtlVector< float32 >
                pub const M_N_FLAGS: usize = 0x88; // uint32
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_fe_morph_layer {
                pub const M_NAME: usize = 0x0; // CUtlString
                pub const M_N_NAME_HASH: usize = 0x8; // uint32
                pub const M_NODES: usize = 0x10; // CUtlVector< uint16 >
                pub const M_INIT_POS: usize = 0x28; // CUtlVector< Vector >
                pub const M_GRAVITY: usize = 0x40; // CUtlVector< float32 >
                pub const M_GOAL_STRENGTH: usize = 0x58; // CUtlVector< float32 >
                pub const M_GOAL_DAMPING: usize = 0x70; // CUtlVector< float32 >
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod dop26_t {
                pub const M_FL_SUPPORT: usize = 0x0; // float32[26]
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_capsule_t {
                pub const M_V_CENTER: usize = 0x0; // Vector[2]
                pub const M_FL_RADIUS: usize = 0x18; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_plane_t {
                pub const M_V_NORMAL: usize = 0x0; // 
                pub const M_FL_OFFSET: usize = 0xC; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_vertex_t {
                pub const M_N_EDGE: usize = 0x0; // uint8
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_half_edge_t {
                pub const M_N_NEXT: usize = 0x0; // uint8
                pub const M_N_TWIN: usize = 0x1; // uint8
                pub const M_N_ORIGIN: usize = 0x2; // uint8
                pub const M_N_FACE: usize = 0x3; // uint8
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_face_t {
                pub const M_N_EDGE: usize = 0x0; // uint8
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_region_svm {
                pub const M_PLANES: usize = 0x0; // CUtlVector< RnPlane_t >
                pub const M_NODES: usize = 0x18; // CUtlVector< uint32 >
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_hull_t {
                pub const M_V_CENTROID: usize = 0x0; // 
                pub const M_FL_MAX_ANGULAR_RADIUS: usize = 0xC; // float32
                pub const M_BOUNDS: usize = 0x10; // AABB_t
                pub const M_V_ORTHOGRAPHIC_AREAS: usize = 0x28; // 
                pub const M_MASS_PROPERTIES: usize = 0x34; // matrix3x4_t
                pub const M_FL_VOLUME: usize = 0x64; // float32
                pub const M_FL_SURFACE_AREA: usize = 0x68; // float32
                pub const M_VERTICES: usize = 0x70; // CUtlVector< RnVertex_t >
                pub const M_VERTEX_POSITIONS: usize = 0x88; // CUtlVector< Vector >
                pub const M_EDGES: usize = 0xA0; // CUtlVector< RnHalfEdge_t >
                pub const M_FACES: usize = 0xB8; // CUtlVector< RnFace_t >
                pub const M_FACE_PLANES: usize = 0xD0; // CUtlVector< RnPlane_t >
                pub const M_N_FLAGS: usize = 0xE8; // uint32
                pub const M_P_REGION_SVM: usize = 0xF0; // CRegionSVM*
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_triangle_t {
                pub const M_N_INDEX: usize = 0x0; // int32[3]
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_wing_t {
                pub const M_N_INDEX: usize = 0x0; // int32[3]
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_node_t {
                pub const M_V_MIN: usize = 0x0; // 
                pub const M_N_CHILDREN: usize = 0xC; // uint32
                pub const M_V_MAX: usize = 0x10; // 
                pub const M_N_TRIANGLE_OFFSET: usize = 0x1C; // uint32
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_mesh_t {
                pub const M_V_MIN: usize = 0x0; // 
                pub const M_V_MAX: usize = 0xC; // 
                pub const M_NODES: usize = 0x18; // CUtlVector< RnNode_t >
                pub const M_VERTICES: usize = 0x30; // CUtlVectorSIMDPaddedVector
                pub const M_TRIANGLES: usize = 0x48; // CUtlVector< RnTriangle_t >
                pub const M_WINGS: usize = 0x60; // CUtlVector< RnWing_t >
                pub const M_MATERIALS: usize = 0x78; // CUtlVector< uint8 >
                pub const M_V_ORTHOGRAPHIC_AREAS: usize = 0x90; // 
                pub const M_N_FLAGS: usize = 0x9C; // uint32
                pub const M_N_DEBUG_FLAGS: usize = 0xA0; // uint32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_shape_desc_t {
                pub const M_N_COLLISION_ATTRIBUTE_INDEX: usize = 0x0; // uint32
                pub const M_N_SURFACE_PROPERTY_INDEX: usize = 0x4; // uint32
                pub const M_USER_FRIENDLY_NAME: usize = 0x8; // CUtlString
            }
            // Parent: rn_shape_desc_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_sphere_desc_t {
                pub const M_SPHERE: usize = 0x10; // SphereBase_t< float32 >
            }
            // Parent: rn_shape_desc_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_capsule_desc_t {
                pub const M_CAPSULE: usize = 0x10; // RnCapsule_t
            }
            // Parent: rn_shape_desc_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_hull_desc_t {
                pub const M_HULL: usize = 0x10; // RnHull_t
            }
            // Parent: rn_shape_desc_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_mesh_desc_t {
                pub const M_MESH: usize = 0x10; // RnMesh_t
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_softbody_particle_t {
                pub const M_FL_MASS_INV: usize = 0x0; // float32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_softbody_spring_t {
                pub const M_N_PARTICLE: usize = 0x0; // uint16[2]
                pub const M_FL_LENGTH: usize = 0x4; // float32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_softbody_capsule_t {
                pub const M_V_CENTER: usize = 0x0; // Vector[2]
                pub const M_FL_RADIUS: usize = 0x18; // float32
                pub const M_N_PARTICLE: usize = 0x1C; // uint16[2]
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_blend_vertex_t {
                pub const M_N_WEIGHT0: usize = 0x0; // uint16
                pub const M_N_INDEX0: usize = 0x2; // uint16
                pub const M_N_WEIGHT1: usize = 0x4; // uint16
                pub const M_N_INDEX1: usize = 0x6; // uint16
                pub const M_N_WEIGHT2: usize = 0x8; // uint16
                pub const M_N_INDEX2: usize = 0xA; // uint16
                pub const M_N_FLAGS: usize = 0xC; // uint16
                pub const M_N_TARGET_INDEX: usize = 0xE; // uint16
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod cast_sphere_sat_params_t {
                pub const M_V_RAY_START: usize = 0x0; // 
                pub const M_V_RAY_DELTA: usize = 0xC; // 
                pub const M_FL_RADIUS: usize = 0x18; // float32
                pub const M_FL_MAX_FRACTION: usize = 0x1C; // float32
                pub const M_FL_SCALE: usize = 0x20; // float32
                pub const M_P_HULL: usize = 0x28; // RnHull_t*
            }
            // Parent: None
            // Fields count: 35
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod rn_body_desc_t {
                pub const M_S_DEBUG_NAME: usize = 0x0; // CUtlString
                pub const M_V_POSITION: usize = 0x8; // 
                pub const M_Q_ORIENTATION: usize = 0x14; // QuaternionStorage
                pub const M_V_LINEAR_VELOCITY: usize = 0x24; // 
                pub const M_V_ANGULAR_VELOCITY: usize = 0x30; // 
                pub const M_V_LOCAL_MASS_CENTER: usize = 0x3C; // 
                pub const M_LOCAL_INERTIA_INV: usize = 0x48; // Vector[3]
                pub const M_FL_MASS_INV: usize = 0x6C; // float32
                pub const M_FL_GAME_MASS: usize = 0x70; // float32
                pub const M_FL_INERTIA_SCALE_INV: usize = 0x74; // float32
                pub const M_FL_LINEAR_DAMPING: usize = 0x78; // float32
                pub const M_FL_ANGULAR_DAMPING: usize = 0x7C; // float32
                pub const M_FL_LINEAR_DRAG: usize = 0x80; // float32
                pub const M_FL_ANGULAR_DRAG: usize = 0x84; // float32
                pub const M_FL_LINEAR_BUOYANCY_DRAG: usize = 0x88; // float32
                pub const M_FL_ANGULAR_BUOYANCY_DRAG: usize = 0x8C; // float32
                pub const M_V_LAST_AWAKE_FORCE_ACCUM: usize = 0x90; // 
                pub const M_V_LAST_AWAKE_TORQUE_ACCUM: usize = 0x9C; // 
                pub const M_FL_BUOYANCY_FACTOR: usize = 0xA8; // float32
                pub const M_FL_GRAVITY_SCALE: usize = 0xAC; // float32
                pub const M_FL_TIME_SCALE: usize = 0xB0; // float32
                pub const M_N_BODY_TYPE: usize = 0xB4; // int32
                pub const M_N_GAME_INDEX: usize = 0xB8; // uint32
                pub const M_N_GAME_FLAGS: usize = 0xBC; // uint32
                pub const M_N_MIN_VELOCITY_ITERATIONS: usize = 0xC0; // int8
                pub const M_N_MIN_POSITION_ITERATIONS: usize = 0xC1; // int8
                pub const M_N_MASS_PRIORITY: usize = 0xC2; // int8
                pub const M_B_ENABLED: usize = 0xC3; // bool
                pub const M_B_SLEEPING: usize = 0xC4; // bool
                pub const M_B_IS_CONTINUOUS_ENABLED: usize = 0xC5; // bool
                pub const M_B_DRAG_ENABLED: usize = 0xC6; // bool
                pub const M_B_BUOYANCY_DRAG_ENABLED: usize = 0xC7; // bool
                pub const M_B_GRAVITY_DISABLED: usize = 0xC8; // bool
                pub const M_B_SPECULATIVE_ENABLED: usize = 0xC9; // bool
                pub const M_B_HAS_SHADOW_CONTROLLER: usize = 0xCA; // bool
            }
            // Parent: None
            // Fields count: 2
            pub mod vertex_position_normal_t {
                pub const M_V_POSITION: usize = 0x0; // 
                pub const M_V_NORMAL: usize = 0xC; // 
            }
            // Parent: None
            // Fields count: 1
            pub mod vertex_position_color_t {
                pub const M_V_POSITION: usize = 0x0; // 
            }
            // Parent: rn_body_desc_t
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod vphysics_save_cphysicsbody_t {
                pub const M_N_OLD_POINTER: usize = 0xD0; // uint64
            }
        }
        // Module: worldrenderer.dll
        // Classes count: 27
        // Enums count: 1
        pub mod worldrenderer {
            // Alignment: 4
            // Members count: 13
            #[repr(i32)]
            pub enum ObjectTypeFlagsT {
                ObjectTypeNone = 0,
                ObjectTypeModel = 8,
                ObjectTypeBlockLight = 16,
                ObjectTypeNoShadows = 32,
                ObjectTypeWorldspaceTexureBlend = 64,
                ObjectTypeDisabledInLowQuality = 128,
                ObjectTypeNoSunShadows = 256,
                ObjectTypeRenderWithDynamic = 512,
                ObjectTypeRenderToCubemaps = 1024,
                ObjectTypeModelHasLods = 2048,
                ObjectTypeOverlay = 8192,
                ObjectTypePrecomputedVismembers = 16384,
                ObjectTypeStaticCubeMap = 32768
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod entity_io_connection_data_t {
                pub const M_OUTPUT_NAME: usize = 0x0; // CUtlString
                pub const M_TARGET_TYPE: usize = 0x8; // uint32
                pub const M_TARGET_NAME: usize = 0x10; // CUtlString
                pub const M_INPUT_NAME: usize = 0x18; // CUtlString
                pub const M_OVERRIDE_PARAM: usize = 0x20; // CUtlString
                pub const M_FL_DELAY: usize = 0x28; // float32
                pub const M_N_TIMES_TO_FIRE: usize = 0x2C; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod entity_key_value_data_t {
                pub const M_CONNECTIONS: usize = 0x8; // CUtlVector< EntityIOConnectionData_t >
                pub const M_KEY_VALUES_DATA: usize = 0x20; // CUtlBinaryBlock
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod perm_entity_lump_data_t {
                pub const M_NAME: usize = 0x8; // CUtlString
                pub const M_HAMMER_UNIQUE_ID: usize = 0x10; // CUtlString
                pub const M_CHILD_LUMPS: usize = 0x18; // CUtlVector< CStrongHandleCopyable< InfoForResourceTypeCEntityLump > >
                pub const M_ENTITY_KEY_VALUES: usize = 0x30; // CUtlLeanVector< EntityKeyValueData_t >
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod scene_object_t {
                pub const M_N_OBJECT_ID: usize = 0x0; // uint32
                pub const M_V_TRANSFORM: usize = 0x4; // Vector4D[3]
                pub const M_FL_FADE_START_DISTANCE: usize = 0x34; // float32
                pub const M_FL_FADE_END_DISTANCE: usize = 0x38; // float32
                pub const M_V_TINT_COLOR: usize = 0x3C; // Vector4D
                pub const M_SKIN: usize = 0x50; // CUtlString
                pub const M_N_OBJECT_TYPE_FLAGS: usize = 0x58; // ObjectTypeFlags_t
                pub const M_V_LIGHTING_ORIGIN: usize = 0x5C; // 
                pub const M_N_OVERLAY_RENDER_ORDER: usize = 0x68; // int16
                pub const M_N_LOD_OVERRIDE: usize = 0x6A; // int16
                pub const M_N_CUBE_MAP_PRECOMPUTED_HANDSHAKE: usize = 0x6C; // int32
                pub const M_N_LIGHT_PROBE_VOLUME_PRECOMPUTED_HANDSHAKE: usize = 0x70; // int32
                pub const M_RENDERABLE_MODEL: usize = 0x78; // CStrongHandle< InfoForResourceTypeCModel >
                pub const M_RENDERABLE: usize = 0x80; // CStrongHandle< InfoForResourceTypeCRenderMesh >
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod base_scene_object_override_t {
                pub const M_N_SCENE_OBJECT_INDEX: usize = 0x0; // uint32
            }
            // Parent: base_scene_object_override_t
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod extra_vertex_stream_override_t {
                pub const M_N_SUB_SCENE_OBJECT: usize = 0x4; // uint32
                pub const M_N_DRAW_CALL_INDEX: usize = 0x8; // uint32
                pub const M_N_ADDITIONAL_MESH_DRAW_PRIMITIVE_FLAGS: usize = 0xC; // MeshDrawPrimitiveFlags_t
                pub const M_EXTRA_BUFFER_BINDING: usize = 0x10; // CRenderBufferBinding
            }
            // Parent: base_scene_object_override_t
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod material_override_t {
                pub const M_N_SUB_SCENE_OBJECT: usize = 0x4; // uint32
                pub const M_N_DRAW_CALL_INDEX: usize = 0x8; // uint32
                pub const M_P_MATERIAL: usize = 0x10; // CStrongHandle< InfoForResourceTypeIMaterial2 >
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod info_overlay_data_t {
                pub const M_TRANSFORM: usize = 0x0; // matrix3x4_t
                pub const M_FL_WIDTH: usize = 0x30; // float32
                pub const M_FL_HEIGHT: usize = 0x34; // float32
                pub const M_FL_DEPTH: usize = 0x38; // float32
                pub const M_V_UV_START: usize = 0x3C; // Vector2D
                pub const M_V_UV_END: usize = 0x44; // Vector2D
                pub const M_P_MATERIAL: usize = 0x50; // CStrongHandle< InfoForResourceTypeIMaterial2 >
                pub const M_N_RENDER_ORDER: usize = 0x58; // int32
                pub const M_V_TINT_COLOR: usize = 0x5C; // Vector4D
                pub const M_N_SEQUENCE_OVERRIDE: usize = 0x6C; // int32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod baked_lighting_info_t {
                pub const M_N_LIGHTMAP_VERSION_NUMBER: usize = 0x0; // uint32
                pub const M_N_LIGHTMAP_GAME_VERSION_NUMBER: usize = 0x4; // uint32
                pub const M_V_LIGHTMAP_UV_SCALE: usize = 0x8; // Vector2D
                pub const M_B_HAS_LIGHTMAPS: usize = 0x10; // bool
                pub const M_LIGHT_MAPS: usize = 0x18; // CUtlVector< CStrongHandle< InfoForResourceTypeCTextureBase > >
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod world_node_on_disk_buffer_data_t {
                pub const M_N_ELEMENT_COUNT: usize = 0x0; // int32
                pub const M_N_ELEMENT_SIZE_IN_BYTES: usize = 0x4; // int32
                pub const M_INPUT_LAYOUT_FIELDS: usize = 0x8; // CUtlVector< RenderInputLayoutField_t >
                pub const M_P_DATA: usize = 0x20; // CUtlVector< uint8 >
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod aggregate_mesh_info_t {
                pub const M_N_VIS_CLUSTER_MEMBER_OFFSET: usize = 0x0; // uint32
                pub const M_N_VIS_CLUSTER_MEMBER_COUNT: usize = 0x4; // uint8
                pub const M_B_HAS_TRANSFORM: usize = 0x5; // bool
                pub const M_N_DRAW_CALL_INDEX: usize = 0x6; // int16
                pub const M_N_LOD_SETUP_INDEX: usize = 0x8; // int16
                pub const M_N_LOD_GROUP_MASK: usize = 0xA; // uint8
                pub const M_V_TINT_COLOR: usize = 0xB; // Color
                pub const M_OBJECT_FLAGS: usize = 0x10; // ObjectTypeFlags_t
                pub const M_N_LIGHT_PROBE_VOLUME_PRECOMPUTED_HANDSHAKE: usize = 0x14; // int32
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod aggregate_lod_setup_t {
                pub const M_V_LOD_ORIGIN: usize = 0x0; // 
                pub const M_F_MAX_OBJECT_SCALE: usize = 0xC; // float32
                pub const M_F_SWITCH_DISTANCES: usize = 0x10; // CUtlVectorFixedGrowable< float32 >
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod aggregate_scene_object_t {
                pub const M_ALL_FLAGS: usize = 0x0; // ObjectTypeFlags_t
                pub const M_ANY_FLAGS: usize = 0x4; // ObjectTypeFlags_t
                pub const M_N_LAYER: usize = 0x8; // int16
                pub const M_AGGREGATE_MESHES: usize = 0x10; // CUtlVector< AggregateMeshInfo_t >
                pub const M_LOD_SETUPS: usize = 0x28; // CUtlVector< AggregateLODSetup_t >
                pub const M_VIS_CLUSTER_MEMBERSHIP: usize = 0x40; // CUtlVector< uint16 >
                pub const M_FRAGMENT_TRANSFORMS: usize = 0x58; // CUtlVector< matrix3x4_t >
                pub const M_RENDERABLE_MODEL: usize = 0x70; // CStrongHandle< InfoForResourceTypeCModel >
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod clutter_tile_t {
                pub const M_N_FIRST_INSTANCE: usize = 0x0; // uint32
                pub const M_N_LAST_INSTANCE: usize = 0x4; // uint32
                pub const M_BOUNDS_WS: usize = 0x8; // AABB_t
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod clutter_scene_object_t {
                pub const M_BOUNDS: usize = 0x0; // AABB_t
                pub const M_FLAGS: usize = 0x18; // ObjectTypeFlags_t
                pub const M_N_LAYER: usize = 0x1C; // int16
                pub const M_INSTANCE_POSITIONS: usize = 0x20; // CUtlVector< Vector >
                pub const M_INSTANCE_SCALES: usize = 0x50; // CUtlVector< float32 >
                pub const M_INSTANCE_TINT_SRGB: usize = 0x68; // CUtlVector< Color >
                pub const M_TILES: usize = 0x80; // CUtlVector< ClutterTile_t >
                pub const M_RENDERABLE_MODEL: usize = 0x98; // CStrongHandle< InfoForResourceTypeCModel >
            }
            // Parent: None
            // Fields count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod world_node_t {
                pub const M_SCENE_OBJECTS: usize = 0x0; // CUtlVector< SceneObject_t >
                pub const M_INFO_OVERLAYS: usize = 0x18; // CUtlVector< InfoOverlayData_t >
                pub const M_VIS_CLUSTER_MEMBERSHIP: usize = 0x30; // CUtlVector< uint16 >
                pub const M_AGGREGATE_SCENE_OBJECTS: usize = 0x48; // CUtlVector< AggregateSceneObject_t >
                pub const M_CLUTTER_SCENE_OBJECTS: usize = 0x60; // CUtlVector< ClutterSceneObject_t >
                pub const M_EXTRA_VERTEX_STREAM_OVERRIDES: usize = 0x78; // CUtlVector< ExtraVertexStreamOverride_t >
                pub const M_MATERIAL_OVERRIDES: usize = 0x90; // CUtlVector< MaterialOverride_t >
                pub const M_EXTRA_VERTEX_STREAMS: usize = 0xA8; // CUtlVector< WorldNodeOnDiskBufferData_t >
                pub const M_LAYER_NAMES: usize = 0xC0; // CUtlVector< CUtlString >
                pub const M_SCENE_OBJECT_LAYER_INDICES: usize = 0xD8; // CUtlVector< uint8 >
                pub const M_OVERLAY_LAYER_INDICES: usize = 0xF0; // CUtlVector< uint8 >
                pub const M_GRASS_FILE_NAME: usize = 0x108; // CUtlString
                pub const M_NODE_LIGHTING_INFO: usize = 0x110; // BakedLightingInfo_t
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod world_builder_params_t {
                pub const M_FL_MIN_DRAW_VOLUME_SIZE: usize = 0x0; // float32
                pub const M_B_BUILD_BAKED_LIGHTING: usize = 0x4; // bool
                pub const M_V_LIGHTMAP_UV_SCALE: usize = 0x8; // Vector2D
                pub const M_N_COMPILE_TIMESTAMP: usize = 0x10; // uint64
                pub const M_N_COMPILE_FINGERPRINT: usize = 0x18; // uint64
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod node_data_t {
                pub const M_N_PARENT: usize = 0x0; // int32
                pub const M_V_ORIGIN: usize = 0x4; // 
                pub const M_V_MIN_BOUNDS: usize = 0x10; // 
                pub const M_V_MAX_BOUNDS: usize = 0x1C; // 
                pub const M_FL_MINIMUM_DISTANCE: usize = 0x28; // float32
                pub const M_CHILD_NODE_INDICES: usize = 0x30; // CUtlVector< int32 >
                pub const M_WORLD_NODE_PREFIX: usize = 0x48; // CUtlString
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod world_t {
                pub const M_BUILDER_PARAMS: usize = 0x0; // WorldBuilderParams_t
                pub const M_WORLD_NODES: usize = 0x20; // CUtlVector< NodeData_t >
                pub const M_WORLD_LIGHTING_INFO: usize = 0x38; // BakedLightingInfo_t
                pub const M_ENTITY_LUMPS: usize = 0x68; // CUtlVector< CStrongHandleCopyable< InfoForResourceTypeCEntityLump > >
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod voxel_vis_block_offset_t {
                pub const M_N_OFFSET: usize = 0x0; // uint32
                pub const M_N_ELEMENT_COUNT: usize = 0x4; // uint32
            }
            // Parent: None
            // Fields count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod c_voxel_visibility {
                pub const M_N_BASE_CLUSTER_COUNT: usize = 0x40; // uint32
                pub const M_N_PVS_BYTES_PER_CLUSTER: usize = 0x44; // uint32
                pub const M_V_MIN_BOUNDS: usize = 0x48; // 
                pub const M_V_MAX_BOUNDS: usize = 0x54; // 
                pub const M_FL_GRID_SIZE: usize = 0x60; // float32
                pub const M_N_SKY_VISIBILITY_CLUSTER: usize = 0x64; // uint32
                pub const M_N_SUN_VISIBILITY_CLUSTER: usize = 0x68; // uint32
                pub const M_NODE_BLOCK: usize = 0x6C; // VoxelVisBlockOffset_t
                pub const M_REGION_BLOCK: usize = 0x74; // VoxelVisBlockOffset_t
                pub const M_ENCLOSED_CLUSTER_LIST_BLOCK: usize = 0x7C; // VoxelVisBlockOffset_t
                pub const M_ENCLOSED_CLUSTERS_BLOCK: usize = 0x84; // VoxelVisBlockOffset_t
                pub const M_MASKS_BLOCK: usize = 0x8C; // VoxelVisBlockOffset_t
                pub const M_N_VIS_BLOCKS: usize = 0x94; // VoxelVisBlockOffset_t
            }
            // Parent: None
            // Fields count: 0
            pub mod v_map_resource_data_t {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            pub mod info_for_resource_type_v_map_resource_data_t {
            }
            // Parent: None
            // Fields count: 0
            pub mod c_entity_component {
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_nameStringableIndex (int32)
            pub mod c_entity_identity {
                pub const M_NAME_STRINGABLE_INDEX: usize = 0x14; // int32
                pub const M_NAME: usize = 0x18; // CUtlSymbolLarge
                pub const M_DESIGNER_NAME: usize = 0x20; // CUtlSymbolLarge
                pub const M_FLAGS: usize = 0x30; // uint32
                pub const M_WORLD_GROUP_ID: usize = 0x38; // WorldGroupId_t
                pub const M_F_DATA_OBJECT_TYPES: usize = 0x3C; // uint32
                pub const M_PATH_INDEX: usize = 0x40; // ChangeAccessorFieldPathIndex_t
                pub const M_P_PREV: usize = 0x58; // CEntityIdentity*
                pub const M_P_NEXT: usize = 0x60; // CEntityIdentity*
                pub const M_P_PREV_BY_CLASS: usize = 0x68; // CEntityIdentity*
                pub const M_P_NEXT_BY_CLASS: usize = 0x70; // CEntityIdentity*
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_pEntity (CEntityIdentity *)
            // NetworkVarNames: m_CScriptComponent (CScriptComponent::Storage_t)
            pub mod c_entity_instance {
                pub const M_ISZ_PRIVATE_V_SCRIPTS: usize = 0x8; // CUtlSymbolLarge
                pub const M_P_ENTITY: usize = 0x10; // CEntityIdentity*
                pub const M_C_SCRIPT_COMPONENT: usize = 0x28; // CScriptComponent*
                pub const M_B_VISIBLEIN_PVS: usize = 0x30; // bool
            }
            // Parent: c_entity_component
            // Fields count: 1
            pub mod c_script_component {
                pub const M_SCRIPT_CLASS_NAME: usize = 0x30; // CUtlSymbolLarge
            }
        }
    }
}
