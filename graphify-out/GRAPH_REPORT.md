# Graph Report - vega  (2026-11-18)

## Corpus Check
- Large corpus: 3740 files · ~5,461,029 words. Semantic extraction will be expensive (many Claude tokens). Consider running on a subfolder.

## Summary
- 111556 nodes · 349049 edges · 2226 communities (1777 shown, 449 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 13398 edges (avg confidence: 0.74)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- UI Actions & Workspace Glue
- Project & Agent Server Core
- LLM Provider Integrations
- Editor Commands & Context
- Multi-Buffer & Diff Hunks
- UI Notices & Components
- GPUI Layout & Examples
- Language Servers & Toolchains
- GPUI Style Primitives
- GPUI Web & Metal Platform
- Apple Platform FFI
- Editor Element Geometry
- Wayland Platform Layer
- Editor Text Transform Actions
- GPUI Window & App Context
- Language Buffer Core
- GPUI Scroll & Input Ops
- Text Shaping & CoreText
- Tasks & JSON Schemas
- Git Proto Messages
- LSP Proto Requests
- Shared Settings Types
- Workspace Actions
- Markdown Rendering
- Remote Collaboration Auth
- Agent Edit Predictions
- Code Action Requests
- Worktree Scanning
- Screen Capture & Wgpu
- GPUI Lists & Minimap
- Agent Settings & Compaction
- X11 Platform Layer
- Language Completions & Deploy
- Pane & Navigation History
- Collab & Debugger Tests
- ACP Thread Content
- Git Repository Backend
- Vim Motions & Display Points
- Git Hosting Providers
- ACP Thread Messages
- Color Contrast & Theming
- Agent Sidebar UI
- Agent Skills & Terminal
- Agent Tools & Extension API
- Mouse & Pane Hit Testing
- Remote Server Protocol
- Agent Conversation View
- Project Panel
- Agent Panel
- Icon Names
- Debug Adapter Sessions
- Item Handles & Following
- Sidebar Tests
- Git Commit UI
- Completion Menu
- ACP Thread Lifecycle
- Project Git Tests
- App Entry & Actions
- Workspace Persistence
- Agent Permission Requests
- Extension Code Labels
- Workspace & Audio Settings
- GPUI App Actions
- GPUI Styled Methods
- Multi-Workspace Windows
- OpenAI Completion Events
- GPUI Frame Profiler
- GPU Atlas & Surfaces
- AWS Bedrock Provider
- Edit Prediction Core
- project: ContinueResponse
- edit_prediction_cli: AnthropicClient
- agent_ui: AvailableCommandInput
- language: ClearHistory
- lsp: run_work_cycle()
- agent: AuthorizationKind
- vim: register()
- git_ui: Close
- editor: parse_zed_link()
- agent: Addon
- agent_ui: copythreadtoclipboard
- keymap_editor: CopyAction
- agent: agentmodelgroupname
- language_model: fake_provider.rs
- agent: bracketmatch
- collab: ack
- settings: asset_str
- workspace: .position_is_valid()
- gpui_linux: asfd
- markdown_preview: bufferdiff
- gpui_wgpu: AdapterInfo
- remote: .global()
- project_panel: project_panel_tests.rs
- src: Accessible Mode {#accessible-m
- sum_tree: Option<BufferId>
- terminal_view: terminal_view.rs
- picker: picker.rs
- zeta_prompt: .format_edit_history()
- editor: Bound
- gpui_macos: anyprotocol
- acp_thread: .can_set_title()
- project: CompletionListItemDefaults
- repl: clearcurrentoutput
- terminal_view: activatenextpane
- ui: axis_as_scrollbaraxis
- agent_servers: agentsessioninfo
- agent_ui: completion_menu_max_width
- language: Buffer
- edit_prediction_metrics: apply_diff_to_string_with_hunk
- agent_ui: ContextMenuEntry
- agent_ui: .is_retained_thread()
- text: .get()
- terminal_view: .to_run()
- terminal: cursor_blinking()
- collab: AnyConnection
- collab: behaviorversion
- client: CrashInfo
- zed: IncomingCall
- terminal: AlacCell
- gpui_windows: common
- workspace: Context
- agent: .create_sibling_thread()
- anthropic: src/anthropic.rs
- acp_thread: mention.rs
- src: debugger.md
- settings_ui: agent_skills_settings_path
- git: BlameBufferResponse
- settings: default.json
- context_server: oauth.rs
- search: base_rem_size_in_px
- git_ui: CheckoutBranch
- agent: .extend_key_context()
- bench_metrics: bench_metrics.rs
- project: .worktree_id()
- settings_content: settings_content/src/language.
- agent_servers: CancelSessionNotification
- gpui: .create_terminal()
- vim: IndentDirection
- buffer_diff: Anchor
- agent_skills: component_matches_ignore_ascii
- language: Anchor
- edit_prediction_cli: FailedHandling
- agent_servers: ConnectionTo
- agent_servers: acpbetafeatureflag
- language: server_precedences()
- debugger_ui: resolve_path()
- edit_prediction_cli: line_start_offset()
- agent_ui: build_terminal_prompt_editor()
- fs: .directories()
- gpui: src/randomized_element_tree.rs
- text: BufferOffsetUtf16
- recent_projects: user_ssh_config_file()
- ui: arrow_key_with_pointer_on_subm
- language_models_cloud: client_supports_status_message
- editor: .get()
- file_finder: parse_file_search_query()
- gpui_windows: WindowAppearance
- language_model_core: language_model_core.rs
- rpc: any
- terminal: Attr
- vim: bracket_pairs
- gpui: .set_input_handler()
- settings_ui: project_skills_relative_path()
- gpui_windows: check_can_show_windows()
- agent_ui: anthropiceventreporter
- fs: CreateOptions
- picker: Stateful
- repl: CellMetadata
- vim: helix.rs
- search: .new()
- outline_panel: entry_git_aware_label_color()
- node_runtime: node_runtime.rs
- sandbox_linux: allow
- agent_ui: choicecard
- collab_ui: CollapseSelectedChannel
- project: point_to_lsp()
- agent_servers: Response
- anthropic: compactionupdate
- gpui: TasksIncluded
- call: call_impl/participant.rs
- editor: .deref()
- gpui: .assert_no_new_leaks()
- search: active_search_view()
- settings_sync: account.rs
- agent_ui: AgentProfileContent
- settings_ui: all_projects()
- settings_content: agent
- project: BreakpointsForFile
- extension_host: codelabelspanliteral
- extension_host: .new()
- editor: assert_chunks_are_contiguous()
- project: AllocateWorktreeId
- git_ui: RestoreFile
- remote: parse_shell()
- reqwest_client: batches
- auto_update: autoupdateevent
- agent_ui: AgentSessionConfigOptions
- agent: agent/src/db.rs
- language: src/buffer_tests.rs
- agent: AcceptEditPredictionBinding
- agent: agentprofileid
- project: BufferReloaded
- editor: BufferSubscription
- agent_ui: OutputTokensUpdate
- client: client/src/llm_token.rs
- dap: IpAddr
- debugger_ui: DebuggerProvider
- zeta_prompt: format_cursor_excerpt()
- cli: app_identifier
- settings_content: InactiveOpacity
- vim: vim/src/command.rs
- agent_settings: agent_settings.rs
- agent_ui: AgentPanelEvent
- git_ui: .staging()
- cloud_api_client: .cached_llm_token()
- file_finder: FileFinder
- settings_content: settings_content/src/editor.rs
- agent_ui: agent_diff_renderer()
- migrator: migrator.rs
- multi_buffer: multi_buffer_tests.rs
- call_hierarchy: buffer_text_style
- gpui_windows: .request_frame()
- project: .stream_enabled_for_test()
- project: .as_ref()
- theme: Item
- src: billing.md
- edit_prediction: asrefstr
- editor: BuildError
- livekit_client: ClaimGrants
- bedrock: models.rs
- sqlez: CacheRow
- fs: BlobReadGateState
- zeta_prompt: extract_marker_span_allow_same
- extension: DebugAdapterRegistryProxy
- dev_container: DevContainerUp
- git_ui: ActivateChangesTab
- sandbox: BpfProgram
- proxy_handshake: establish()
- scheduler: scheduler/src/executor.rs
- theme_settings: FontFamilyName
- audio: ArrayQueue
- settings_content: settings_content/src/agent.rs
- agent_ui: agentmodelicon
- prompt_store: BuiltInPrompt
- auto_update_helper: beginpaint
- deployment: deploy.rs
- editor: ColumnarSelectionRows
- rope: rope.rs
- agent_ui: agentsessionlistrequest
- shell_command_parser: ast
- agent: test_resolve_path()
- context_server: create_initialize_response()
- language_core: .adapter_language_name()
- xtask: container
- gpui_macos: ns_string()
- editor: AcceptInlayHint
- settings: ActivePaneModifiers
- eval_cli: agentconnection_as
- git_ui: commitsummary
- editor: TabEdit
- editor: assert_hovers()
- repl: init()
- agent: SandboxPermissions
- cloud_llm_client: LanguageModelToolResultContent
- editor: columnar_selection.rs
- gpui: frame_cost()
- github-community-pr-board.py: functools
- tasks_ui: Context
- editor: Match
- sandbox: windows_wsl.rs
- settings_content: settings_content/src/project.r
- collab: alias
- docs_preprocessor: bookitem
- collab: .optional_room_transaction()
- deployment: dirs.rs
- livekit_client: audio_mixer
- zed: CliRequest
- google_ai: google_ai/src/completion.rs
- paths: paths/src/paths.rs
- util: util/src/paths.rs
- vim: ClipboardItem
- command_palette: CommandInterceptResult
- copilot_chat: src/copilot_chat.rs
- git_ui: CommitDetails
- gpui_macos: gpui_macos/src/display_link.rs
- gpui: .on_action()
- settings_content: settings_content/src/theme.rs
- debugger_ui: AddWatch
- deployment: AsyncTlsConnector
- gpui_macos: cfdatagetbyteptr
- gpui: AnimationExample
- terminal: hyperlinks.rs
- deployment: authorization
- markdown: cowstr
- gpui: .keyboard_mapper()
- sandbox_linux: broker.rs
- sandbox_linux: OwnedFd
- gpui_macos: appleactivity
- sandbox_linux: asfd_as
- acp_tools: AcpTools
- recent_projects: AddToWorkspace
- project: ContextServer
- search: active_match_index
- deployment: AgentClient
- deployment: model
- collab: AxumMessage
- gpui_apple: Bounds_ScaledPixels
- which_key: humanize_action_name()
- language_model_core: chat_completion.rs
- editor: accepteditprediction
- agent_ui: ContentBlockView
- src: CONTRIBUTING.md
- agent_ui: ArchiveListItem
- component_preview: ActivePageId
- deployment: Server
- dev_container: DevContainerError
- editor: FoldChunks
- workspace: .supported_options()
- editor: AcceptNextLineEditPrediction
- html_to_markdown: fetch_url_content()
- dev_container: DevContainerMessage
- markdown: empty_render_image_does_not_pa
- settings_content: ModelMode
- sandbox_linux: seccomp.rs
- gpui_ios: allocated
- language_model: authenticateerror
- rpc: FakeServer
- http_client: parse_authorization_header()
- git_ui: ButtonStates
- gpui: AnyWindowHandle
- gpui: key_dispatch.rs
- xtask: extension_tests
- eval_cli: function
- gpui_web: router()
- dev_container: devcontainer_json.rs
- editor: cursor_animation.rs
- editor: .map()
- sandbox: sandbox.rs
- tabular_data_preview: render_table.rs
- gpui_linux: Atom
- language_models: CacheTtl
- deployment: Engine
- editor: assert_blame_rows()
- gpui_macos: caller_action_ids_do_not_colli
- image_viewer: displayed_render_image()
- util: darwin.rs
- workspace: ActivePaneDecorator
- settings_ui: contextservercommand
- editor: InsertSnippet
- agent_servers: AvailableCommandsUpdate
- path: test_deserialize_manifest_with
- gpui_macos: .set_initial_windowing()
- project: create_gpui_image()
- src: extensions.md
- rope: ArrayString
- xtask: cargoopt
- acp_thread: .create_terminal()
- editor: FoldAtLevel
- git_ui: branch_mode.rs
- http_proxy: proxy/connection.rs
- scheduler: SessionId
- sandbox_linux: addressfamily
- agent: tools/edit_file_tool.rs
- text: .fmt()
- editor: test_click_on_parameter_inlay_
- extension_host: ExtensionIndex
- git_hosting_providers: gerrit.rs
- extensions_ui: DevExtensionNotInstalledError
- project: integration/dynamic_registrati
- gpui: whitespace_map
- vim: CharClassifier
- lints: ui/map_lookup_then_insert.rs
- net: accept
- agent_ui: build_assist_editor_renderer()
- gpui: example_editor.rs
- gpui_platform: AllocationStats
- agent: templates.rs
- debugger_ui: ActiveBreakpointStripMode
- gpui_apple: metal_renderer.rs
- xtask: .when()
- open_ai: ReasoningEffort
- remote_server: AddWorktree
- http_proxy: SandboxNetworkAccess
- compliance: compliance/src/git.rs
- crashes: crasheventresult
- git_ui_core: git_ui_core/src/notifications.
- livekit_client: ConnectionQuality
- terminal: mappings/mouse.rs
- editor: BlameHover
- channel: .link()
- git_ui_core: DeleteWorktreeTooltip
- multi_buffer: .forget_transaction()
- repl: clearoutput
- git_ui: CompareWithBranch
- xtask: Concurrency
- language_models: ClientTokenProvider
- compliance: authorassociation_as_octocraba
- edit_prediction_types: DataCollectionState
- gpui: .tabbed_windows()
- title_bar: AccountButton
- gpui_apple: CAMetalDrawable
- copilot: copilot/src/request.rs
- debugger_tools: DapLogToolbarItemView
- keymap_editor: keystroke_input.rs
- sandbox_linux: examples/end_to_end.rs
- terminal_view: alternatescroll
- agent: blameentry
- sentry-fetch: collections
- editor: .to_item_events()
- edit_prediction_context: editable_context.rs
- editor: .convert_point_from_companion(
- editor: bookmark_path()
- editor: ActiveScrollbarState
- settings: ActionBuildError
- mermaid_render: accent_colors.rs
- project: AdaptiveBatcher<T>
- util: disambiguate.rs
- xtask: gh_workflow
- call: CallCanceled
- edit_prediction: EditPredictionRejectReason
- debugger_ui: send_telemetry()
- language_tools: EnabledLogStreams
- onboarding: go_to_welcome_page()
- rope: .lines()
- lints: ui/blocking_io_on_foreground.r
- http_client: FetchStep
- agent_ui: agent_configuration.rs
- remote_connection: .ask_password()
- editor: Editor
- vim: Direction
- extension: test_debug_adapter_schemas()
- extension_host: Event
- fs: atomic_replace()
- gpui: process_info.rs
- language_models: add_message_content_part()
- perf: implementation.rs
- agent_ui: ActionLogTelemetry
- eval_cli: AgentInstallSpec
- eval_cli: ArgumentParser
- eval_cli: contextlib
- edit_prediction_cli: format_prompt.rs
- gpui: .or()
- extension_host: .work_dir()
- json_schema_store: $ref
- open_path_prompt: DirectoryState
- agent_ui: agent_panel
- xtask: all
- project: BinarySettings
- collab: Channel
- copilot_ui: .status()
- edit_prediction_ui: ActivePrediction
- gpui: input()
- gpui: bench_task_reports_long_task_w
- language_tools: ServerHealth
- picker_preview: picker_preview.rs
- github-check-new-issue-for-duplicates.py: re
- bedrock: anytoolchoice_as_bedrockanytoo
- agent: contextserverstatus
- copilot_chat: CopilotChatModel
- editor: test_client_can_wrap_with_emme
- git_ui: ButtonStates
- project_panel: DeleteEntryOutcome
- util: truncate_variables()
- theme: default_colors.rs
- x_ai_subscribed: src/x_ai_subscribed.rs
- gpui_ios: cgpoint
- edit_prediction: childentriesoptions
- file_content: coderresult
- release_channel: should_install_crash_handler()
- git_ui: ButtonStates
- zeta_prompt: .tag()
- language_models: provider/open_ai_compatible.rs
- sum_tree: Item
- settings_ui: AnthropicCompatibleAvailableMo
- audio: apm
- agent_ui: collect_markdowns()
- editor: test_setup()
- remote: .fmt()
- gpui: .set_dock_menu()
- language_models: ConfigurationView
- markdown: html_parser.rs
- settings_ui: settings_ui/src/components.rs
- streaming_diff: src/streaming_diff.rs
- etw_tracing: bstr
- time_format: cfabsolutetime
- debugger_ui: Drag
- dev_container: MountDefinition
- diagnostics: init()
- settings_content: settings_content/src/terminal.
- agent: deserialize_maybe_stringified(
- extension: types/lsp.rs
- language: markdown_inline_lang()
- agent: ResolvedMatch
- agent_ui: Diff
- gpui_macros: feature_flags_macros.rs
- git_ui: stash.rs
- toolchain_selector: ImageLoadingExample
- gpui: Context
- gpui: AtomicU16
- fuzzy: char_bag.rs
- workspace: StoreData
- sandbox: FsSetup
- vim: normal/search.rs
- deployment: broadcast
- repl: ClientControlConnection
- collab_ui: CopyLink
- collab: ChannelRole
- copilot: Copilot
- diagnostics: BufferDiagnosticsEditor
- edit_prediction_metrics: reversal.rs
- extension_api: Component
- git_ui: copy_branch_name()
- language_models: BoxStream
- vim: visual.rs
- workspace: AutoHideFade
- github-assign-contributor-issue.py: date
- settings_ui: aisettingitem
- fuzzy_nucleo: batchsize
- json_schema_store: borrowappcontext_as
- gpui_windows: cf_dib
- debugger_ui: .stack_frame_list()
- extension_host: dap::TcpArguments
- gpui: Counter
- migrator: m_2025_01_02/settings.rs
- vim: end_of_document()
- git_ui: date_style
- zed: clone_and_open
- editor: DiagnosticRenderer
- json_schema_store: $ref
- language_tools: DisplayItem
- remote: MessageId
- onboarding: indent_guides
- extension_cli: build_debug_adapter_schema_pat
- deployment: CertificateDer
- tabular_data_preview: ClearSelection
- agent_ui: .emit_load_error()
- agent: AssertionFn
- deployment: .connect()
- hang_telemetry: .queued_events()
- extensions_ui: ExtensionStatus
- project: resolve_git_worktree_to_main_r
- sandbox_linux: RequestedAccess
- theme: init()
- eval_cli: base64
- agent: decide_permission_for_paths()
- db: OnboardingUpsell
- collab: collab.template.yml
- collections: vecmap.rs
- miniprofiler_ui: ProfilingCollector
- language_models: AnthropicLanguageModelProvider
- language_models: BedrockAuthMethodContent
- edit_prediction_cli: PlainLlmClient
- edit_prediction_metrics: src/kept_rate.rs
- extensions_ui: RELOAD_DEBOUNCE_DURATION
- json_schema_store: definitions
- language_models: provider/x_ai.rs
- language_tools: language_tools.rs
- lsp_locations: build_location_matches()
- gpui_platform: BenchWindowContext
- explorer_command_injector: class_e_classnotavailable
- agent: path_matches_always_confirm()
- agent_ui: all_models()
- context_server: AnyNotification
- eval_cli: rejudge.py
- project: lsp_log_view_tests.rs
- project_panel: Trash
- sidebar: App
- github-track-duplicate-bot-effectiveness.py: matplotlib_pyplot
- gpui: binaryheap
- editor: debugger_hover.rs
- git_ui: git_picker.rs
- search: buffer_search_query()
- settings_json: settings_json.rs
- tabular_data_preview: tabular_data_preview/src/parse
- theme_importer: vscode/syntax.rs
- agent_ui: loading_contents_spinner()
- sqlez: AppMigrator
- git_ui: BufferSubscriptions
- gpui: gpui/src/queue.rs
- language_model_core: deserialize()
- gpui_windows: closethreadpooltimer
- sandbox: commandandargs
- project: context_server_store
- deployment: HostKeyPolicy
- title_bar: .get_menus()
- livekit_client: livekit_client/src/lib.rs
- workspace: AnyElement
- ui: gpui::AnimationElement<T>
- vim: helix/paste.rs
- collab: ActiveModel
- agent: BreakpointEventReason
- debugger_ui: CompletionItemType
- agent: .handle_focus_out()
- copilot_chat: ChatLocation
- livekit_api: Client
- lsp_command_selector: active_buffer()
- util: quote_cmd_command_arg_for_oute
- src: freebsd.md
- repl: addcodeblock
- dap: AsyncSeek
- ztracing: Attributes
- action_log: ActionLog
- agent: ActionLog
- agent: .new()
- agent_ui: ChooseProfileMode
- collab: ChannelMember
- edit_prediction_metrics: edit_prediction_metrics/src/ma
- outline: assert_single_caret_at_row()
- settings_ui: page_data.rs
- vim: repeat.rs
- workflows: extension_auto_bump.yml
- workflows: nix_build.yml
- notifications: AddNotification
- agent: EditFileToolPartialInput
- agent_ui: AgentRegistryCard
- bench_metrics: Counter
- collab_ui: ChannelMembership
- eval_cli: benchmarks.py
- extension_host: AsyncFn
- gpui_windows: gpui_windows/src/keyboard.rs
- json_schema_store: schemas/tsconfig.json
- language_models: available_model_to_google_mode
- markdown: html_minifier.rs
- settings_ui: tool_permissions_setup.rs
- src: Action Reference
- workflows: if
- proto: architecture
- github_releases: asyncseekext
- task: cli/build.rs
- deployment_ui: deployment/src/project.rs
- editor: Crease
- language_models: language_model()
- languages: ClientCommand
- mermaid_render: css_color()
- path: abs_path()
- project: compiled_selector()
- remote_server: check_pid_file()
- tabular_data_preview: Context
- util: shell_from_proto()
- terminal: InternalEvent
- compliance: checks.rs
- language_model: image_format_from_external_con
- eval_cli: report.py
- snippet_provider: test_snippets()
- gpui: .read()
- language_core: highlight_cache.rs
- outline_panel: CachedEntry
- recent_projects: DefaultState
- remote: App
- workflows: bump_version.yml
- gpui_macos: autoreleasepool
- gpui: bitflags
- gpui_apple: CFData
- settings: Colors
- collab: connectionid
- context_server: OAuthTokenProvider
- go_to_line: UserCaretPosition
- gpui: underline_width()
- outline_panel: active_outline_panel()
- search: SearchResults
- settings_ui: App
- sqlez: binding_multiple_statements_wi
- workspace: SaveStrategy
- workflows: comment_on_potential_duplicate
- workflows: community_pr_board.yml
- workflows: community_pr_board_refresh.yml
- keymap_editor: ClearKeystrokes
- settings_ui: codestral_api_key_state
- editor: DiffHunkSecondaryStatus
- client: calculate_json_checksum()
- dev_container: devcontainer_api.rs
- edit_prediction_context: bm25_context.rs
- inspector_ui: assert_inspector_watchers()
- json_schema_store: description
- project: pending_op.rs
- sandbox: macos_seatbelt.rs
- theme: scale.rs
- skills: humanizer/SKILL.md
- workflows: welcome_first_contribution.yml
- gpui: alloc
- settings: agent
- project_bookmarks: bookmarkstore
- acp_thread: .patch_preview()
- language_selector: active_picker()
- migrator: m_2025_10_17/settings.rs
- recent_projects: remote_connections.rs
- settings_content: .fmt()
- settings_sync: account_button.rs
- workspace: workspace_error.rs
- workflows: bump_patch_version.yml
- lints: applicability
- action_log: clock
- tab_switcher: CloseSelectedItem
- sandbox_linux: controlmessage
- gpui_windows: controls
- benchmarks: edit_range()
- editor: BookmarksTabEvent
- zed: WatchSnapshot
- gpui: examples/testing.rs
- util: path_to_string()
- json_schema_store: description
- project: Bookmark
- text: Locator
- which_key: text_for_keybinding_keystrokes
- workspace: AnyView
- workspace: OpenRecentProject
- workflows: track_duplicate_bot_effectiven
- xtask: release.rs
- gpui_linux: action
- xtask: annotationkind
- agent: skill_tool.rs
- edit_prediction_cli: teacher_jumps.md
- editor: CodeActionSource
- project_panel: ui_scrollbar_settings_from_raw
- project: .initial_graph_data()
- git_ui_core: build_buffer_diff()
- gpui: gpui/src/subscription.rs
- path: Component
- project: src/debugger.rs
- remote: handle_io()
- search: search/src/search.rs
- theme_selector: init()
- ui: Context
- workspace: AppNotifications
- x_ai_subscribed: do_oauth_flow()
- xtask: PublishPlanEntry
- util: assignprocesstojobobject
- gpui_macros: braced
- extension: compilationconcurrency
- agent: PermissionPattern
- channel: src/channel.rs
- cloud_api_types: KnownOrUnknown
- dev_container: command_json.rs
- terminal: extension_api/src/process.rs
- git_ui: commit_message_editor()
- scheduler: .drop()
- text: ContextLessSummary
- zed: EventWrapper
- vim: increment.rs
- .doc-examples: complex-feature.md
- workflows: after_release.yml
- workflows: community_close_stale_issues.y
- workflows: community_pr_cleanup.yml
- workflows: compliance_check.yml
- workflows: guild_assignment_status.yml
- workflows: wezel.yml
- vim: ChildStdin
- agent: ProjectSnapshot
- benchmarks: benches/edit_file_tool.rs
- context_server: AnyRequest
- sqlez: open_fallback_db()
- edit_prediction_cli: BatchingOpenAiClient
- gpui_windows: directx_devices.rs
- project: BranchDiffEvent
- search: registrar.rs
- tabular_data_preview: ColumnFilterDelegate
- .conventions: CONVENTIONS.md
- workflows: guild_stale_assignments.yml
- workflows: guild_weekly_shipped.yml
- workflows: slack_notify_first_responders.
- compliance: ReviewResult
- gpui_windows: core
- agent_ui: .active_initial_content()
- ai_onboarding: App
- call: call_impl/diagnostics.rs
- feature_flags: DebuggerHistoryFeatureFlag
- edit_prediction: Anchor
- editor: editor/src/clipboard.rs
- json_schema_store: default
- language_models: ConfigurationView
- language: bogus_candidates()
- migrator: migrator/src/migrations.rs
- svg_preview: App
- terminal: assert_wakeups_and_notifies()
- src: Any Bracket Functionality
- workflows: guild_new_pr_notify.yml
- theme_settings: AccentContent
- agent_ui: agentthreadentry
- gpui_web: applicationhandle
- acp_thread: .clear_completed_plan_entries(
- agent: reindent.rs
- editor: .clear_highlights()
- git_ui: common_prefix()
- git_ui: AnyEntity
- git_ui: build_clipboard_buffer()
- gpui: ActiveTextContext
- outline_panel: add_outline_panel()
- project: SerializedBookmark
- project: Buffer
- repl: plain.rs
- theme_selector: .focus_handle()
- vim: surround.rs
- vim: Context
- workspace: multi_workspace_tests.rs
- workspace: Entity<T>
- settings: profiles
- snippets_ui: ConfigureSnippets
- activity_indicator: ActivityIcon
- agent_ui: message_queue.rs
- collab: Database
- edit_prediction_cli: teacher.md
- gpui_shared_string: gpui_shared_string.rs
- gpui_web: SendForegroundExecutor
- markdown: markdown/src/selection.rs
- proto: proto/src/error.rs
- repl: local_kernel_specifications()
- rope: point_utf16.rs
- rpc: rpc/src/auth.rs
- lints: expr_or_init
- xtask: vars.rs
- vim: ChangeCase
- agent_skills: agent_skills/README.md
- agent: default_context_lines()
- editor: scroll_amount.rs
- git_ui: .editor_position()
- input_latency_ui: histogram_bucket_counts()
- gpui: AbsoluteLength
- json_schema_store: description
- language: .modeline()
- sandbox_linux: Action
- settings_content: fallible_options.rs
- workspace: Dismiss
- src: Choosing Language Servers
- tab_switcher: activatepreviousitem
- shell_command_parser: Assignment
- agent: basestrategy_as
- gpui_apple: cascade_list_for_languages
- gpui: conditional
- language: .file_read_time()
- agent: decide_permission_for_path_all
- agent: collapse_multiline_range()
- agent: CompileConfig
- agent_ui: Callout
- agent_ui: App
- dap_adapters: DebugpyLaunchMode
- edit_prediction_metrics: summary.rs
- extension_host: ExtensionKeyValueStore
- gpui: HeadlessAppContext
- platform_title_bar: WindowButton
- gpui_windows: Apartment
- project: InnerTreeNode
- repl: build_grouped_entries()
- sandbox_linux: dns.rs
- settings_ui: pages.rs
- onboarding: agentsetupbutton
- github-find-top-duplicated-bugs.py: argparse
- extension_api: attachrequest
- language: BufferState
- collab_ui: CallDiagnosticsReport
- edit_prediction: fixed_line_window_around_curso
- extension_host: ExtensionKeyValueStore
- extension_host: ExtensionKeyValueStore
- extension_host: ExtensionKeyValueStore
- gpui_windows: DirectManipulationEventHandler
- ui_input: InputField
- ui: PreviewLikeButtons
- zlog: sink.rs
- compliance: .fmt()
- editor: addselectiontothread
- language_models: copilotsettings
- ui: GeneratingSpinner
- dev_container: FeatureOptions
- diagnostics: diagnostics_tests.rs
- editor: editor/src/edit_prediction.rs
- fuzzy_nucleo: fuzzy_nucleo/src/strings.rs
- gpui: AppContext
- vim: duplicate.rs
- src: Basedpyright
- src: AI Assistance
- workflows: danger.yml
- lints: render_consumer/src/lib.rs
- xtask: autofix_pr.rs
- acp_thread: chunk_from_v1()
- agent_ui: cycling_modes_preserves_choice
- benchmarks: markdown_renderer.rs
- deployment: mapper.rs
- edit_prediction: ts_error_count.rs
- edit_prediction_context: CachedDefinition
- editor: inline_input.rs
- gpui_apple: gpui_apple/src/dispatcher.rs
- gpui_macros: derive_app_context.rs
- http_proxy: allowlist.rs
- text: text/src/tests.rs
- ui: AiSettingItem
- workspace: init()
- zlog: src/filter.rs
- src: Check your current scale facto
- workflows: add_commented_closed_issue_to_
- workflows: with
- compliance: OctocrabAuthorAssociation
- settings: horizontal
- editor: cancel_flycheck
- agent: InvalidRegexPattern
- agent_ui: .find_thread_sandbox_error()
- debugger_ui: .focus_handle()
- debugger_ui: ModuleList
- editor: assert_code_lens_block_count()
- editor: .apply_restore()
- editor: Editor
- editor: EditorLspTestContext
- encoding_selector: available_encodings()
- extension_host: latest::DownloadedFileType
- gpui_linux: .get_serial()
- gpui_macros: gpui_macros/src/bench.rs
- gpui_windows: ClusterAnalyzer<'t>
- json_schema_store: description
- open_ai: StreamEvent
- project: DefaultPrettier
- src: `-a`, `--add`
- workflows: with
- xtask: release_job
- terminal: AlacHyperlink
- settings: on_create
- acp_thread: diff_change_label()
- agent: create_directory_tool.rs
- agent: init_test()
- eval_utils: eval_cant_do()
- cloud_api_types: cloud_api_types/src/extension.
- copilot_chat: IncompleteDetails
- dap_adapters: JsDebugAdapter
- editor: DiffHunkKey
- eval_cli: run_index.py
- gpui: hover_targets()
- gpui: BoundsTree<U>
- gpui: AtomicUsize
- json_schema_store: fundingWay
- language_models: authenticate_does_not_fetch_mo
- recent_projects: Cancel
- settings_profile_selector: display_name()
- tabular_data_preview: EditorState
- ui: table_row.rs
- workflows: catch_blank_issues.yml
- workflows: congrats.yml
- workflows: env
- workflows: steps
- fuzzy_nucleo: atomkind
- zed: CliResponse
- edit_prediction_ui: ContextRetrievalStartedDebugEv
- git: .summary()
- git_ui: Any
- gpui_util: ArcCow
- http_proxy: pinned_host.rs
- http_proxy: upstream.rs
- language_core: SymbolKind
- migrator: m_2025_01_29/keymap.rs
- onboarding: BaseKeymapSelector
- settings_content: ModelMode
- zed: GlobalMigrationNotification
- src: Additional AI Options
- workflows: with
- workflows: pr_issue_labeler.yml
- workflows: with
- workflows: release.yml
- workflows: steps
- github-pr-cleanup.py: is_community_pull_request()
- crates_io_client: ApiToken
- lints: ()
- watch: ArcWake
- gpui_windows: clsctx_inproc_server
- acp_thread: src/elicitation.rs
- acp_thread: ForegroundActivity
- client: event_coalescer.rs
- collab: .can_see_channel()
- db: db/src/db.rs
- deployment: servers/compose.yml
- extension_host: latest::DownloadedFileType
- line_ending_selector: init()
- livekit_client: examples/test_app.rs
- project_symbols: init()
- repl: outputs/json.rs
- settings_content: ActionName
- shell_command_parser: and_or_list_validation()
- zlog: init_env_filter()
- src: Adding Context {#adding-contex
- src: A Note on Licensing and Teleme
- src: Additional AI Options
- workflows: steps
- lints: ui/entity_update_in_render.rs
- mermaid_render: bytestext
- ui: codefade
- gpui_macros: Comma
- ztracing: consume_all_tokens_as_debug_sp
- agent: react.py
- agent_ui: MentionCrease
- deployment: copy_with_progress()
- edit_prediction_context: byte_offset_to_position()
- extension_host: ExtensionVersion
- extension_host: latest::DownloadedFileType
- gpui: hot_reload.rs
- language_models: ApiCompatibleProviderConfigura
- recent_projects: App
- settings_ui: push_a11y_text_runs()
- ui: format_distance.rs
- vim: digraph.rs
- src: Cleaning Up Older Keymap Entri
- src: Configuration
- lints: exprkind
- crates_io_client: CratesIoState
- xtask: workflows/extension_tests.rs
- skills: gpui-bench/SKILL.md
- aws_http_client: AwsConnector
- bundle-windows.ps1: blob_store
- agent_ui: .sandbox_confusable_findings()
- agent_ui: ModelIcon
- collab_ui: .update_entries()
- fs: .record_watcher_diagnostics()
- gpui: window/a11y.rs
- gpui_windows: .new()
- language_models: capability_cells_update_when_a
- openai_subscribed: ChatGptModel
- remote_server: tests/command.rs
- ui: E
- src: Aligning clauses with related
- src: Additional AI Options
- workflows: community_update_all_top_ranki
- workflows: community_update_weekly_top_ra
- workflows: good_first_issue_notifier.yml
- workflows: env
- workflows: with
- watch: ArcSwap
- settings_ui: audioinputdevicename
- cli: bash
- collab: ChannelChatParticipantId
- agent: AddSelectionsState
- channel: Channel
- editor: editor/src/rewrap.rs
- eval_cli: eval_cli/README.md
- gpui: Node
- gpui_util: gpui_util/src/lib.rs
- json_schema_store: tsType
- livekit_client: mock_client.rs
- markdown: .events()
- mermaid_render: postprocess.rs
- project: job_debug_queue.rs
- project: CodeLensActionId
- project: .search_history()
- settings_content: ExtendingSet<T>
- theme: theme_settings_provider.rs
- ui: styled_ext.rs
- util: Child
- src: Adding Skills {#adding-skills}
- src: Anthropic {#anthropic}
- src: Agent Panel
- xtask: DocumentMut
- workflows: deploy_nightly_docs.yml
- tests: a11y_atspi_test.py
- crates_io_client: bad_request()
- AGENTS.md: AGENTS.md
- edit_prediction_cli: AnthropicResponse
- extension_api: fetch
- settings: BaseKeymapContent
- agent_ui: test_data_retention_error_maps
- fs: .as_fake()
- fuzzy_nucleo: case_penalty()
- git_ui: .show_commit_message_error()
- gpui: .action_json_schema()
- gpui: profiler/actions.rs
- language_tools: init()
- migrator: m_2025_03_29/settings.rs
- migrator: m_2025_05_08/settings.rs
- project: can_resolve_link()
- rope: .offset_to_point()
- watch: Changed
- src: Automatic scenario creation
- workflows: .github/workflows/run_tests.ym
- workflows: slack_notify_label_created.yml
- workflows: update_duplicate_magnets.yml
- danger: ref_util
- shell_command_parser: AndOrList
- agent_servers: api_key_for_gemini_cli()
- edit_prediction_context: edit_prediction_context_tests.
- snippet: CompletionEdit
- editor: NextScrollCursorCenterTopBotto
- test-extension: CodeLabelSpan
- fuzzy_nucleo: match_path_sets()
- git_ui_core: git_ui_core.rs
- gpui: toggle_headless.rs
- gpui_web: test_canvas_fallback.mjs
- livekit_client: RemoteTrack
- lsp_locations: active_picker()
- mermaid_render: element_fixup.rs
- project: .show_notification()
- project: create_remote_shell()
- repl: RemoteRunningKernel
- ui: control_characters.rs
- .doc-examples: reference.md
- src: Built-in Security Rules
- src: `copy_path`
- src: Avoiding continual rebuilds
- src: Auto-reveal
- src: Cell mode
- workflows: maintainer_edits_nudge.yml
- workflows: env
- settings: git_panel
- agent: DeletePathTool
- cloud_api_types: internal_api.rs
- editor: bracket_colors_markup()
- editor: CollaborationHub
- extensions_ui: Feature
- feature_flags: App
- git_ui: git_runtime_diagnostics.rs
- git_ui: remote_output.rs
- gpui_macros: src/derive_inspector_reflectio
- language_models: add_message_content_part()
- mistral: Model
- oauth_callback_server: oauth_callback_server.rs
- recent_projects: DisconnectedOverlay
- repl: App
- theme: App
- workspace: .detach_and_prompt_err()
- zlog: flush()
- src: Agent-Specific Auth and Config
- src: Additional AI Options
- skills: skills/brand-writer/SKILL.md
- workflows: steps
- workflows: DRY_RUN
- xtask: Permissions
- settings: date_style
- gpui: backspace
- project: ChunksExact
- command_palette: command_palette/src/persistenc
- context_server: .client()
- copilot: File
- dap_adapters: MockDelegate
- extension_api: .context_server_command()
- json_schema_store: additionalProperties
- json_schema_store: description
- language_models: ConfigurationView
- mermaid_render: mermaid_render.rs
- path: windows_prefix.rs
- picker: head.rs
- project: LazyGlobSet
- sandbox_linux: DESIGN.md
- x_ai: Model
- .conventions: .conventions/brand-writer/SKIL
- src: Terminal
- workflows: extensions/workflows/run_tests
- workflows: slack_notify_community_automat
- privacy-policy.md: privacy-policy.md
- compliance: AutomatedChangeFailure
- crates_io_client: MIN_REQUEST_INTERVAL
- crates_io_client: listed()
- zed: ActionStatistics
- gpui_windows: c_uint
- repl: codehandler
- agent_servers: init_test()
- agent: Canvas
- agent: Canvas
- copilot_chat: CopilotChatStatus
- gpui_macros: tests/derive_inspector_reflect
- gpui_util: F
- language_models: language_models.rs
- openai_subscribed: .new()
- outline_panel: BufferOutlines
- platform_title_bar: App
- prettier: prettier_server.js
- project: src/color_extractor.rs
- sandbox: sandbox/README.md
- search: ActionButtonState
- settings_content: settings_content/src/title_bar
- settings_sync: GlobalSettingsSync
- settings_ui: font_picker.rs
- shell_command_parser: TerminalCommandValidation
- terminal: .default()
- ui_macros: dynamic_spacing.rs
- util: util/src/markdown.rs
- .conventions: .conventions/brand-writer/voic
- src: Configuration
- skills: skills/brand-writer/voice-exam
- workflows: bump_collab_staging.yml
- workflows: defaults
- collab: ChannelMemberId
- xtask: commitsfromversiontoversion
- compose.yml: compose.yml
- feedback: copysystemspecsintoclipboard
- agent: prompt.md
- collab: location_project()
- context_server: Child
- edit_prediction_cli: test_tokenize()
- editor: .modify_transaction_selection_
- editor: .apply_stage_or_unstage()
- extension: extension_events.rs
- feature_flags: feature_flags/src/store.rs
- text: .save()
- gpui: .as_f32()
- gpui: spring.rs
- gpui_windows: .add_fonts()
- json_schema_store: description
- language_model_core: tool_schema.rs
- net: async_net.rs
- onboarding: MultibufferHint
- open_router: CacheControl
- openai_subscribed: fake_token_response()
- picker: Picker<D>
- repl: outputs/markdown.rs
- repl: cell_content()
- workspace: InvalidItemView
- zed: mac_only_instance.rs
- src: Approval Prompts {#approval-pr
- src: Actions
- src: bash.md
- ISSUE_TEMPLATE: 10_bug_report.yml
- workflows: release_nightly.yml
- workflows: if
- compliance: CommitAuthor
- skills: gpui-test/SKILL.md
- skills: zed-cherry-pick/SKILL.md
- agent: Bindings
- editor: MessageEditorAddon
- copilot_chat: ResponseFunctionOutput
- dap_adapters: GoDebugAdapter
- dap: dap_types::EvaluateArgumentsCo
- editor: code_completion_tests.rs
- editor: Editor
- git_ui: commit_history_from_response()
- gpui: A11y
- gpui_tokio: gpui_tokio.rs
- gpui_util: DebugAsDisplay
- gpui_windows: .all_font_names()
- gpui_windows: vsync.rs
- project: context_server_store/registry.
- project: ChildEntriesGitIter<'a>
- project: LabelPresence
- project: integration/trusted_worktrees.
- proxy_handshake: PipeStream
- rope: unclipped.rs
- settings_ui: icon_theme_picker()
- settings_ui: OllamaModelPicker
- settings_ui: App
- sqlez: Domain
- util: shell_env.rs
- worktree: IgnoreKind
- .conventions: .conventions/brand-writer/tabo
- README.md: docs/README.md
- src: Amp Notifications {#amp-notifi
- src: Binding runnable tags to task
- skills: skills/brand-writer/taboo-phra
- workflows: with
- workflows: run_bundling.yml
- workflows: triage_queue_board.yml
- lints: LocalDefId
- danger: danger
- PROMPTS.md: PROMPTS.md
- project: color_extractor
- multi_buffer: .expand_excerpt()
- agent: from_pixels_constructor/before
- agent: translate_doc_comments/before.
- agent: FindPathTool
- agent_ui: AgentNotification
- client: zed_urls.rs
- context_server: FakeTransport
- dap_adapters: CodeLldbDebugAdapter
- editor: LinkedEditingRanges
- feature_flags: FeatureFlagValue
- feature_flags: FeatureFlagStore
- file_icons: FileIcons
- git_ui: git_panel_settings.rs
- gpui: gpui/src/test.rs
- gpui: .with_creator()
- gpui_windows: apply_font_features()
- json_schema_store: additionalProperties
- migrator: m_2026_02_04/settings.rs
- picker: Right
- project: integration/signature_help.rs
- recent_projects: recent_projects/src/ssh_config
- refineable: Cascade
- workspace: GlobalHistoryManager
- src: Code formatting
- src: Auto Fold Dirs
- src: Colors not updating after chan
- theme: plugins.js
- renovate.json: renovate.json
- workflows: steps
- workflows: default
- workflows: extension_workflow_rollout.yml
- workflows: with
- workflows: env
- settings: inlay_hints
- acp_thread: subagents.rs
- agent_ui: EndTrialUpsell
- collab: join_channel()
- collab_ui: ChannelEditingState
- context_server: .http()
- copilot_chat: ChatMessage
- deepseek: src/deepseek.rs
- deployment_ui: init()
- gpui: inhibit_idle_sleep()
- gpui: animation_phases_interpolate_o
- gpui_web: queue_test.rs
- inspector_ui: inspector_ui/README.md
- languages: cpp.rs
- livekit_client: remote_video_track_view.rs
- lsp_command_selector: init_selector_test()
- prettier: Prettier
- repl: encode_test_image()
- sqlez_macros: sqlez_macros.rs
- ui: .render()
- workspace: Event
- AGENTS.md: docs/AGENTS.md
- src: AI by Company
- src: Amazon Bedrock {#amazon-bedroc
- src: Build fails: Path too long
- src: Alternate Targets
- ISSUE_TEMPLATE: 11_crash_report.yml
- workflows: deploy_collab.yml
- terms.md: 16.1.
- cargo-timing-info.js: ref_fs
- lints: consumer/src/lib.rs
- settings: title_bar
- agent: LanguageConfiguration
- client: client/src/proxy.rs
- editor: ToggleBlockComments
- editor: blink_manager.rs
- settings_ui: FeatureFlagDescriptor
- fs: git_clone_progress.rs
- gpui: global.rs
- gpui_windows: A11yActionHandler
- json_schema_store: devEngineDependency
- languages: init()
- migrator: m_2026_09_16/settings.rs
- picker: .color()
- project: concurrent_remote_askpass_requ
- util_macros: util_macros.rs
- util: schemars.rs
- zlog: env_config.rs
- lints: defkind
- src: AI usage {#ai-usage}
- src: Call Hierarchy
- src: Attaching to an existing insta
- src: Building from Source
- workflows: bump_zed_version.yml
- workflows: default
- workflows: description
- workflows: with
- workflows: extension_bump.yml
- workflows: ACTIONLINT_BIN
- crates_io_client: CratesIoClient
- DEPLOYMENT.md: DEPLOYMENT.md
- settings: diagnostics
- action_log: .keep_all_edits()
- workflows: evals.rs
- agent: .deref()
- collab: DbErr
- dap: dap_types::Module
- editor: Direction
- editor: GutterButtonIntent
- eval_cli: judge_proxy.py
- gpui_linux: PriorityQueueCalloopReceiver<T
- html_to_markdown: html_to_markdown.rs
- json_schema_store: schemas/package.json
- json_schema_store: anyOf
- markdown: path_range.rs
- migrator: m_2026_04_10/settings.rs
- openai_subscribed: Context
- tabular_data_preview: LineNumber
- telemetry_events: AssistantKind
- telemetry: Event
- .conventions: .conventions/brand-writer/rubr
- .doc-examples: configuration.md
- src: Agent Settings
- src: Agent Panel Usage
- src: Channel Notes
- src: Configuring Zed
- src: Arguments
- src: Debugging
- skills: skills/brand-writer/rubric.md
- workflows: jobs
- workflows: env
- workflows: needs
- workflows: POSTGRES_HOST_AUTH_METHOD
- workflows: POSTGRES_HOST_AUTH_METHOD
- workflows: with
- livekit.yaml: livekit.yaml
- agent_skills: create-skill/SKILL.md
- agent: Range<DisplayRow>
- agent: Accessing raw response data (e
- agent: FindReferencesTool
- agent: GoToDefinitionTool
- agent: RenameTool
- ai_onboarding: .render()
- deepseek: Model
- extension_suggest: extension_suggest.rs
- fuzzy_nucleo: fuzzy_nucleo/src/matcher.rs
- git_ui_core: worktree_names.rs
- gpui: examples/README.md
- gpui: SpringElementState
- gpui: f32
- gpui_windows: Ref
- language_model_core: CompletionIntent
- language_onboarding: BasedPyrightBanner
- mermaid_render: postprocess/util.rs
- openai_subscribed: codex_extra_headers()
- openai_subscribed: compact()
- project_benchmarks: BenchmarkRemoteClient
- project: ContextServerSettings
- repl: CellControlType
- theme_extension: App
- AGENTS.md: Change Classification
- .doc-examples: simple-feature.md
- src: Plans {#plans}
- src: Importing External Agent Threa
- src: 1. Open a Project
- src: globs.md
- src: Choosing a language server
- src: r.md
- src: performance.md
- src: Axes
- html: HtmlExtension
- workflows: autofix_pr.yml
- workflows: pr_number
- workflows: POSTGRES_HOST_AUTH_METHOD
- workflows: default
- workflows: POSTGRES_HOST_AUTH_METHOD
- workflows: env
- subprocessors.md: subprocessors.md
- terms.md: terms.md
- compliance: escape_markdown_link_text()
- collab: ChannelBufferCollaboratorId
- agent_servers: .list_sessions()
- agent_ui: profile_modal_header.rs
- ai_onboarding: AgentPanelOnboarding
- ai_onboarding: edit_prediction_onboarding_con
- channel: channel_store_tests.rs
- collab: project_collaborator.rs
- dap: dap_types::ModuleId
- deepseek: Request
- editor: Editor
- extension_api: ContextServerSettings
- feature_flags: Context<'_, V>
- gpui: focus_cleanup.rs
- gpui_platform: bare_grid()
- gpui: .fmt()
- gpui: Poll
- json_schema_store: description
- language_models: .api_url()
- line_ending_selector: LineEndingIndicator
- livekit_client: CaptureInput
- lmstudio: ImageUrl
- migrator: m_2025_01_29/settings.rs
- migrator: m_2026_02_25/settings.rs
- migrator: m_2026_04_17/settings.rs
- open_ai: Role
- project: CompletionDocumentation
- proxy_handshake: AsyncRead
- remote: json_log.rs
- repl: ExecutionStatus
- settings_content: GitHostingProviderKind
- settings: editable_setting_control.rs
- sqlez: sqlez/src/util.rs
- util: assertions.rs
- watch: watch/src/error.rs
- zed_actions: ExtensionCategoryFilter
- zlog: EnabledStatus
- AGENTS.md: Blockquote Formatting
- src: Applicable Terms {#applicable-
- src: Automatic prompt
- src: lua.md
- src: Attach debugger to a server ru
- src: Activate on close
- src: Auto Fetch
- src: Codestral Predictions {#edit-p
- src: Admin {#role-admin}
- workflows: CC
- workflows: with
- workflows: env
- workflows: steps
- workflows: defaults
- workflows: if
- workflows: rollout_workflows_to_extension
- workflows: publish_extension_cli.yml
- workflows: orchestrate
- lints: async_block_without_await.rs
- SETTINGS_SYNC.md: SETTINGS_SYNC.md
- gpui_util: BuildHasher
- CLAUDE.md: CLAUDE.md
- acp_thread: PermissionOptionChoice
- agent_servers: .drop()
- agent_ui: Formatter
- ai_onboarding: PlanDefinitions
- collab: project_repository_statuses.rs
- collab: tables/room.rs
- collab: worktree_settings_file.rs
- deepseek: FunctionContent
- deepseek: ImageUrl
- edit_prediction: init_test()
- editor: invisibles.rs
- editor: editor/src/persistence.rs
- gpui: contexts.md
- gpui: gpui/README.md
- gpui: SpringAnimation<T>
- gpui_util: LogErrorFuture<F>
- gpui_windows: check_unicode_separator_cursor
- livekit_client: Participant
- llama_cpp: ImageUrl
- project: DiffKind
- project: Candidates
- project: telemetry_snapshot.rs
- sqlez: Connection
- src: Adding Context
- src: AI Quick Start
- src: Agent Path Boundaries {#agent-
- src: ChatGPT Plus / Pro {#chatgpt}
- src: Appearance
- src: Changing a member's role
- src: Built-in terminal
- src: Capabilities
- src: Client-Side Metrics
- src: Agent Error Messages
- src: Adding Folders to a Project
- glsl: GlslExtension
- DISCUSSION_TEMPLATE: feature-requests.yml
- workflows: permissions
- workflows: BRANCH
- workflows: steps
- workflows: steps
- workflows: GH_TOKEN
- workflows: COMMIT_SHA
- workflows: if
- danger: danger/tsconfig.json
- lints: lints/README.md
- skills: lint-creator/SKILL.md
- settings: gutter
- settings: markdown_preview
- settings: preview_tabs
- settings: search
- gpui_apple: config
- agent_ui: DelayedSelectionAgentServer
- cli: OpenBehavior
- collab: tables/buffer.rs
- collab: tables/notification.rs
- collab: ConnectionPoolGuard
- dap: inline_value.rs
- db: db_path()
- deepseek: Role
- editor: GitBlameEntrySummary
- editor: Editor
- extension_host: dap::AttachRequest
- extension_host: dap::LaunchRequest
- git_ui: .should_serialize()
- gpui_ios: tests/text_input.rs
- gpui: RandomizedChildMutation
- markdown_preview: markdown_preview.rs
- migrator: m_2025_01_30/keymap.rs
- migrator: m_2025_03_03/keymap.rs
- mistral: Role
- open_router: Role
- openai_subscribed: CatalogModel
- sandbox_linux: ALLOCATOR
- settings_content: serde_helper.rs
- settings_profile_selector: active_settings_profile_picker
- settings_ui: a11y_text_position()
- snippet_provider: ListOrDirect
- theme: PlayerColors
- util: marked_text_offsets_by()
- worktree: PathEntrySummary
- zed: PreviewTarget
- zlog: Record
- src: Advanced Configuration/JDTLS i
- src: yaml.md
- src: Button
- src: Build Your Theme
- workflows: if
- workflows: steps
- workflows: cherry_pick.yml
- workflows: jobs
- workflows: CC
- workflows: deploy_docs.yml
- workflows: extension_tests.yml
- workflows: steps
- third-party-terms.md: third-party-terms.md
- settings: file_finder
- settings: minimap
- settings: tabs
- multi_buffer: B1
- collab: ContactId
- acp_thread: SandboxPermission
- agent: Context
- agent_ui: FileLinkTestLogger
- collab: dotenv.rs
- collab: observed_buffer_edits.rs
- collab: tables/user.rs
- edit_prediction_cli: repair.md
- editor: LspDataKind
- editor: MinimapVisibility
- editor: Editor
- extension_api: extension_api/README.md
- json_schema_store: description
- json_schema_store: extendsDefinition
- json_schema_store: description
- json_schema_store: anyOf
- json_schema_store: target
- language_extension: extract_int()
- migrator: m_2025_03_06/keymap.rs
- migrator: m_2025_04_15/keymap.rs
- migrator: m_2025_12_08/keymap.rs
- openai_subscribed: make_fresh_credentials()
- project: Manager
- project: DebouncedDelay<E>
- search: text_finder/render.rs
- terminal: CursorShape
- theme_importer: Appearance
- src: Accessing admin controls
- src: diagnostics.md
- src: Build and Debug Binary
- src: Inline Schema Specification
- src: kotlin.md
- src: proto.md
- src: Configuration
- src: Current Line Highlight
- src: Enable keep preview on code na
- src: File Finder
- workflows: steps
- workflows: GIT_AUTHOR_EMAIL
- workflows: description
- workflows: with
- workflows: description
- workflows: if
- workflows: env
- workflows: working-directory
- workflows: orchestrate
- workflows: with
- workflows: message
- workflows: steps
- workflows: RUST_LIB_BACKTRACE
- terms.md: 3.1. Price
- uninstall.sh: uninstall.sh
- crates_io_client: CratesIoError
- crates_io_client: parse_owner_additions()
- ROADMAP.md: ROADMAP.md
- settings: toolbar
- http_client_tls: configverifierext
- acp_thread: .fmt()
- acp_thread: Formatter
- buffer_diff: HunkStatus
- collab: collab/README.md
- collab: contributor.rs
- collab: JoinChannelInternalResponse
- context_server: BearerError
- context_server: Completion
- dap: dap_types::Checksum
- dap: dap_types::CompletionItem
- editor: ActiveDiagnostic
- editor: .fmt()
- editor: .report_editor_event()
- editor: .deref()
- extension_host: lsp::InsertTextFormat
- extension_host: lsp::Symbol
- extension_host: lsp::SymbolKind
- extension_host: DebugAdapterBinary
- feature_flags: DemoFlag
- fuzzy_nucleo: Case
- gpui: RingOperation
- gpui: seeds.rs
- gpui_web: gallery.js
- gpui_web: http_client_test.rs
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: anyOf
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: default
- json_schema_store: reactNamespace
- json_schema_store: removeComments
- json_schema_store: rewriteRelativeImportExtension
- json_schema_store: skipDefaultLibCheck
- json_schema_store: skipLibCheck
- json_schema_store: sourceMap
- json_schema_store: suppressExcessPropertyErrors
- json_schema_store: suppressImplicitAnyIndexErrors
- json_schema_store: tsBuildInfoFile
- json_schema_store: useUnknownInCatchVariables
- lmstudio: Model
- migrator: m_2025_01_30/settings.rs
- migrator: m_2025_12_15/settings.rs
- proto: buf.yaml
- proto: Item
- settings_content: ExtensionCapabilityContent
- settings_content: flatten_collision_probe.rs
- sqlez: Connection
- sum_tree: Cursor<'_, '_, T, D>
- terminal_view: print256color.sh
- ui: AnimationDuration
- watch: Changed<'_, T>
- watch: Changed<'_, T>
- x_ai_subscribed: refresh_token()
- zed: DisplayActions
- zed: remote_debug.rs
- AGENTS.md: Code and Settings
- src: Agent Path Boundaries {#agent-
- src: Instruction File Support {#sup
- src: AI Data Retained by Zed {#ai-d
- src: Authenticate with Zed
- src: ansible.md
- src: Diagnostics
- src: Find All References
- src: Agent Panel {#agent-panel}
- src: Completions
- actionlint.yml: actionlint.yml
- ISSUE_TEMPLATE: config.yml
- workflows: next_version
- workflows: defaults
- workflows: RUST_LIB_BACKTRACE
- workflows: continue-on-error
- workflows: if
- workflows: on
- terms.md: 13.1. Definition
- compliance: ParsedOutput
- install.sh: install.sh
- lib: deploy-helpers.sh
- crates_io_client: simulated_crates_io_client.rs
- settings: tab_bar
- themes: ayu.json
- themes: gruvbox.json
- themes: one.json
- project: CacheInlayHintsTask
- acp_thread: dyn AgentConnection
- agent: AskUserToolOutput
- agent: CreateThreadToolOutput
- agent: Advanced Usage
- agent: Claude Desktop Integration
- agent: FindPathToolOutput
- collab: buffer_operation.rs
- collab: buffer_snapshot.rs
- collab: tables/extension.rs
- collab: language_server.rs
- collab: project_repository.rs
- collab: tables/worktree.rs
- dap: DapStatus
- edit_prediction_cli: qa.md
- editor: CompletionMenuScrollBarSetting
- editor: ActionFetchReady
- editor: ContextMenuOptions
- extension_host: dap::BuildTaskDefinition
- extension_host: dap::StartDebuggingRequestArgu
- extension_host: dap::StartDebuggingRequestArgu
- extension_host: dap::TaskTemplate
- extension_host: dap::BuildTaskDefinition
- extension_host: dap::IpAddress
- google_ai: Task
- gpui: GlobalLease<G>
- gpui: ArenaBox<T>
- gpui_windows: HiLoWord
- icons: icons/README.md
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: description
- json_schema_store: rootDir
- json_schema_store: sourceRoot
- json_schema_store: stripInternal
- language_models_cloud: AnthropicModelMode
- migrator: m_2025_11_25/settings.rs
- project_panel: cmp_worktree_entries()
- project_panel: ReversibleIterable<It>
- proto: proto/src/macros.rs
- search: InputPanel
- sum_tree: Arbitrary
- terminal_view: terminal_view/README.md
- text: Edit<D>
- ui: .spacing()
- AGENTS.md: Bad: Defensive or dismissive
- AGENTS.md: Bad: Defensive or dismissive
- src: Context Windows {#context-wind
- src: Anthropic-Compatible Providers
- src: Agent Paths {#agent-paths}
- src: Additional admin controls
- src: Configuring Icon Themes
- src: ocaml.md
- src: Dark
- src: Dark
- src: Editor Tab Bar
- src: Fast Scroll Sensitivity
- src: Terminal: Button
- src: update.md
- theme: page-toc.js
- workflows: needs
- workflows: R2_ACCESS_KEY_ID
- workflows: if
- workflows: if
- workflows: if
- workflows: run_tests_windows
- terms.md: 2.1. Eligibility
- terms.md: 4.1. Zed's Use of Customer Dat
- terms.md: 9.1. Term
- lints: MaybeTyCtxt
- extension_api: zed_extension_api
- lib: blob-store.sh script
- lints: gpui_shared_string/src/lib.rs
- settings: collaboration_panel
- settings: telemetry
- settings: theme
- action_log: TrackedBufferStatus
- agent: TrackedBufferStatus
- agent: RasterizationOptions
- agent: RasterizationOptions
- agent: Long Requests
- agent_ui: undo_reject_toast.rs
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: notification_kind.rs
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: Entity
- collab: tables/server.rs
- collab: Entity
- collab: worktree_diagnostic_summary.rs
- collab: Entity
- collab: worktree_entry.rs
- dap: IoKind
- deepseek: ReasoningEffort
- deepseek: Thinking
- edit_prediction_cli: codex-acp--add-derive.md
- edit_prediction_cli: flask--add-and-rename-test-fun
- edit_prediction_cli: flask--add-import-statement.md
- edit_prediction_cli: flask--add-test-function.md
- edit_prediction_cli: flask--rename-accepted-predict
- edit_prediction_cli: hello-world--rename-accepted-g
- edit_prediction_cli: terraform--add-comment.md
- edit_prediction_cli: tree-sitter--if-let-to-match.m
- edit_prediction_cli: tree-sitter--tuple-to-struct-d
- edit_prediction_cli: tree-sitter--tuple-to-struct-d
- edit_prediction_cli: tree-sitter--tuple-to-struct-f
- edit_prediction_cli: tree-sitter--tuple-to-struct-f
- edit_prediction_cli: tree-sitter--tuple-to-struct-l
- edit_prediction_cli: vscode--add-async-and-await.md
- edit_prediction_cli: vscode--add-class-decorator.md
- edit_prediction_cli: vscode--add-interface-method.m
- edit_prediction_cli: vscode--log-object-property.md
- edit_prediction_cli: zed--add-eprintln.md
- edit_prediction_cli: zed--change-match-arm.md
- edit_prediction_types: EditPredictionGranularity
- editor: BreakpointPromptEditAction
- editor: SelectSyntaxNodeScrollBehavior
- extension_api: PENDING_CHANGES.md
- extension_host: BuildTaskTemplate
- extension_host: DebugRequest
- feature_flags: AgentThreadWorktreeLabel
- fs: Backend
- language_core: Override<T>
- proto: Nonce
- proto: SystemTime
- rpc: rpc/src/macros.rs
- search: Event
- search: ViewEvent
- shell_command_parser: extract_commands_from_extended
- sidebar: SerializedSidebar
- ui: HoverTarget
- workspace: Box<dyn WeakSearchableItemHand
- AGENTS.md: Behavioral Guidelines
- AGENTS.md: Keymap File Locations
- AGENTS.md: Page Structure Patterns
- src: Audio Settings {#audio-setting
- src: Editing Code
- src: Anthropic Retention {#telemetr
- src: Example configuration
- src: Action Reference
- src: Adding toolchains manually
- src: Configuring broad worktree tru
- EXTRACTION.md: EXTRACTION.md
- README.md: extensions/README.md
- workflows: if
- workflows: if
- workflows: description
- workflows: pr_number
- workflows: name
- workflows: needs
- workflows: needs
- workflows: if
- workflows: if
- workflows: if
- workflows: if
- workflows: if
- workflows: if
- terms.md: 11.1.
- terms.md: 15.1. Governing Law, Jurisdict
- tests: sandboxing/README.md
- BRAND.md: BRAND.md
- terminal: AlacDirection
- terminal: AlacSelectionType
- terminal: AlacViMotion
- settings: active_pane_modifiers
- settings: audio
- settings: calls
- settings: centered_layout
- settings: drag_and_drop_selection
- settings: focus_follows_mouse
- docs-proxy: docs-proxy/src/worker.js
- README.md: .cloudflare/README.md
- action_log: UndoBufferStatus
- agent_servers: dyn AgentServer
- agent: Creating a batch
- agent: Echo Server
- cli: cli/README.md
- collections: Iter<'a, K, V>
- dap: dap_types::SourcePresentationH
- dap: dap_types::StackFrame
- dap: dap_types::SteppingGranularity
- dap: dap_types::Variable
- dev_container: ComposeUpBehavior
- editor: FoldStatus
- eval_utils: EvalOutput<M>
- extension_host: latest::Command
- extension_host: latest::Command
- extension_host: latest::CodeLabel
- extension_host: latest::CodeLabelSpan
- extension_host: latest::CodeLabelSpanLiteral
- extension_host: latest::Command
- extension_host: latest::CodeLabel
- extension_host: latest::CodeLabelSpan
- extension_host: latest::CodeLabelSpanLiteral
- extension_host: latest::Command
- extension_host: latest::CodeLabel
- extension_host: latest::CodeLabelSpan
- extension_host: latest::CodeLabelSpanLiteral
- extension_host: latest::Command
- extension_host: latest::CodeLabel
- extension_host: latest::CodeLabelSpan
- extension_host: latest::CodeLabelSpanLiteral
- extension_host: latest::CodeLabel
- extension_host: latest::CodeLabelSpan
- extension_host: latest::CodeLabelSpanLiteral
- extension_host: latest::CodeLabel
- extension_host: latest::CodeLabelSpan
- extension_host: latest::CodeLabelSpanLiteral
- extension_host: github::GithubReleaseAsset
- extension_host: latest::CodeLabel
- extension_host: latest::CodeLabelSpan
- extension_host: latest::CodeLabelSpanLiteral
- extension_host: latest::DebugConfig
- extension_host: latest::DebugRequest
- extension_host: latest::DebugScenario
- extension_host: latest::DebugTaskDefinition
- extension_host: lsp::Completion
- extension_host: extension::CodeLabel
- extension_host: extension::CodeLabelSpan
- extension_host: extension::CodeLabelSpanLitera
- extension_host: extension::ContextServerConfig
- extension_host: extension::DebugScenario
- extension_host: extension::SlashCommandArgumen
- extension_host: extension::SlashCommandOutput
- extension_host: extension::StartDebuggingReque
- extension_host: extension::StartDebuggingReque
- extension_host: github::GithubReleaseAsset
- feature_flags: Intensity
- gpui: gpui/build.rs
- gpui: gpui/src/bench.rs
- gpui: Formatter
- gpui_util: DebugAsDisplay<'_, E>
- gpui_util: Deferred<F>
- outline_panel: CachedEntriesUpdate
- outline_panel: ItemsDisplayMode
- project_panel: project_panel/src/utils.rs
- project_panel: ReversibleIterable<T>
- project: Notification
- sandbox_linux: MatchingComponents<'a>
- schema_generator: schema_generator/README.md
- ui: SubmenuOpenTrigger
- vim: vim/README.md
- watch: Target
- workspace: init()
- zed: BuildTiming
- ztracing_macro: ztracing_macro/src/lib.rs
- AGENTS.md: Core Principles
- AGENTS.md: Must
- AGENTS.md: Output Format
- src: Agent Path vs. LLM Provider {#
- src: Migrating to Skills {#migratin
- src: How It Differs from Other Agen
- src: business-support.md
- src: Getting started
- src: Adding a Contact
- src: Completions
- src: release-notes.md
- src: opentofu.md
- src: Active Pane Modifiers
- src: Agent
- src: Debounce
- src: Diagnostics
- src: Hour Format
- src: Quick start
- workflows: defaults
- workflows: DIGITALOCEAN_SPACES_ACCESS_KEY
- lints: LintStore
- CLAUDE.md: .claude/CLAUDE.md
- open-source-website-assets: open-source-website-assets/src
- CODE_OF_CONDUCT.md: CODE_OF_CONDUCT.md
- agent: Contributing
- agent: Determining the installed vers
- agent: Streaming Helpers
- agent_ui_benchmarks: agent_ui_benchmarks/README.md
- agent_ui: plan_summary_label()
- collab: production.sh
- collab: staging.sh
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- collab: ActiveModel
- dap: breakpoints.md
- db: db/README.md
- eval_utils: eval_utils/README.md
- extension_api: wit::Range
- extension_host: latest::Range
- extension_host: Completion
- extension_host: latest::Range
- extension_host: latest::Range
- extension_host: DebugConfig
- extension_host: DebugRequest
- extension_host: DebugScenario
- extension_host: DebugTaskDefinition
- extension_host: Completion
- extension_host: DebugScenario
- extension_host: DebugTaskDefinition
- extension_host: SlashCommand
- extension_host: std::ops::Range<usize>
- gpui: key_dispatch.md
- gpui_ios: build-simulator.sh
- livekit_api: protocol/README.md
- outline_panel: Event
- proto: LspQuery
- proto: Receipt<T>
- sandbox: bind_source_toctou_test.sh
- terminal_view: truecolor.sh
- theme_importer: theme_importer/README.md
- vim: default.rs
- workspace: Box<dyn SearchableItemHandle>
- zed: zed.sh
- zlog: zlog/README.md
- src: src/quick-start.md
- src: Global LSP Settings
- src: default-key-bindings.md
- test-extension: test-extension/README.md
- zizmor.yml: zizmor.yml
- compliance: compliance
- digital-ocean-db.sh: digital-ocean-db.sh
- lib: bump-version.sh
- mitm-proxy.sh: mitm-proxy.sh
- compliance: Repository<'static>
- brand: export-icons.sh
- gpui_ios: SceneConnectionOptions
- gpui_ios: SFSafariViewController
- gpui_shared_string: gpui_shared_string
- agent_ui_benchmarks: agent_ui_benchmarks
- auto_update_helper: auto_update_helper
- benchmarks: benchmarks
- lints: consumer
- explorer_command_injector: explorer_command_injector
- lints: lints
- lints: render_consumer
- corgi: scratch
- update_top_ranking_issues: update-top-ranking-issues
- windows_resources: windows_resources
- eval_cli: zed-eval
- lints: gpui
- lints: gpui_shared_string

## God Nodes (most connected - your core abstractions)
1. `TestAppContext` - 6601 edges
2. `Task` - 2487 edges
3. `Pixels` - 1598 edges
4. `SharedString` - 1555 edges
5. `Workspace` - 1503 edges
6. `px()` - 1382 edges
7. `AsyncApp` - 1302 edges
8. `Project` - 1282 edges
9. `Editor` - 1241 edges
10. `Editor` - 723 edges

## Surprising Connections (you probably didn't know these)
- `Code Lens` --references--> `Benchmark`  [INFERRED]
  docs/src/languages/go.md → crates/eval_cli/zed_eval/benchmarks.py
- `Customizing Semantic Token Styles` --references--> `background_color()`  [INFERRED]
  docs/src/extensions/languages.md → crates/gpui_apple/src/shaders.metal
- `Semantic Token Rules {#global-lsp-semantic-token-rules}` --references--> `background_color()`  [INFERRED]
  docs/src/reference/all-settings.md → crates/gpui_apple/src/shaders.metal
- `Rule Structure` --references--> `background_color()`  [INFERRED]
  docs/src/semantic-tokens.md → crates/gpui_apple/src/shaders.metal
- `run_gpui_eval()` --references--> `outcome`  [INFERRED]
  crates/agent/src/tools/evals.rs → .github/workflows/release.yml

## Import Cycles
- 1-file cycle: `crates/git_ui/src/project_diff.rs -> crates/git_ui/src/project_diff.rs`
- 2-file cycle: `crates/git_ui/src/project_diff.rs -> crates/git_ui/src/staged_diff.rs -> crates/git_ui/src/project_diff.rs`
- 2-file cycle: `tooling/xtask/src/tasks/workflows/release.rs -> tooling/xtask/src/tasks/workflows/run_bundling.rs -> tooling/xtask/src/tasks/workflows/release.rs`
- 2-file cycle: `crates/language_model_core/src/language_model_core.rs -> crates/language_model_core/src/rate_limiter.rs -> crates/language_model_core/src/language_model_core.rs`
- 2-file cycle: `crates/agent/src/agent.rs -> crates/agent/src/native_agent_server.rs -> crates/agent/src/agent.rs`
- 2-file cycle: `crates/agent/src/agent.rs -> crates/agent/src/thread.rs -> crates/agent/src/agent.rs`
- 2-file cycle: `crates/agent/src/tools.rs -> crates/agent/src/tools/read_file_tool.rs -> crates/agent/src/tools.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/element.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/element/header.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/git/blame.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/hover_links.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/hover_popover.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/indent_guides.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/inlays.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/inlays/inlay_hints.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/items.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/linked_editing_ranges.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/lsp_ext.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/mouse_context_menu.rs -> crates/editor/src/editor.rs`
- 2-file cycle: `crates/editor/src/editor.rs -> crates/editor/src/movement.rs -> crates/editor/src/editor.rs`

## Communities (2226 total, 449 thin omitted)

### Community 0 - "UI Actions & Workspace Glue"
Cohesion: 0.00
Nodes (622): DEFERRED_SCAN_MESSAGE_TIMEOUT, GIT_OPERATION_DELAY, MAX_MESSAGE_LEN, AgentSettings, AgentModelSelector, ManageProfiles, ModelUsageContext, InlineAssistant (+614 more)

### Community 1 - "Project & Agent Server Core"
Cohesion: 0.00
Nodes (706): from_v1(), is_supported(), legacy_definitions_preserve_authentication_data(), from_v1(), legacy_definitions_preserve_input_and_metadata(), from_v1(), legacy_categories_convert_to_v2(), legacy_options_preserve_fields_in_v2() (+698 more)

### Community 2 - "LLM Provider Integrations"
Cohesion: 0.00
Nodes (634): BatchIndividualResponse, BatchRequest, create_batch(), CreateBatchRequest, MessageBatch, MessageBatchRequestCounts, retrieve_batch(), retrieve_batch_results() (+626 more)

### Community 3 - "Editor Commands & Context"
Cohesion: 0.01
Nodes (1020): default_settings_for_agent(), init_registry_with_agents(), init_test(), set_agent_server_settings(), test_agent_in_registry_store_is_registry(), test_agent_with_registry_settings_type_is_registry(), test_unknown_agent_is_not_registry(), test_following_stops_on_unshare() (+1012 more)

### Community 4 - "Multi-Buffer & Diff Hunks"
Cohesion: 0.01
Nodes (331): AUTO_OUTLINE_SIZE, BufferContent, get_buffer_content_or_outline(), render_entries(), render_outline(), test_large_file_fallback_to_subset(), apply_non_conflicting_edits(), diff_snapshots() (+323 more)

### Community 5 - "UI Notices & Components"
Cohesion: 0.01
Nodes (239): preview_notice(), session_notice_list(), SessionNotice, SessionNoticePreview, quit(), ComponentExample, ComponentExampleGroup, empty_example() (+231 more)

### Community 6 - "GPUI Layout & Examples"
Cohesion: 0.01
Nodes (228): resolve_agent_image(), long_single_line_selection_uses_one_geometry_fragment(), main(), PrepaintState, run_example(), start(), TextElement, EditorText (+220 more)

### Community 7 - "Language Servers & Toolchains"
Cohesion: 0.01
Nodes (269): IndexSet, clear_copilot_dir(), download_server_binary(), AssetKind, Gz, TarBz2, TarGz, Zip (+261 more)

### Community 8 - "GPUI Style Primitives"
Cohesion: 0.01
Nodes (304): NotesVersion, TypeIdHashSet, CommandInterceptItem, GlobalCommandPaletteFilter, Args, main(), parse_args(), assert_color_render_modes() (+296 more)

### Community 9 - "GPUI Web & Metal Platform"
Cohesion: 0.00
Nodes (101): BACK_COMMIT_VELOCITY, back_event(), modifiers(), scroll_event(), touch_force(), touch_id(), touch_location_in_view(), touch_phase() (+93 more)

### Community 10 - "Apple Platform FFI"
Cohesion: 0.00
Nodes (96): KillRing, notification_window_options(), main_thread_marker(), main_thread_marker_rejects_background_threads(), StatusBarContentStyle, Dark, Light, current_platform() (+88 more)

### Community 11 - "Editor Element Geometry"
Cohesion: 0.01
Nodes (217): animated_corners_overlap_target(), EditorMargins, StickyHeaderExcerpt, column_pixels(), EditorSnapshot, EditorStyle, FILE_HEADER_HEIGHT, GutterDimensions (+209 more)

### Community 12 - "Wayland Platform Layer"
Cohesion: 0.01
Nodes (155): Backend, Wayland, X11, select_backend(), LinuxKeyboardLayout, cursor_style_to_icon_names(), open_uri_internal(), reveal_path_internal() (+147 more)

### Community 13 - "Editor Text Transform Actions"
Cohesion: 0.01
Nodes (48): make_suggestion_styles(), AccentData, AddSelectionsGroup, AddSelectionsState, BackgroundHighlight, BreadcrumbsVisibility, BufferSerialization, All (+40 more)

### Community 14 - "GPUI Window & App Context"
Cohesion: 0.01
Nodes (136): AnyTooltip, InputPreference, CharacterInput, KeyBindings, E, UnderlineStyle, atomic_incr_if_not_zero(), floor_to_device_pixel() (+128 more)

### Community 15 - "Language Buffer Core"
Cohesion: 0.01
Nodes (240): ServerStatus, build_diff_options(), apply_related_actions(), is_c_language(), switch_source_header(), find_specific_language_server_in_selection(), lsp_task_context(), lsp_tasks() (+232 more)

### Community 16 - "GPUI Scroll & Input Ops"
Cohesion: 0.01
Nodes (193): COLUMNAR_SELECTION_MODIFIERS, Example, MousePressureExample, button(), HelloWorld, line(), main(), popover() (+185 more)

### Community 17 - "Text Shaping & CoreText"
Cohesion: 0.01
Nodes (205): AppleTextSystem, AppleTextSystemState, bounds_from_rect(), bounds_from_rect_i(), family_name(), font_kit_metrics_to_metrics(), font_smoothing_allowed_by_user(), FontKey (+197 more)

### Community 18 - "Tasks & JSON Schemas"
Cohesion: 0.01
Nodes (230): build_terminal_auth_task(), ensure_dap_interface(), GdbDebugAdapter, .ADAPTER_NAME, DebugAdapter, DebugAdapterName, FakeAdapter, .ADAPTER_NAME (+222 more)

### Community 19 - "Git Proto Messages"
Cohesion: 0.01
Nodes (86): ProjectId, append_pattern_to_ignore_file(), AutoFetchState, branch_to_proto(), BufferGitState, CommitDataState, Loaded, Loading (+78 more)

### Community 20 - "LSP Proto Requests"
Cohesion: 0.01
Nodes (86): Global, DiagnosticEntry<PointUtf16>, Location, range_to_lsp(), symbol_kind_to_lsp(), deserialize_anchor_range(), deserialize_transaction(), deserialize_version() (+78 more)

### Community 21 - "Shared Settings Types"
Cohesion: 0.01
Nodes (195): buffer_line_height, icon_theme, AgentModelGroupName, StubAgentSessionEditor, StubAgentSessionSetTitle, Event, ShowStatus, empty_file_is_ignored() (+187 more)

### Community 22 - "Workspace Actions"
Cohesion: 0.02
Nodes (89): open_disabled_globs_setting_in_editor(), PathPromptOptions, FocusFollowsMouse, T, add_saved_rows(), read_default_window_bounds(), ActivateInDirectionTarget, Dock (+81 more)

### Community 23 - "Markdown Rendering"
Cohesion: 0.01
Nodes (226): HelloWorld, Context, apply_html_highlight_style(), ensure_theme_initialized(), html_list_item_prefix(), HtmlSourceAllocator, MarkdownElement, render_markdown_text() (+218 more)

### Community 24 - "Remote Collaboration Auth"
Cohesion: 0.01
Nodes (93): EditSessionTarget, project_path_for_skill(), Collaborator, Event, Dismissed, Selected, ImgState, project_scan_indicator() (+85 more)

### Community 26 - "Code Action Requests"
Cohesion: 0.02
Nodes (88): PendingWork, CachedLspAdapter, ServerBinaryCache, LanguageServerSelector, Id, Name, ProjectTransaction, applicable_lsp_command_capabilities_for_buffer() (+80 more)

### Community 27 - "Worktree Scanning"
Cohesion: 0.01
Nodes (109): Watcher, worktree_to_proto(), PathMatchCandidateSetIter, PathMatchCandidateSetNucleoIter, BackgroundScanner, BackgroundScannerPhase, Events, EventsReceivedDuringInitialScan (+101 more)

### Community 28 - "Screen Capture & Wgpu"
Cohesion: 0.01
Nodes (157): ScrollbarMarkerState, ScrollbarMarkerState, SettledScrollRange, AtlasUpload, AtlasUpload<'a>, create_atlas(), insert_tile(), make_image_key() (+149 more)

### Community 29 - "GPUI Lists & Minimap"
Cohesion: 0.02
Nodes (244): main(), test_minimap_thumb_not_draggable_without_room_to_move(), resolved_position(), GradientViewer, main(), run_example(), start(), main() (+236 more)

### Community 30 - "Agent Settings & Compaction"
Cohesion: 0.02
Nodes (169): ClientUserMessageId, acp_v2::ContentBlock, agent_text_message(), AgentMessage, AgentMessageContent, RedactedThinking, Text, Thinking (+161 more)

### Community 31 - "X11 Platform Layer"
Cohesion: 0.01
Nodes (137): DEFAULT_CURSOR_ICON_NAME, log_cursor_icon_warning(), Window, Cursor, LoadedTheme, ResizeEdge, ResizeEdgeWaylandExt, capslock_toggle_produces_uppercase() (+129 more)

### Community 32 - "Language Completions & Deploy"
Cohesion: 0.01
Nodes (176): Choice, codestral_api_key(), CODESTRAL_API_KEY_ENV_VAR, codestral_api_key_state(), CODESTRAL_API_URL, codestral_api_url(), CodestralEditPredictionDelegate, CodestralRequest (+168 more)

### Community 33 - "Pane & Navigation History"
Cohesion: 0.02
Nodes (182): ProjectItemKind, ActivateItem, ActivateNextItem, ActivatePreviousItem, ActivationHistoryEntry, add_labeled_item(), arbitrary_path(), assert_item_labels() (+174 more)

### Community 34 - "Collab & Debugger Tests"
Cohesion: 0.01
Nodes (294): search(), test_model_search_builds_candidates_in_background(), ParticipantIndex, assert_active_item_is_screen_share_for_peer(), assert_no_screen_share_tabs_exist(), AutoWatchTestSetup, setup_auto_watch_late_joiner_test(), setup_auto_watch_test() (+286 more)

### Community 35 - "ACP Thread Content"
Cohesion: 0.01
Nodes (164): EntryUpdated, AgentLocation, AgentThreadEntry, AssistantMessage, ContextCompaction, Elicitation, SessionMessage, ToolCall (+156 more)

### Community 36 - "Git Repository Backend"
Cohesion: 0.02
Nodes (160): AskPassDelegate, PasswordPrompt, Blame, run_git_blame(), CommitDiffObjectKind, Blob, Gitlink, RunHook (+152 more)

### Community 37 - "Vim Motions & Display Points"
Cohesion: 0.02
Nodes (236): Anchor, assert_headers_resolve(), DisplayPoint, DisplaySnapshot, Point, ColumnarSelectionState, FromMouse, FromSelection (+228 more)

### Community 38 - "Git Hosting Providers"
Cohesion: 0.01
Nodes (174): GitBlameBuffer, parse_commit_details(), get_host_from_git_remote_url(), register_additional_providers(), test_get_host_from_git_remote_url(), Azure, pull_request_regex(), test_azure_extract_pull_request() (+166 more)

### Community 39 - "ACP Thread Messages"
Cohesion: 0.03
Nodes (246): TokenUsageUpdated, append_keyed_chunk(), assert_failed_tool_update_resumes_sleep_prevention(), assert_process_terminal_can_stop(), assert_stale_completion_does_not_affect_follow_up_turn(), AuthorizationKind, ActionChoice, PermissionGrant (+238 more)

### Community 40 - "Color Contrast & Theming"
Cohesion: 0.01
Nodes (128): adjacent_distance(), adjust_color_for_background(), adjusted_lightness_candidate(), background_contrast(), bracket_colorization_accents(), compare_candidates(), dark_editor_background(), light_editor_background() (+120 more)

### Community 41 - "Agent Sidebar UI"
Cohesion: 0.02
Nodes (95): DiffStats, import_threads_from_other_channels(), format_history_entry_timestamp(), RemoteBranchName, test_worktree_create_targets(), worktree_create_targets(), WorktreeCreateTarget, CurrentBranch (+87 more)

### Community 42 - "Agent Skills & Terminal"
Cohesion: 0.02
Nodes (163): global_skills_dir(), AcpTerminalHandle, AGENTS_PREFIX, ambiguous_mcp_prompt_names(), apply_skill_overrides(), assert_failed_native_command_context(), available_native_agent(), Command (+155 more)

### Community 43 - "Agent Tools & Extension API"
Cohesion: 0.01
Nodes (153): AgentTool, .NAME, ApplyCodeActionTool, .NAME, ApplyCodeActionToolInput, accept_with(), AskUserTool, .NAME (+145 more)

### Community 44 - "Mouse & Pane Hit Testing"
Cohesion: 0.01
Nodes (72): convert_mouse_position(), TestAppWindow<V>, Along, auto(), bounds(), Bounds<T>, Corners<T>, Edges<T> (+64 more)

### Community 45 - "Remote Server Protocol"
Cohesion: 0.01
Nodes (265): test_channel_guest_promotion(), test_channel_guests(), test_channel_requires_zed_cla(), blame_entry(), extract_color_inlays(), extract_hint_labels(), extract_semantic_token_ranges(), GuestSemanticTokensTestConfig (+257 more)

### Community 46 - "Agent Conversation View"
Cohesion: 0.03
Nodes (238): active_thread(), add_to_workspace(), add_to_workspace_with_size(), assert_auth_required_on_initial_connect(), assert_thread_list_item_count_matches_entries(), assert_unsupported_source_message_cannot_regenerate(), AuthErrorSource, Marker (+230 more)

### Community 47 - "Project Panel"
Cohesion: 0.02
Nodes (56): Color, entry_diagnostic_aware_icon_decoration_and_color(), entry_diagnostic_aware_icon_name_and_color(), transparent_white(), ClipboardEntry, Copied, Cut, ContextMenuPlacement (+48 more)

### Community 48 - "Agent Panel"
Cohesion: 0.02
Nodes (43): agent_panel_dock_position(), AgentPanel, AgentPanelTerminalInfo, AgentTerminal, AgentThread, apply_native_model_override(), assert_loading_selection_removal(), BaseView (+35 more)

### Community 49 - "Icon Names"
Cohesion: 0.01
Nodes (300): IconName, AcpRegistry, AiAnthropic, AiAnthropicCompat, AiBedrock, AiClaude, AiDeepSeek, AiEdit (+292 more)

### Community 50 - "Debug Adapter Sessions"
Cohesion: 0.02
Nodes (55): SessionId, DebugAdapterState, ProjectState, RpcMessages, .MESSAGE_QUEUE_LIMIT, Arc<T>, Attach, ConfigurationDone (+47 more)

### Community 51 - "Item Handles & Following"
Cohesion: 0.02
Nodes (28): AnyView, Box<dyn ItemHandle>, Dedup, KeepExisting, ReplaceExisting, dyn ItemHandle, Entity<T>, FollowableItem (+20 more)

### Community 52 - "Sidebar Tests"
Cohesion: 0.03
Nodes (247): TestTerminalMetadataDbName, active_session_id(), open_thread_with_connection(), VisualTestContext, add_agent_panel(), add_test_project(), assert_active_draft(), assert_active_thread() (+239 more)

### Community 53 - "Git Commit UI"
Cohesion: 0.02
Nodes (115): commit_context_menu(), COMMIT_TAG_LIST_WIDTH_IN_REMS, CommitContextMenuData, CommitContextMenuSource, GitGraph, GitPanel, CUSTOM_GIT_COMMANDS_DOCS_SLUG, git_context_menu_tasks() (+107 more)

### Community 54 - "Completion Menu"
Cohesion: 0.01
Nodes (140): ConfirmCompletion, SpawnNearestTask, AvailableCodeAction, bold(), CODE_ACTION_MENU_MAX_WIDTH, CODE_ACTION_MENU_MIN_WIDTH, CodeActionContents, colored() (+132 more)

### Community 55 - "ACP Thread Lifecycle"
Cohesion: 0.02
Nodes (91): AcpThread, AcpThreadEvent, ActivityConfirmedIdle, AvailableCommandsUpdated, ConfigOptionsUpdated, ConnectionLost, ElicitationRequested, ElicitationResponded (+83 more)

### Community 56 - "Project Git Tests"
Cohesion: 0.02
Nodes (243): assert_hunks(), assert_entry_git_state(), assert_line_endings_after_format(), chunks_with_diagnostics(), code_action_project(), code_action_project_with(), diagnostics_pull_project(), DiagnosticsPullServer (+235 more)

### Community 57 - "App Entry & Actions"
Cohesion: 0.02
Nodes (192): test_markdown_inline_html_highlighting(), markdown_lang(), local_tasks_file_relative_path(), bool, SaturatingBool, initial_project_settings_content(), MarkdownString, accessibility_tree_dump() (+184 more)

### Community 58 - "Workspace Persistence"
Cohesion: 0.02
Nodes (148): ComponentPreviewDb, .MIGRATIONS, .NAME, EditorDb, .MIGRATIONS, .NAME, test_save_and_get_file_folds(), test_save_and_get_serialized_editor() (+140 more)

### Community 59 - "Agent Permission Requests"
Cohesion: 0.02
Nodes (38): PermissionRequest, PermissionRequestId, AcpServerViewEvent, ActiveThreadChanged, AgentCodeSpanResolver, AgentCodeSpanResolverInner, AuthGatedAgentConnection, .AUTH_METHOD_ID (+30 more)

### Community 60 - "Extension Code Labels"
Cohesion: 0.06
Nodes (256): CodeLabelSpan, CodeRange, Literal, CodeLabelSpanLiteral, gpui, acp_thread, acp_tools, action_log (+248 more)

### Community 61 - "Workspace & Audio Settings"
Cohesion: 0.01
Nodes (100): AudioSettings, AutoUpdateSetting, ClientSettings, ProxySettings, TelemetrySettings, CollaborationPanelSettings, CommandPaletteSettings, dap_granularity_from_settings() (+92 more)

### Community 62 - "GPUI App Actions"
Cohesion: 0.02
Nodes (36): App, app_borrow_diagnostics(), ApplicationHandle, AppLifecycleHandler, AppRef, AppRefMut, async_app_refresh_flushes_refresh_effect(), CursorHideMode (+28 more)

### Community 63 - "GPUI Styled Methods"
Cohesion: 0.02
Nodes (81): AlignContent, Center, End, FlexEnd, FlexStart, SpaceAround, SpaceBetween, SpaceEvenly (+73 more)

### Community 64 - "Multi-Workspace Windows"
Cohesion: 0.03
Nodes (186): WindowHandle, ProjectHost, Local, Remote, HANDLE_HITBOX_SIZE, activate_any_workspace_window(), ActivatePane, add_an_item_to_active_pane() (+178 more)

### Community 65 - "OpenAI Completion Events"
Cohesion: 0.02
Nodes (202): LanguageModelCompletionEvent, Compaction, InputTransformations, Queued, ReasoningDetails, RedactedThinking, Started, StartMessage (+194 more)

### Community 66 - "GPUI Frame Profiler"
Cohesion: 0.03
Nodes (152): excludes_input_that_arrives_during_a_draw(), interruptions_drop_latency_samples_but_hiding_keeps_draw_work(), a_pending_frame_prevents_idle_until_presentation(), a_poll_spanning_an_unnotified_suspend_is_dropped(), a_present_without_a_draw_clears_the_window_pending_state(), a_presentation_after_the_deadline_seals_the_whole_interval(), advance_test_suspended_time(), an_expired_frame_no_longer_blocks_idle_boundaries() (+144 more)

### Community 67 - "GPU Atlas & Surfaces"
Cohesion: 0.02
Nodes (116): SurfaceBounds, build_scene(), Case, cases(), insert_primitives(), Kind, Path, Quad (+108 more)

### Community 68 - "AWS Bedrock Provider"
Cohesion: 0.02
Nodes (122): AwsHttpClient, MantleModel, Custom, Gpt5_4, Gpt5_5, Gpt5_6Luna, Gpt5_6Sol, Gpt5_6Terra (+114 more)

### Community 69 - "Edit Prediction Core"
Cohesion: 0.02
Nodes (91): RelatedExcerptStoreEvent, FinishedRefresh, StartedRefresh, capture_prediction_context(), CapturedPredictionContext, compute_uncommitted_diff(), exclusive_end_row(), MAX_UNCOMMITTED_DIFF_SIZE (+83 more)

### Community 70 - "project: ContinueResponse"
Cohesion: 0.02
Nodes (51): VariableReference, DataBreakpointContext, Address, Expression, Variable, CacheableCommand, client_source(), CompletionsQuery (+43 more)

### Community 71 - "edit_prediction_cli: AnthropicClient"
Cohesion: 0.02
Nodes (184): AnthropicClient, Dummy, Plain, run_distill(), ActualCursor, deserialize_null_as_empty_string(), Example, ExamplePrediction (+176 more)

### Community 72 - "agent_ui: AvailableCommandInput"
Cohesion: 0.03
Nodes (101): AvailableCommand, load_external_image_from_path(), build_chunks_from_creases(), COMMAND_HINT_INLAY_ID, command_input(), current_completion_labels(), current_completion_labels_with_documentation(), find_matching_bracket() (+93 more)

### Community 73 - "language: ClearHistory"
Cohesion: 0.02
Nodes (115): test_ssh_restarting_language_server_replaces_remote_status(), clean_fim_completion(), FIM_CONTEXT_TOKENS, FIM_PROMPT_FORMATS, fim_stop_tokens(), fim_stop_tokens_match_prompt_markers(), fim_stop_tokens_respect_openai_stop_limit(), FimRequestOutput (+107 more)

### Community 74 - "lsp: run_work_cycle()"
Cohesion: 0.02
Nodes (74): run_work_cycle(), LspStdoutHandler, test_backpressure_when_messages_are_not_consumed(), deserialize_params(), deserialize_result(), FakeLanguageServer, framed_test_message(), i64 (+66 more)

### Community 75 - "agent: AuthorizationKind"
Cohesion: 0.02
Nodes (67): CancellationAwareTool, .NAME, CancellationAwareToolInput, DelayTool, .NAME, DelayToolInput, EchoTool, .NAME (+59 more)

### Community 76 - "vim: register()"
Cohesion: 0.02
Nodes (105): register(), register(), register(), register(), test_enter_and_exit_insert_mode(), test_insert_ctrl_r(), test_insert_ctrl_y(), test_insert_with_counts() (+97 more)

### Community 77 - "git_ui: Close"
Cohesion: 0.03
Nodes (9): GitCommitTemplate, git_panel_context_menu(), git_panel_view_options_menu(), GitPanel, panel_editor_container(), prompt(), test_directory_discard_tracked_changes(), test_stage_section_scopes_to_selected_section() (+1 more)

### Community 78 - "editor: parse_zed_link()"
Cohesion: 0.02
Nodes (58): parse_zed_link(), ZedLink, Channel, ChannelNotes, AddSelectionAbove, AddSelectionBelow, CancelEditReviewComment, ComposeCompletion (+50 more)

### Community 79 - "agent: Addon"
Cohesion: 0.01
Nodes (19): Addon, AutocloseRegion, BackgroundHighlight, ContextMenuOptions, Editor, .EDIT_PREDICTION_POPOVER_PADDING_X, .EDIT_PREDICTION_POPOVER_PADDING_Y, EditorMode (+11 more)

### Community 80 - "agent_ui: copythreadtoclipboard"
Cohesion: 0.02
Nodes (150): AGENT_PANEL_KEY, AgentPanelEntryKind, Terminal, Thread, build_conflict_resolution_prompt(), build_conflicted_files_resolution_prompt(), conflict_resource_block(), expect_resource_block() (+142 more)

### Community 81 - "keymap_editor: CopyAction"
Cohesion: 0.03
Nodes (50): ActionArgumentsEditor, ActionInformation, ActionMapping, base_button_style(), binding_is_unbound_by_unbind(), binding_is_unbound_by_unbind_respects_precedence(), ConflictKeybindMapping, ConflictOrigin (+42 more)

### Community 82 - "agent: agentmodelgroupname"
Cohesion: 0.03
Nodes (138): always_allow_tools(), collect_events_until_stop(), disable_sandboxing(), expect_tool_call(), expect_tool_call_update_fields(), FakeSubagentHandle, FakeTerminalHandle, FakeThreadEnvironment (+130 more)

### Community 83 - "language_model: fake_provider.rs"
Cohesion: 0.02
Nodes (25): FakeLanguageModelProvider, FakeProviderState, PendingCompletion, ToolUseRequest, DisabledReason, init(), LanguageModel, LanguageModelCostInfo (+17 more)

### Community 84 - "agent: bracketmatch"
Cohesion: 0.01
Nodes (108): ActiveDebugLine, all_edits_insertions_or_deletions(), BreakpointPromptEditor, .MAX_LINES, char_len_with_expanded_tabs(), ClipboardSelection, CODE_ACTION_TIMEOUT, CODE_ACTIONS_DEBOUNCE_TIMEOUT (+100 more)

### Community 85 - "collab: ack"
Cohesion: 0.03
Nodes (106): acknowledge_buffer_version(), acknowledge_channel_message(), broadcast(), broadcast_project_message_from_host(), build_channels_update(), build_update_user_channels(), call(), cancel_call() (+98 more)

### Community 86 - "settings: asset_str"
Cohesion: 0.02
Nodes (70): KeyBindingValidatorRegistration, CopilotSettings, DEFAULT_KEYMAP_PATH, default_settings(), test_settings(), init(), initial_local_debug_tasks_content(), initial_tasks_content() (+62 more)

### Community 87 - "workspace: .position_is_valid()"
Cohesion: 0.03
Nodes (20): AnyView, Dock, DockPosition, Bottom, Left, Right, DockRestoreState, Finished (+12 more)

### Community 88 - "gpui_linux: asfd"
Cohesion: 0.02
Nodes (66): power_mode(), PriorityQueueCalloopReceiver, abandoned_acquisition_is_an_error(), acquisition_error_passes_through(), assert_error_contains(), await_idle_sleep_prevention(), Blank, capslock_from_xkb() (+58 more)

### Community 89 - "markdown_preview: bufferdiff"
Cohesion: 0.03
Nodes (84): assert_editor_is_active_and_focused(), assert_editor_replaced_by_preview(), assert_no_markdown_preview_items(), assert_resolved_preview_image_path(), automatic_preview_preserves_pane_activation_and_focus(), automatic_previews_preserve_temporary_tab_semantics(), click_tab_menu_target(), close_and_return_to_editor_closes_preview_and_focuses_source_editor() (+76 more)

### Community 90 - "gpui_wgpu: AdapterInfo"
Cohesion: 0.02
Nodes (64): get_gamma_correction_ratios(), CompositorGpuHint, device_errors_are_observed_independently(), device_errors_coalesce_and_new_observers_see_latest(), DeviceErrorState, parse_pci_id(), PreparedWebGraphics, test_parse_device_id() (+56 more)

### Community 91 - "remote: .global()"
Cohesion: 0.02
Nodes (77): GlobalExtensionStore, init(), AgentServerStoreState, Collab, Local, Remote, EnvironmentOrigin, Cli (+69 more)

### Community 92 - "project_panel: project_panel_tests.rs"
Cohesion: 0.03
Nodes (167): assert_drag_state_cleared(), click_project_entry(), create_drag_test_panel(), drag_entries_onto(), drag_selection_to(), ensure_no_open_items_and_panes(), ensure_single_file_is_opened(), enter_file_drag_over_root() (+159 more)

### Community 93 - "src: Accessible Mode {#accessible-m"
Cohesion: 0.01
Nodes (202): Accessible Mode {#accessible-mode}, Agent Buffer Font Family {#agent-buffer-font-family}, Agent Buffer Font Size {#agent-buffer-font-size}, Agent Servers {#agent-servers}, Agent UI Font Family {#agent-ui-font-family}, Agent UI Font Size, Allow Rewrap, Always Treat Brackets As Autoclosed (+194 more)

### Community 94 - "sum_tree: Option<BufferId>"
Cohesion: 0.03
Nodes (43): Option<BufferId>, (), Cursor<'a, 'b, T, D>, End, End<D>, FilterCursor, FilterCursor<'a, 'b, F, T, D>, FilterCursor<'a, 'b, F, T, U> (+35 more)

### Community 95 - "terminal_view: terminal_view.rs"
Cohesion: 0.03
Nodes (68): active_entry_directory_active_dir(), active_entry_directory_active_file(), active_entry_directory_no_active_entry(), active_entry_worktree_is_dir(), active_entry_worktree_is_file(), add_display_only_terminal(), altgr_character_input_is_not_swallowed_as_meta_sequence(), assert_drop_writes_to_terminal() (+60 more)

### Community 96 - "picker: picker.rs"
Cohesion: 0.03
Nodes (29): ConfirmInput, ContainerKind, List, UniformList, DEFAULT_MODAL_MAX_HEIGHT, DEFAULT_MODAL_WIDTH, default_shape_for_layout(), Direction (+21 more)

### Community 97 - "zeta_prompt: .format_edit_history()"
Cohesion: 0.02
Nodes (186): compute_editable_and_context_ranges(), compute_editable_range_from_text(), compute_legacy_excerpt_ranges(), compute_line_starts(), containing_syntax_boundaries_from_ranges(), estimate_tokens_for_row_range(), ExcerptRanges, expand_context_from_text() (+178 more)

### Community 98 - "editor: Bound"
Cohesion: 0.04
Nodes (75): Cursor, assert_excerpts_resolve(), assert_wrap_edits_cover_changes(), balancing_block(), Block, BufferHeader, Custom, ExcerptBoundary (+67 more)

### Community 99 - "gpui_macos: anyprotocol"
Cohesion: 0.03
Nodes (111): NSRange, TISCopyCurrentKeyboardInputSource(), accepts_first_mouse(), add_mouse_tracking_area(), add_titlebar_accessory_view_controller(), apply_simple_fullscreen_plan(), attributed_substring_for_proposed_range(), BLURRED_VIEW_CLASS (+103 more)

### Community 100 - "acp_thread: .can_set_title()"
Cohesion: 0.02
Nodes (32): FakeAgentSessionSetTitle, AgentConnection, AgentSessionClientUserMessageIds, AgentSessionInfo, AgentSessionInfoUpdate, V1, V2, AgentSessionRetry (+24 more)

### Community 101 - "project: CompletionListItemDefaults"
Cohesion: 0.02
Nodes (49): lsp_to_symbol_kind(), point_from_lsp(), range_from_lsp(), LanguageServerId, process_full_diagnostics_report(), process_full_workspace_diagnostics_report(), process_related_documents(), process_unchanged_diagnostics_report() (+41 more)

### Community 102 - "repl: clearcurrentoutput"
Cohesion: 0.02
Nodes (66): KernelListItem, build_python_discovery_shell_script(), extract_environment_kind(), Kernel, ErroredLaunch, Restarting, RunningKernel, Shutdown (+58 more)

### Community 103 - "terminal_view: activatenextpane"
Cohesion: 0.03
Nodes (68): RevealStrategy, Always, Never, NoFocus, build_serialized_pane_group(), deserialize_pane_group(), deserialize_terminal_panel(), deserialize_terminal_views() (+60 more)

### Community 104 - "ui: axis_as_scrollbaraxis"
Cohesion: 0.02
Nodes (84): Position, Absolute, Relative, taffy::style::Position, AnimationState, InProgress, Stale, BORDER_WIDTH (+76 more)

### Community 105 - "agent_servers: agentsessioninfo"
Cohesion: 0.04
Nodes (114): abandoned_new_session_closes_late_response(), abandoned_new_session_releases_registration_barrier(), AcpSessionConfigOptions, additional_directories_support_respects_agent_capability(), agent_exit_preserves_final_updates_and_diagnostics(), agent_exit_with_inherited_handles_preserves_final_output(), assert_agent_exit_preserves_final_output(), build_fake_acp_connection() (+106 more)

### Community 106 - "agent_ui: completion_menu_max_width"
Cohesion: 0.03
Nodes (90): AgentContextSelection, Editor, Terminal, AgentContextSource, Editor, TerminalPanel, TerminalView, AvailableSkill (+82 more)

### Community 107 - "language: Buffer"
Cohesion: 0.03
Nodes (32): Buffer, BufferBranchState, BufferEditSource, Agent, Remote, User, BufferEvent, CapabilityChanged (+24 more)

### Community 108 - "edit_prediction_metrics: apply_diff_to_string_with_hunk"
Cohesion: 0.03
Nodes (149): classify_retrieved_rows(), classify_values(), count_missing_rows(), counts_false_negatives_and_file_false_positives(), deletion_counts_deleted_old_line_as_true_positive(), editable_context_coverage(), EditableContextCoverage, empty_patch_with_no_context_has_perfect_f1() (+141 more)

### Community 110 - "agent_ui: .is_retained_thread()"
Cohesion: 0.04
Nodes (82): thread_metadata_to_debug_json(), delete(), active_thread_id(), acp_thread::AgentSessionInfo, ArchivedGitWorktree, clear_thread_metadata_remote_connection_backfill(), DbOperation, Delete (+74 more)

### Community 111 - "text: .get()"
Cohesion: 0.03
Nodes (35): Lamport, .MAX, .MIN, Seq, Buffer, Edit<(D1, D2)>, EditedBufferSnapshot, EditOperation (+27 more)

### Community 112 - "terminal_view: .to_run()"
Cohesion: 0.03
Nodes (75): assert_emergency_wrap_preserves_graphemes(), Boundary, build_wrapper(), CharWidthCache, generate_test_runs(), LineFragment, LineFragment<'a>, Element (+67 more)

### Community 113 - "terminal: cursor_blinking()"
Cohesion: 0.03
Nodes (87): cursor_blinking(), new_term(), alt_scroll(), assert_content_eventually(), assert_foreground_process_command_eventually(), build_test_subprocess_terminal(), build_test_terminal(), build_test_terminal_with_arguments() (+79 more)

### Community 114 - "collab: AnyConnection"
Cohesion: 0.02
Nodes (72): TestDb, ChannelBufferOperation, EditChannelNotes, JoinChannelNotes, LeaveChannelNotes, Noop, RandomChannelBufferTest, test_random_channel_buffers() (+64 more)

### Community 115 - "collab: behaviorversion"
Cohesion: 0.02
Nodes (80): calculate_json_checksum(), for_snowflake(), post_events(), SnowflakeRow, ZedChecksumHeader, download_extension(), download_latest_extension(), DownloadExtensionParams (+72 more)

### Community 116 - "client: CrashInfo"
Cohesion: 0.02
Nodes (61): Client, ClientCredentialsProvider, ClientState, Credentials, EstablishConnectionError, InvalidHeaderValue, Io, Other (+53 more)

### Community 117 - "zed: IncomingCall"
Cohesion: 0.03
Nodes (67): IncomingCall, init(), init(), init(), IncomingCallNotification, IncomingCallNotificationState, init(), init() (+59 more)

### Community 118 - "terminal: AlacCell"
Cohesion: 0.02
Nodes (84): add_alacritty_mode(), add_terminal_mode(), adjusted_last_hovered_word(), alacritty_cursor_shape(), alacritty_cursor_style(), AlacrittyCell, AlacrittyGridIterator, AlacrittyPty (+76 more)

### Community 119 - "gpui_windows: common"
Cohesion: 0.03
Nodes (67): quad_depth(), quad_depth_scale(), AGS_CURRENT_VERSION, AGSContext, agsDeInitialize_t, AGSGPUInfo, agsInitialize_t, Annotation (+59 more)

### Community 120 - "workspace: Context"
Cohesion: 0.04
Nodes (21): show_configure_mcp_modal(), ProjectGroupKey, DraggedSidebar, Entity<T>, HeldWorkspace, MultiWorkspace, MultiWorkspaceEvent, ActiveWorkspaceChanged (+13 more)

### Community 121 - "agent: .create_sibling_thread()"
Cohesion: 0.03
Nodes (113): AvailableAgent, AvailableAgents, AvailableModel, SiblingThreadInfo, SiblingThreadRequest, ThreadEnvironment, CreateThreadTool, .NAME (+105 more)

### Community 122 - "anthropic: src/anthropic.rs"
Cohesion: 0.02
Nodes (138): AdaptiveThinkingDisplay, Omitted, Summarized, ANTHROPIC_API_URL, AnthropicError, ApiError, BuildRequestBody, DeserializeResponse (+130 more)

### Community 123 - "acp_thread: mention.rs"
Cohesion: 0.03
Nodes (98): absolute_path_mention(), bare_path_target(), decode_path_escapes(), DecodePercentEscapes, No, Yes, default_deprecated_rule_id(), default_include_errors() (+90 more)

### Community 124 - "src: debugger.md"
Cohesion: 0.01
Nodes (108): Additional Community Language Extensions, AsciiDoc, Astro, Using the Tailwind CSS Language Server with Astro, Biome, Biome Language Support, Configuration, Clojure (+100 more)

### Community 125 - "settings_ui: agent_skills_settings_path"
Cohesion: 0.03
Nodes (89): render_ollama_model_picker(), get_current_device(), render_audio_device_dropdown(), render_input_audio_device_dropdown(), render_output_audio_device_dropdown(), render_settings_audio_device_dropdown(), SYSTEM_DEFAULT, ActionLink (+81 more)

### Community 126 - "git: BlameBufferResponse"
Cohesion: 0.02
Nodes (106): get_messages(), get_messages_impl(), get_tag_names(), parse_tag_names(), test_parse_tag_names_for_lightweight_and_annotated_tags(), BISECT_LOG, COMMIT_MESSAGE, decode_hex_digit() (+98 more)

### Community 127 - "settings: default.json"
Cohesion: 0.01
Nodes (159): accessible_mode, agent_buffer_font_family, agent_buffer_font_size, agent_ui_font_family, agent_ui_font_size, allow_rewrap, always_treat_brackets_as_autoclosed, auto_indent (+151 more)

### Community 128 - "context_server: oauth.rs"
Cohesion: 0.03
Nodes (136): auth_server_metadata_urls(), AuthServerMetadata, AuthServerMetadataResponse, build_authorization_url(), canonical_server_uri(), CIMD_URL, ClientRegistrationStrategy, Cimd (+128 more)

### Community 129 - "search: base_rem_size_in_px"
Cohesion: 0.05
Nodes (48): Tab, BufferSearchBar, display_points_of(), init(), init_globals(), init_multibuffer_test(), init_test(), MAX_BUFFER_SEARCH_HISTORY_SIZE (+40 more)

### Community 130 - "git_ui: CheckoutBranch"
Cohesion: 0.04
Nodes (73): Branch, BranchesScanResult, CommitSummary, delete_branch_flag(), Upstream, BRANCH_DELETE_FORCE_DELETE_PROMPTS, branch_filter_menu(), branch_matches_ref() (+65 more)

### Community 131 - "agent: .extend_key_context()"
Cohesion: 0.03
Nodes (23): BufferOffset, choose_completion_range(), CodeActionProvider, CollaborationHub, CompletionProvider, Entity<Project>, get_uncommitted_diff_for_buffer(), inline_completion_edit_text() (+15 more)

### Community 132 - "bench_metrics: bench_metrics.rs"
Cohesion: 0.03
Nodes (62): active_secondaries(), allocation_stats(), AllocationCounter, AllocationMetric, AllocatedBytes, Allocations, allocations_name(), AllocationStats (+54 more)

### Community 133 - "project: .worktree_id()"
Cohesion: 0.03
Nodes (63): tasks_file(), missing_parent_dirs(), context_with_greeting(), DebugScenario, .GLOBAL_SOURCE_FILE, .LABEL, DebugScenarioContext, greeting_template() (+55 more)

### Community 134 - "settings_content: settings_content/src/language."
Cohesion: 0.02
Nodes (126): &'a FileTypeMap, AllLanguageSettingsContent, AutoIndentMode, None, PreserveIndent, SyntaxAware, CodestralSettingsContent, CompletionSettingsContent (+118 more)

### Community 135 - "agent_servers: CancelSessionNotification"
Cohesion: 0.05
Nodes (95): is_resource_not_found(), begin_session_creation(), bounded(), cancel_request_elicitations(), config_option_supports_value(), finish_request(), finish_session_creation(), Harness (+87 more)

### Community 136 - "gpui: .create_terminal()"
Cohesion: 0.03
Nodes (19): TerminalHandle, AsyncApp, AsyncWindowContext, install_cli_binary(), install_script(), register_zed_scheme(), FakeCredentialsProvider, TestCredentialsProvider (+11 more)

### Community 137 - "vim: IndentDirection"
Cohesion: 0.02
Nodes (112): IndentDirection, Auto, In, Out, Vim, Motion, CurrentLine, Down (+104 more)

### Community 138 - "buffer_diff: Anchor"
Cohesion: 0.04
Nodes (50): Anchor, BufferDiff, BufferDiffEvent, BaseTextChanged, DiffChanged, BufferDiffSnapshot, BufferDiffUpdate, compare_hunks() (+42 more)

### Community 139 - "agent_skills: component_matches_ignore_ascii"
Cohesion: 0.02
Nodes (134): AGENTS_DIR_NAME, BUILTIN_SKILL_ENTRIES, builtin_skills(), CREATE_SKILL_CONTENT, decode_skill_share_link(), decode_skill_share_link_rejects_non_skill_links(), encode_skill_share_link(), extract_frontmatter() (+126 more)

### Community 140 - "language: Anchor"
Cohesion: 0.02
Nodes (49): Anchor, ItemSummary, Range<Anchor>, ChangedRegion, ChangeRegionSet, ChangeStartPosition, get_injections(), has_parent() (+41 more)

### Community 141 - "edit_prediction_cli: FailedHandling"
Cohesion: 0.03
Nodes (126): FailedHandling, Keep, Skip, SkipNoFiles, apply_edits(), count_lines(), DiffStats, edit_locations() (+118 more)

### Community 142 - "agent_servers: ConnectionTo"
Cohesion: 0.03
Nodes (29): AgentSessionList, AgentSessionModes, AgentTelemetry, AcpConnection, AcpConnectionDefaults, AcpSessionList, AcpSessionModes, apply_v1_subagent_update() (+21 more)

### Community 143 - "agent_servers: acpbetafeatureflag"
Cohesion: 0.03
Nodes (63): AcpSession, apply_subagent_update(), apply_v2_session_notification(), beta_selection_uses_fresh_v1_fallback_only_for_version_mismatch(), claude_adapter_initialize_smoke(), ClientContext, ClientDisconnectWork, close_undelivered_session() (+55 more)

### Community 144 - "language: server_precedences()"
Cohesion: 0.02
Nodes (47): server_precedences(), BinaryStatus, CheckingForUpdate, Downloading, Failed, None, Starting, Stopped (+39 more)

### Community 145 - "debugger_ui: resolve_path()"
Cohesion: 0.04
Nodes (34): resolve_path(), build_serialized_layout(), build_serialized_pane_layout(), DEBUGGER_PANEL_PREFIX, DebuggerPaneItem, BreakpointList, Console, Frames (+26 more)

### Community 146 - "edit_prediction_cli: line_start_offset()"
Cohesion: 0.03
Nodes (88): line_start_offset(), fetch_if_needed(), EpAppState, init(), apply_edit_history(), cursor_position(), run_load_project(), setup_project() (+80 more)

### Community 147 - "agent_ui: build_terminal_prompt_editor()"
Cohesion: 0.03
Nodes (42): build_terminal_prompt_editor(), CompletionState, Generated, Pending, Rated, extract_message_creases(), GenerationMode, Generate (+34 more)

### Community 148 - "fs: .directories()"
Cohesion: 0.04
Nodes (84): PathBuf, PathEvent, PathEventKind, Changed, Created, Removed, Rescan, case_insensitive_registration_matches_differently_cased_event() (+76 more)

### Community 149 - "gpui: src/randomized_element_tree.rs"
Cohesion: 0.04
Nodes (68): AppliedRandomizedElementTreeMutation, assert_matches_reference(), bounds_sample_within_range_and_reproduce_from_the_seed(), children_at_path(), children_at_path_mut(), draw_fresh_reference(), draw_window(), ELEMENT_COUNT (+60 more)

### Community 150 - "text: BufferOffsetUtf16"
Cohesion: 0.03
Nodes (17): BufferOffsetUtf16, AnchorRangeExt, OffsetRangeExt, Range<Anchor>, Range<T>, Selection<Anchor>, Anchor, BufferSnapshot (+9 more)

### Community 151 - "recent_projects: user_ssh_config_file()"
Cohesion: 0.05
Nodes (47): user_ssh_config_file(), AddWslDistro, CreateRemoteDevContainer, CreateRemoteServer, DevContainerCreationProgress, Creating, Error, SelectingConfig (+39 more)

### Community 152 - "ui: arrow_key_with_pointer_on_subm"
Cohesion: 0.05
Nodes (49): arrow_key_with_pointer_on_submenu_trigger_navigates_the_submenu(), assert_focus_selection(), can_navigate_back_over_headers(), clicking_embedded_view_outside_its_focus_keeps_content_focused(), confirm_without_selection_dismisses(), confirming_submenu_trigger_focuses_its_content(), ContextMenu, ContextMenuItem (+41 more)

### Community 153 - "language_models_cloud: client_supports_status_message"
Cohesion: 0.04
Nodes (69): CompletionBody, CompletionEvent, Event, Status, ListModelsResponse, CompactionResult, anthropic_request(), ApiError (+61 more)

### Community 154 - "editor: .get()"
Cohesion: 0.04
Nodes (25): CodeActionsMenu, CodeContextMenu, CodeActions, Completions, CompletionMatchResults, CompletionMenuEntry, Divider, GroupHeader (+17 more)

### Community 155 - "file_finder: parse_file_search_query()"
Cohesion: 0.05
Nodes (111): parse_file_search_query(), parse_line_range_query(), SEARCH_DEBOUNCE, active_file_picker(), advance_worktree_update_refresh(), assert_match_at_position(), assert_match_selection(), bind_default_keymap() (+103 more)

### Community 156 - "gpui_windows: WindowAppearance"
Cohesion: 0.03
Nodes (30): configure_dwm_dark_mode(), get_window_long(), set_window_long(), system_appearance(), calculate_client_rect(), calculate_window_rect(), Callbacks, ClickState (+22 more)

### Community 157 - "language_model_core: language_model_core.rs"
Cohesion: 0.02
Nodes (91): AuthenticateError, ConnectionRefused, CredentialsNotFound, Other, category_from_cloud_failure(), CompactionUpdate, Failed, Finished (+83 more)

### Community 158 - "rpc: any"
Cohesion: 0.02
Nodes (30): Envelope, REMOTE_SERVER_PEER_ID, AnyTypedEnvelope, EntityMessage, EnvelopedMessage, .NAME, .PRIORITY, LspRequestId (+22 more)

### Community 159 - "terminal: Attr"
Cohesion: 0.02
Nodes (96): AlacrittyHyperlink, cursor_line(), history_size(), to_vte_rgb(), AnsiSpans, ClipboardFormatter, ColorFormatter, convert_lf_to_crlf() (+88 more)

### Community 160 - "vim: bracket_pairs"
Cohesion: 0.03
Nodes (112): BracketPair, Vim, AngleBrackets, CandidateRange, CandidateWithRanges, CurlyBrackets, DelimiterPredicate, get_visible_line_range() (+104 more)

### Community 161 - "gpui: .set_input_handler()"
Cohesion: 0.03
Nodes (25): HeadlessWindowState, ConfigurationTestView, ElementInputHandler, ElementInputHandler<V>, EntityInputHandler, text_input_configuration_and_focus_state_are_forwarded_on_change(), text_notifications_require_a_focused_input_and_track_field_changes(), Autocapitalize (+17 more)

### Community 162 - "settings_ui: project_skills_relative_path()"
Cohesion: 0.03
Nodes (63): project_skills_relative_path(), BODY_FIELD_TAB_INDEX, derived_description_from_markdown(), derived_skill_name_from_url(), derived_skill_name_strips_markdown_extension_case_insensitively(), DESCRIPTION_FIELD_TAB_INDEX, DISABLE_MODEL_INVOCATION_TAB_INDEX, ensure_markdown_path() (+55 more)

### Community 163 - "gpui_windows: check_can_show_windows()"
Cohesion: 0.02
Nodes (21): check_can_show_windows(), check_device_lost(), handle_gpu_device_lost(), has_package_identity(), is_window_station_visible(), load_icon(), open_target(), open_target_in_explorer() (+13 more)

### Community 164 - "agent_ui: anthropiceventreporter"
Cohesion: 0.05
Nodes (54): test_rollback_on_canceled_receiver(), BufferCodegen, CodegenAlternative, CodegenEvent, Finished, Undone, FAILURE_MESSAGE_TOOL_NAME, failure_tool_use() (+46 more)

### Community 165 - "fs: CreateOptions"
Cohesion: 0.03
Nodes (15): CreateOptions, FakeFs, .SYSTEMTIME_INTERVAL, FakeFsEntry, Dir, File, Symlink, FakeFsState (+7 more)

### Community 166 - "picker: Stateful"
Cohesion: 0.04
Nodes (37): Stateful, last_layout_key(), layout_as_str(), load_last_preview_layout(), parse_layout(), parse_shape(), PickerConfig, PICKERS_NAMESPACE (+29 more)

### Community 167 - "repl: CellMetadata"
Cohesion: 0.03
Nodes (36): assert_layer_languages(), Cell, Code, Markdown, Raw, CellControl, CellPosition, First (+28 more)

### Community 168 - "vim: helix.rs"
Cohesion: 0.03
Nodes (98): active_helix_jump_labels(), active_helix_jump_overlay_counts(), assert_helix_jump_bypasses_text_input(), assert_helix_jump_cleared(), bind_vim_jump_to_word(), dispatch_native_printable_key(), HELIX_JUMP_ALPHABET, helix_jump_label_for_word() (+90 more)

### Community 169 - "search: .new()"
Cohesion: 0.03
Nodes (34): ContactFinderDelegate, ExtensionVersionSelectorDelegate, PickerPromptDelegate, Picker, Presentation, Embedded, Modal, Popover (+26 more)

### Community 170 - "outline_panel: entry_git_aware_label_color()"
Cohesion: 0.05
Nodes (17): entry_git_aware_label_color(), entry_label_color(), ActiveItem, Dir, empty_icon(), find_active_indent_guide_ix(), FsChildren, ExternalFile (+9 more)

### Community 171 - "node_runtime: node_runtime.rs"
Cohesion: 0.03
Nodes (80): ArchiveType, TarGz, Zip, build_npm_command_args(), check_node_version(), deserialize_npm_info_from_response(), deserialize_npm_info_time(), DetectError (+72 more)

### Community 172 - "sandbox_linux: allow"
Cohesion: 0.03
Nodes (77): Access, Allow, Ask, Deny, AddressClass, LocalNetwork, Loopback, Public (+69 more)

### Community 173 - "agent_ui: choicecard"
Cohesion: 0.04
Nodes (86): ElicitationStatus, Accepted, Canceled, Completed, Declined, Pending, BooleanHandler, display_url_segments() (+78 more)

### Community 174 - "collab_ui: CollapseSelectedChannel"
Cohesion: 0.05
Nodes (8): ChannelId, ChannelMoveClipboard, CollabPanel, ContactContextMenu, init(), panel_row_height(), render_participant_name_and_handle(), render_tree_branch()

### Community 175 - "project: point_to_lsp()"
Cohesion: 0.03
Nodes (41): point_to_lsp(), file_path_to_lsp_url(), LspCommand, make_lsp_text_document_position(), make_text_document_identifier(), EMMET_SERVER_NAME, ExpandAbbreviation, ExpandAbbreviationOptions (+33 more)

### Community 176 - "agent_servers: Response"
Cohesion: 0.04
Nodes (35): Response, drain_foreground_queue(), enqueue_notification(), ForegroundWork, map_acp_error(), mcp_configurations_for_project(), ClientRuntime, disconnect_cleanup() (+27 more)

### Community 177 - "anthropic: compactionupdate"
Cohesion: 0.03
Nodes (104): CacheControl, CacheControlType, Ephemeral, CacheTtl, FiveMinutes, OneHour, ContentDelta, CompactionDelta (+96 more)

### Community 178 - "gpui: TasksIncluded"
Cohesion: 0.04
Nodes (76): TasksIncluded, CompletedAndRunning, OnlyCompleted, ActiveTiming, begin_input_at(), clear_trace_buffers(), disabling_tracing_clears_frame_events(), draw_and_present() (+68 more)

### Community 179 - "call: call_impl/participant.rs"
Cohesion: 0.03
Nodes (26): LocalParticipant, RemoteParticipant, Event, LocalScreenShareStarted, LocalScreenShareStopped, ParticipantLocationChanged, RemoteAudioTracksChanged, RemoteProjectInvitationDiscarded (+18 more)

### Community 180 - "editor: .deref()"
Cohesion: 0.04
Nodes (32): Fold, consolidate_fold_edits(), consolidate_inlay_edits(), Fold, FoldChunks<'a>, FoldEdit, FoldMap, FoldMapWriter (+24 more)

### Community 181 - "gpui: .assert_no_new_leaks()"
Cohesion: 0.04
Nodes (25): AccessedEntities, AnyEntity, AnyWeakEntity, BacktraceFormatter, double_lease_panic(), Entity, Entity<T>, EntityLeakData (+17 more)

### Community 182 - "search: active_search_view()"
Cohesion: 0.05
Nodes (13): active_search_view(), ActiveSettings, init(), is_buffer_stale(), ProjectSearchBar, ProjectSearchSettings, ProjectSearchView, register_workspace_action() (+5 more)

### Community 183 - "settings_sync: account.rs"
Cohesion: 0.03
Nodes (84): AccountStatus, Error, SignedIn, SignedOut, SigningIn, Starting, CREDENTIALS_URL, GitHubAccount (+76 more)

### Community 184 - "agent_ui: AgentProfileContent"
Cohesion: 0.04
Nodes (34): AgentProfile, AgentProfileSettings, ASK, AvailableProfiles, ContextServerPreset, explicit_false_disables_tool_when_enable_all_is_true(), explicit_true_enables_tool_when_enable_all_is_false(), is_builtin() (+26 more)

### Community 185 - "settings_ui: all_projects()"
Cohesion: 0.06
Nodes (33): all_projects(), CONTENT_MIN_WIDTH, draw_settings_page(), last_setting_preserves_focused_input_across_reflow(), navbar_double_click_toggle(), navbar_focus_navigation_reaches_last_section(), navbar_navigation_reaches_last_section(), navbar_selection_follows_content_scroll() (+25 more)

### Community 186 - "settings_content: agent"
Cohesion: 0.02
Nodes (113): AudioInputDeviceName, AudioOutputDeviceName, AudioSettingsContent, BaseKeymapContent, Atom, Cursor, Emacs, JetBrains (+105 more)

### Community 187 - "project: BreakpointsForFile"
Cohesion: 0.03
Nodes (42): ActiveStackFrame, BreakpointSessionState, BreakpointsInFile, BreakpointStore, BreakpointStoreEvent, BreakpointsCleared, BreakpointsUpdated, ClearDebugLines (+34 more)

### Community 188 - "extension_host: codelabelspanliteral"
Cohesion: 0.04
Nodes (50): WasmState, authorize_access_to_unreleased_wasm_api_version(), Extension, V0_0_1, V0_0_4, V0_0_6, V0_1_0, V0_2_0 (+42 more)

### Community 189 - "extension_host: .new()"
Cohesion: 0.03
Nodes (25): FakeExtension, ExtensionCall, WasmExtension, Extension, KeyValueStoreDelegate, ProjectDelegate, WorktreeDelegate, CodeLabel (+17 more)

### Community 190 - "editor: assert_chunks_are_contiguous()"
Cohesion: 0.04
Nodes (37): assert_chunks_are_contiguous(), chunk_search_range_for_test(), clip_ranges(), compute_format_target(), compute_modified_ranges(), deserialize_anchor(), deserialize_editor(), deserialize_selection() (+29 more)

### Community 191 - "project: AllocateWorktreeId"
Cohesion: 0.04
Nodes (24): DbTrustedPaths, find_worktree_in_store(), init(), AbsPath, Worktree, track_worktree_trust(), TrustedPaths, Restricted (+16 more)

### Community 192 - "git_ui: RestoreFile"
Cohesion: 0.05
Nodes (70): RestoreFile, assert_editor_opened_with_path(), assert_entry_paths(), await_git_panel_entries(), directory_index_for_repo_path(), discard_row(), entry_index_for_repo_path(), init_test() (+62 more)

### Community 193 - "remote: parse_shell()"
Cohesion: 0.04
Nodes (47): parse_shell(), build_command_posix(), command_output(), command_output_drains_output_while_writing_input(), command_output_preserves_stderr_when_input_fails(), environment_posix_probe_retries_when_stdin_is_consumed(), escape_sftp_path(), find_existing_control_master() (+39 more)

### Community 194 - "reqwest_client: batches"
Cohesion: 0.02
Nodes (42): forward_frame(), MessageStream, CloudApiClient, Connection, KEEPALIVE_INTERVAL, CloudApiClient, CONNECT_TIMEOUT, Connection (+34 more)

### Community 195 - "auto_update: autoupdateevent"
Cohesion: 0.03
Nodes (65): AssetQuery, AutoUpdateEvent, UpToDate, AutoUpdater, AutoUpdateStatus, Checking, Downloading, Errored (+57 more)

### Community 196 - "agent_ui: AgentSessionConfigOptions"
Cohesion: 0.05
Nodes (32): AgentSessionConfigOptions, AgentServer, config_defaults_wait_for_authoritative_response(), config_option_search_matches_all_searchable_fields(), config_option_search_preserves_name_order_with_different_metadata(), config_option_search_ranks_names_before_metadata(), config_option_search_returns_more_than_one_hundred_matches(), ConfigOptionPickerDelegate (+24 more)

### Community 197 - "agent: agent/src/db.rs"
Cohesion: 0.04
Nodes (46): acp_thread::AgentSessionInfo, DataType, Json, Zstd, DbLanguageModel, DbMessage, DbSandboxGrants, DbSummary (+38 more)

### Community 198 - "language: src/buffer_tests.rs"
Cohesion: 0.04
Nodes (111): assert_bracket_pairs(), assert_has_syntax_errors(), c_lang(), color_index_for_open(), erb_lang(), file(), get_tree_sexp(), html_lang() (+103 more)

### Community 199 - "agent: AcceptEditPredictionBinding"
Cohesion: 0.03
Nodes (37): consume_contiguous_rows(), DisplayDiffHunk, Folded, Unfolded, DisplayRow, EditorSnapshot, EditorStyle, EditPredictionPreview (+29 more)

### Community 200 - "agent: agentprofileid"
Cohesion: 0.06
Nodes (98): allow_anchor_prevents_middle(), allow_exact_match(), allow_middle_pattern(), allow_no_match_confirms(), allow_no_match_global_allows(), allow_no_match_tool_allow_overrides_global_confirm(), allow_no_match_tool_confirm_overrides_global_allow(), allow_one_of_many_patterns() (+90 more)

### Community 201 - "project: BufferReloaded"
Cohesion: 0.05
Nodes (20): split_operations(), apply_initial_line_ending(), BufferStore, BufferStoreEvent, BufferAdded, BufferChangedFilePath, BufferDropped, SharedBufferClosed (+12 more)

### Community 202 - "editor: BufferSubscription"
Cohesion: 0.05
Nodes (51): GoToDiagnostic, GoToPreviousDiagnostic, Str, chunks(), diagnostic_style(), DisplayMap, HighlightedChunk<'a>, init_test() (+43 more)

### Community 203 - "agent_ui: OutputTokensUpdate"
Cohesion: 0.03
Nodes (83): OutputTokensUpdate, LegacyAdd, Snapshot, Unchanged, ContentBlock, AuthorizeToolCall, DEFAULT_THREAD_TITLE, open_abs_path_at_point() (+75 more)

### Community 204 - "client: client/src/llm_token.rs"
Cohesion: 0.03
Nodes (34): global_llm_token(), GlobalRefreshLlmTokenListener, http_client::Response<http_client::AsyncBody>, LlmTokenRefreshedEvent, NeedsLlmTokenRefresh, RefreshLlmTokenListener, TokenRefreshMode, ClearAndRefresh (+26 more)

### Community 205 - "dap: IpAddr"
Cohesion: 0.03
Nodes (30): TcpArguments, DapMessageHandler, DebugAdapterClient, init_test(), test_calls_event_handler(), test_calls_event_handler_for_reverse_request(), test_initialize_client(), FakeTransport (+22 more)

### Community 206 - "debugger_ui: DebuggerProvider"
Cohesion: 0.05
Nodes (8): DebuggerProvider, DebugPanel, register_session_inner(), DebugPanel, SessionListEntry, DebugSession, test_truncate_and_trailoff(), truncate_and_trailoff()

### Community 207 - "zeta_prompt: format_cursor_excerpt()"
Cohesion: 0.03
Nodes (84): format_cursor_excerpt(), ACCEPTED_PREDICTION_MARKER, CURSOR_POSITION_HEADING, EDIT_HISTORY_HEADING, ExampleSpec, EXPECTED_PATCH_HEADING, FrontMatter, HumanFeedback (+76 more)

### Community 208 - "cli: app_identifier"
Cohesion: 0.03
Nodes (52): CliBehaviorSetting, ExistingWindow, NewWindow, anonymous_fd(), App, Args, Bundle, App (+44 more)

### Community 209 - "settings_content: InactiveOpacity"
Cohesion: 0.02
Nodes (108): InactiveOpacity, DockSide, Left, Right, ShowIndentGuides, Always, Never, ActivateOnClose (+100 more)

### Community 210 - "vim: vim/src/command.rs"
Cohesion: 0.04
Nodes (71): act_on_range(), assert_active_item(), command_interceptor(), CommandRange, commands(), DeleteMarks, AllLocal, Marks (+63 more)

### Community 211 - "agent_settings: agent_settings.rs"
Cohesion: 0.03
Nodes (74): AutoCompactSettings, AutoCompactThreshold, .DEFAULT, Percentage, TokensRemaining, TokensUsed, check_hardcoded_security_rules(), COMPACTION_PROMPT (+66 more)

### Community 212 - "agent_ui: AgentPanelEvent"
Cohesion: 0.04
Nodes (50): AgentPanelEvent, ActiveViewChanged, ActiveViewFocused, EntryChanged, TerminalCloseRequested, ThreadInteracted, compose_terminal_thread_title(), DbOperation (+42 more)

### Community 213 - "git_ui: .staging()"
Cohesion: 0.03
Nodes (52): StageStatus, PartiallyStaged, Staged, Unstaged, DiffTarget, Staged, Uncommitted, Unstaged (+44 more)

### Community 214 - "cloud_api_client: .cached_llm_token()"
Cohesion: 0.03
Nodes (45): EditPredictionUsage, RequestUsage, Authentication, Credentials, Session, build_credentials_authenticated_request_with_authorization_header(), build_request(), build_session_authenticated_request_without_authorization_header() (+37 more)

### Community 215 - "file_finder: FileFinder"
Cohesion: 0.05
Nodes (22): FileFinder, FileFinderDelegate, FileSearchQuery, FoundPath, full_path_budget(), init(), Match, Absolute (+14 more)

### Community 216 - "settings_content: settings_content/src/editor.rs"
Cohesion: 0.02
Nodes (111): CodeLens, Menu, Off, On, CompletionDetailAlignment, Left, Right, CompletionMenuItemKind (+103 more)

### Community 217 - "agent_ui: agent_diff_renderer()"
Cohesion: 0.05
Nodes (27): agent_diff_renderer(), AgentDiff, AgentDiffAddon, AgentDiffGlobal, AgentDiffHunkRenderer, AgentDiffPane, AgentDiffToolbar, AgentDiffToolbarItem (+19 more)

### Community 218 - "migrator: migrator.rs"
Cohesion: 0.04
Nodes (112): assert_migrate_keymap(), assert_migrate_settings(), assert_migrate_with_migrations(), assert_migrated_correctly(), EDIT_PREDICTION_SETTINGS_MIGRATION_QUERY, define_query!, migrate(), migrate_edit_prediction_provider_settings() (+104 more)

### Community 219 - "multi_buffer: multi_buffer_tests.rs"
Cohesion: 0.04
Nodes (96): AnchorContractOp, BiasAnchor, ChangeBaseText, CreateAnchor, EditBuffer, RekeyPath, SwapDiff, ToggleSecondBuffer (+88 more)

### Community 220 - "call_hierarchy: buffer_text_style"
Cohesion: 0.05
Nodes (61): attach_labels(), BASE_MODAL_WIDTH, Call, call_display_path(), CallDisplay, CallHierarchyDelegate, CallHierarchyMode, Incoming (+53 more)

### Community 221 - "gpui_windows: .request_frame()"
Cohesion: 0.05
Nodes (16): cancel_dialog(), cancellation_requested(), close_dialog(), task_dialog_callback(), task_dialog_cancellation_requires_cancel_button_or_internal_request(), current_modifiers(), get_frame_thicknessx(), handle_key_event() (+8 more)

### Community 222 - "project: .stream_enabled_for_test()"
Cohesion: 0.04
Nodes (45): IoKind, StdErr, StdIn, StdOut, LanguageServerLogType, Log, Rpc, Trace (+37 more)

### Community 223 - "project: .as_ref()"
Cohesion: 0.04
Nodes (33): AgentServerCommand, AgentServerStore, AllAgentServersSettings, create_agent_server_store(), CustomAgentServerSettings, Custom, Registry, ExternalAgentEntry (+25 more)

### Community 224 - "theme: Item"
Cohesion: 0.02
Nodes (113): ThemeColorField, Background, Border, BorderDisabled, BorderFocused, BorderSelected, BorderTransparent, BorderVariant (+105 more)

### Community 225 - "src: billing.md"
Cohesion: 0.08
Nodes (26): Text Threads, All Actions, Command Palette, Agent Server Extensions, Core differences, Helix Mode, Configuration, XML (+18 more)

### Community 226 - "edit_prediction: asrefstr"
Cohesion: 0.04
Nodes (91): PREDICT_EDITS_MODE_HEADER_NAME, PREDICT_EDITS_REQUEST_ID_HEADER_NAME, PREDICT_EDITS_TRIGGER_HEADER_NAME, PredictEditsV3Request, PredictEditsV3Response, RawCompletionChoice, RawCompletionRequest, RawCompletionResponse (+83 more)

### Community 227 - "editor: BuildError"
Cohesion: 0.05
Nodes (12): ColumnarMode, FromMouse, FromSelection, DeferredSelectionEffectsState, SelectPhase, Begin, BeginColumnar, End (+4 more)

### Community 228 - "livekit_client: ClaimGrants"
Cohesion: 0.04
Nodes (26): LocalAudioTrack, LocalVideoTrack, RemoteAudioTrack, RemoteVideoTrack, assert_token_was_revoked(), ConnectionState, Connected, Disconnected (+18 more)

### Community 229 - "bedrock: models.rs"
Cohesion: 0.02
Nodes (71): BedrockAdaptiveThinkingEffort, High, Low, Max, Medium, XHigh, BedrockModelCacheConfiguration, BedrockModelMode (+63 more)

### Community 230 - "sqlez: CacheRow"
Cohesion: 0.04
Nodes (40): CacheRow, CacheRow, BreakpointState, Disabled, Enabled, (), Arc<Path>, Arc<str> (+32 more)

### Community 231 - "fs: BlobReadGateState"
Cohesion: 0.03
Nodes (21): BlobReadGateState, FakeBlobReadGate, FakeCommitDataEntry, Fail, Success, FakeCommitSnapshot, FakeGitRepository, FakeGitRepositoryState (+13 more)

### Community 232 - "zeta_prompt: extract_marker_span_allow_same"
Cohesion: 0.04
Nodes (113): extract_marker_span_allow_same(), apply_marker_span(), apply_marker_span_impl(), apply_marker_span_v0316(), apply_marker_span_v0317(), apply_marker_span_v0318(), collect_line_info(), collect_marker_tags() (+105 more)

### Community 233 - "extension: DebugAdapterRegistryProxy"
Cohesion: 0.03
Nodes (27): DebugAdapterRegistryProxy, init(), ExtensionContextServerProxy, ExtensionDebugAdapterProviderProxy, ExtensionGrammarProxy, ExtensionHostProxy, ExtensionLanguageModelProviderProxy, ExtensionLanguageProxy (+19 more)

### Community 234 - "dev_container: DevContainerUp"
Cohesion: 0.04
Nodes (98): DevContainerUp, build_tarball(), check_for_existing_container_errors_when_multiple_match(), compose_fragment_declares_name(), compose_fragment_declares_name_detects_top_level_name_key(), DEFAULT_REMOTE_PROJECT_DIR, derive_project_name(), derive_project_name_compose_name_wins_over_fallback() (+90 more)

### Community 235 - "git_ui: ActivateChangesTab"
Cohesion: 0.04
Nodes (67): base_picker_open(), branch_base(), branch_diff_bases(), BranchModeTest, DraggedGitEntryView, EntryMenuTitles, GenerateCommitMessageConfigurationTooltip, GIT_PANEL_KEY (+59 more)

### Community 236 - "sandbox: BpfProgram"
Cohesion: 0.04
Nodes (89): BridgeStream, build_bwrap_args(), build_bwrap_args_with_sandbox_paths(), build_command_seccomp_program(), build_wrapped_args_for_test(), bundled_bwrap_path(), BwrapLocation, Found (+81 more)

### Community 237 - "proxy_handshake: establish()"
Cohesion: 0.03
Nodes (91): establish(), Tunneled, domain_target(), Exchange, Expect, Send, Expectation, HttpResponseHead (+83 more)

### Community 238 - "scheduler: scheduler/src/executor.rs"
Cohesion: 0.04
Nodes (35): BackgroundExecutor, box_dedicated(), DedicatedExecutor, FallibleTask, FallibleTask<T>, FallibleTaskState, Downcast, Ready (+27 more)

### Community 239 - "theme_settings: FontFamilyName"
Cohesion: 0.04
Nodes (66): FontFamilyName, override_global_theme(), retain_original_opposing_theme(), update_mode_if_new_appearance_is_different_from_system(), syntax_overrides(), ThemeContent, ThemeFamilyContent, adjust_agent_buffer_font_size() (+58 more)

### Community 240 - "audio: ArrayQueue"
Cohesion: 0.04
Nodes (30): callback_gets_all_samples(), callback_modifies_yielded(), ConstantSampleRate, ConstantSampleRate<S>, continues_after_history(), InspectBuffer, InspectBuffer<N, S, F>, keeps_correct_amount_of_seconds() (+22 more)

### Community 241 - "settings_content: settings_content/src/agent.rs"
Cohesion: 0.03
Nodes (59): agent_config_option_value_merge_replaces_existing_value(), agent_config_option_value_serializes_value_id_as_string(), agent_profile_tools_schema(), AgentConfigOptionValue, Boolean, AgentProfileContent, AgentSettingsContent, AutoCompactSettingsContent (+51 more)

### Community 242 - "agent_ui: agentmodelicon"
Cohesion: 0.04
Nodes (39): AgentModelIcon, Named, Path, AgentModelId, AgentModelInfo, AgentModelList, Flat, Grouped (+31 more)

### Community 243 - "prompt_store: BuiltInPrompt"
Cohesion: 0.04
Nodes (65): BuiltInPrompt, CommitMessage, init(), MetadataCache, PromptId, BuiltIn, User, PromptIdV1 (+57 more)

### Community 244 - "auto_update_helper: beginpaint"
Cohesion: 0.03
Nodes (24): Args, init_log(), main(), parse_args(), run(), show_error(), test_parse_args(), test_parse_args_preserves_launch_arguments() (+16 more)

### Community 245 - "deployment: deploy.rs"
Cohesion: 0.04
Nodes (74): collect_local(), collect_remote(), detect_conflict(), download_to(), ensure_remote_dir(), fetch_for_compare(), local_meta(), LocalMeta (+66 more)

### Community 246 - "editor: ColumnarSelectionRows"
Cohesion: 0.05
Nodes (35): ColumnarSelectionRows, SelectMode, All, Character, Line, Word, SelectNextOrigin, Caret (+27 more)

### Community 247 - "rope: rope.rs"
Cohesion: 0.05
Nodes (32): Bytes, Bytes<'a>, Chunk, ChunkBitmaps, Chunks, Chunks<'a>, ChunkWithBitmaps, ChunkWithBitmaps<'a> (+24 more)

### Community 248 - "agent_ui: agentsessionlistrequest"
Cohesion: 0.04
Nodes (44): deserialize_external_agent_id(), NewExternalAgentThread, Cancel, AcpThreadImportOnboarding, .KEY, AgentEntry, AgentImportStatus, Error (+36 more)

### Community 249 - "shell_command_parser: ast"
Cohesion: 0.04
Nodes (101): extract_commands(), test_and_operator(), test_append_output_and_error_redirect(), test_append_output_and_error_redirect_to_dev_null_skipped(), test_append_redirect_to_dev_null_skipped(), test_arithmetic_expansion_doubly_nested_command_substitution(), test_arithmetic_expansion_inside_double_quotes(), test_arithmetic_expansion_nested_backtick_substitution() (+93 more)

### Community 250 - "agent: test_resolve_path()"
Cohesion: 0.04
Nodes (61): test_resolve_path(), agent_edit_buffer(), apply_char_operations(), EditPipeline, EditPipelineEntry, ResolvingOldText, StreamingNewText, EditSession (+53 more)

### Community 251 - "context_server: create_initialize_response()"
Cohesion: 0.03
Nodes (102): create_initialize_response(), BlobResourceContents, CallToolParams, CallToolResponse, CancelledParams, ClientCapabilities, ClientNotification, Cancelled (+94 more)

### Community 252 - "language_core: .adapter_language_name()"
Cohesion: 0.03
Nodes (18): Capture, Optional, Required, ParseableLanguage, populate_capture_indices(), Named, LanguageId, LanguageName (+10 more)

### Community 253 - "xtask: container"
Cohesion: 0.07
Nodes (87): create_sentry_release(), run_autofix(), scheduled_compliance_check(), danger_job(), deploy(), deploy_collab(), publish(), style() (+79 more)

### Community 254 - "gpui_macos: ns_string()"
Cohesion: 0.02
Nodes (15): ns_string(), A11yActionHandler, A11yActivationHandler, display_id_for_screen(), display_id_for_screen_handles_missing_metadata(), FrameSource, DisplayLink, Metal (+7 more)

### Community 255 - "editor: AcceptInlayHint"
Cohesion: 0.04
Nodes (72): GotoDefinitionKind, Declaration, Implementation, Symbol, Type, assert_inlay_hint_acceptance_available(), DefinitionLocations, document_link_target_to_hover_link() (+64 more)

### Community 256 - "settings: ActivePaneModifiers"
Cohesion: 0.05
Nodes (20): imported_reduce_motion(), imported_soft_wrap_indent(), skip_default(), test_import_command_palette_history(), test_import_disabled_globs_extends_inherited_patterns(), test_import_disabled_globs_omits_empty_results(), test_import_file_exclusions(), test_import_file_exclusions_without_usable_patterns() (+12 more)

### Community 257 - "eval_cli: agentconnection_as"
Cohesion: 0.03
Nodes (60): agent_sidebar_side(), init(), init_language_model_settings(), maybe_backfill_editor_layout(), PARALLEL_AGENT_LAYOUT_BACKFILL_KEY, rerun_rules_to_skills_migration(), setup_backfill_test(), show_rules_to_skills_migration_toast() (+52 more)

### Community 258 - "git_ui: commitsummary"
Cohesion: 0.04
Nodes (36): activate_existing_blame_editor(), deploy_blame_entry_context_menu(), existing_blame_editor(), format_blame_text(), GIT_BLAME_MAX_AUTHOR_CHARS_DISPLAYED, GitBlameRenderer, GitBlameStatus, open_buffer_blame_at_revision() (+28 more)

### Community 259 - "editor: TabEdit"
Cohesion: 0.06
Nodes (30): TabEdit, indent_adjustment_for(), init_test(), init_wrap_test(), LineFragmentBuilder, push_isomorphic(), SumTree<Transform>, SumTreeExt (+22 more)

### Community 260 - "editor: assert_hovers()"
Cohesion: 0.05
Nodes (49): assert_hovers(), Editor, FallbackHoverState, get_hover_popover_delay(), handle_hover_requests(), hide_hover(), hide_popovers(), hover() (+41 more)

### Community 261 - "repl: init()"
Cohesion: 0.06
Nodes (5): init(), NotebookEditor, NotebookItem, test_run_cell_with_missing_interpreter_shows_error(), test_save_goes_through_the_project()

### Community 262 - "agent: SandboxPermissions"
Cohesion: 0.05
Nodes (62): SandboxPermissions, all_access_covers_any_concrete_write(), any_host_grant_covers_specific_and_any_host(), covers(), db_form_drops_unparsable_persisted_hosts(), db_form_preserves_any_host_and_write_all(), effective(), effective_any_host_subsumes_specific_hosts() (+54 more)

### Community 263 - "cloud_llm_client: LanguageModelToolResultContent"
Cohesion: 0.03
Nodes (61): LanguageModelToolResultContent, WebSearchToolOutput, Error, Success, AcceptEditPredictionBody, CLIENT_SUPPORTS_STATUS_MESSAGES_HEADER_NAME, CLIENT_SUPPORTS_STATUS_STREAM_ENDED_HEADER_NAME, CLIENT_SUPPORTS_X_AI_HEADER_NAME (+53 more)

### Community 264 - "editor: columnar_selection.rs"
Cohesion: 0.07
Nodes (65): assert_batch_points(), assert_columnar_action_work(), assert_plain_rows(), assert_row_boundaries(), BoundaryCursor, BoundaryQuery, Check, Next (+57 more)

### Community 265 - "gpui: frame_cost()"
Cohesion: 0.04
Nodes (69): frame_cost(), frame_work_starts_at_the_later_of_signal_and_invalidation(), a_hang_outliving_the_frame_deadline_keeps_its_frame_association(), a_journal_gap_suppresses_budget_inference_but_retains_observed_hangs(), a_slow_frame_with_little_foreground_spend_is_not_an_incident(), active_use_counts_time_within_a_minute_of_deliberate_input(), ACTIVE_USE_WINDOW, ActiveUse (+61 more)

### Community 266 - "github-community-pr-board.py: functools"
Cohesion: 0.05
Nodes (74): closing_issues_in_repository(), compute_contributor(), compute_signals(), compute_size_bucket(), connection_nodes(), count_interested_people(), current_field_values(), engaged_users() (+66 more)

### Community 267 - "tasks_ui: Context"
Cohesion: 0.05
Nodes (56): spawn_task_or_modal(), emulate_task_schedule(), open_spawn_tasks(), Picker<TasksModalDelegate>, query(), ShowAttachModal, task_names(), TaskOverrides (+48 more)

### Community 268 - "editor: Match"
Cohesion: 0.05
Nodes (5): Match, DiffStyleControls, LhsEditor, SplitLhsDiffHunkRenderer, SplittableEditor

### Community 269 - "sandbox: windows_wsl.rs"
Cohesion: 0.04
Nodes (91): bad_request_errors_do_not_claim_sandboxing_is_unavailable(), build_bwrap_args(), bwrap_allows_network_when_requested(), bwrap_binds_explicit_writable_file_paths(), bwrap_blocks_wsl_interop_by_default(), bwrap_blocks_wsl_interop_even_with_fs_write(), bwrap_denies_network_by_default(), bwrap_does_not_forward_windows_specific_env() (+83 more)

### Community 270 - "settings_content: settings_content/src/project.r"
Cohesion: 0.03
Nodes (85): AutoFetchSettingsContent, BinarySettings, BlameSettings, BranchPickerSettingsContent, CommitViewSettingsContent, ContextServerCommand, ContextServerSettingsContent, Extension (+77 more)

### Community 271 - "collab: alias"
Cohesion: 0.03
Nodes (35): Contact, Accepted, Incoming, Outgoing, CreatedChannelMessage, db_status_to_proto(), ExtensionVersionConstraints, Invite (+27 more)

### Community 272 - "docs_preprocessor: bookitem"
Cohesion: 0.05
Nodes (79): absolute_docs_url(), add_llms_markdown_directive(), add_markdown_alternate_link(), copy_markdown_sources(), docs_page_description(), docs_page_metadata(), docs_pages(), docs_path() (+71 more)

### Community 273 - "collab: .optional_room_transaction()"
Cohesion: 0.07
Nodes (6): JoinRoom, Database, Database, TransactionGuard, TransactionGuard<T>, TransactionHandle

### Community 274 - "deployment: dirs.rs"
Cohesion: 0.03
Nodes (42): cache_dir(), config_dir(), global_servers_file(), HOME, home_override(), project_key(), state_dir(), trusted_hosts_file() (+34 more)

### Community 275 - "livekit_client: audio_mixer"
Cohesion: 0.04
Nodes (23): NEXT_WAYLAND_SHARE_ID, PIPEWIRE_TIMEOUT_S, start_wayland_desktop_capture(), WaylandScreenCaptureStream, _AssertSend, AudioMixerSource, AudioStack, AudioStream (+15 more)

### Community 276 - "zed: CliRequest"
Cohesion: 0.05
Nodes (70): CliRequest, DebugState, Open, SetOpenBehavior, CliResponseSink, handle_open_request(), agent_url_with_prompt(), assert_ssh_parse() (+62 more)

### Community 277 - "google_ai: google_ai/src/completion.rs"
Cohesion: 0.04
Nodes (82): completed_stream_emits_stop_once(), convert_usage(), GoogleEventMapper, incomplete_stream_reports_unexpected_end(), into_google(), into_google_nests_tool_result_images_in_function_response(), into_google_replays_signed_thinking_as_thought_text_part(), into_google_requests_thought_summaries_and_thinking_level() (+74 more)

### Community 278 - "paths: paths/src/paths.rs"
Cohesion: 0.04
Nodes (71): add_vscode_user_data_paths(), agents_file(), APP_NAME, APP_NAME_LOWERCASE, CONFIG_DIR, config_dir(), copilot_dir(), crashes_dir() (+63 more)

### Community 279 - "util: util/src/paths.rs"
Cohesion: 0.03
Nodes (63): case_group_key(), compare_lowercase(), compare_numeric_segments(), compare_paths(), compare_paths_case_semi_sensitive(), compare_paths_with_dots(), compare_paths_with_same_name_different_extensions(), compare_rel_paths() (+55 more)

### Community 280 - "vim: ClipboardItem"
Cohesion: 0.05
Nodes (26): ClipboardItem, Mark, Buffer, Local, Path, MarkLocation, Buffer, Path (+18 more)

### Community 281 - "command_palette: CommandInterceptResult"
Cohesion: 0.06
Nodes (29): CommandInterceptResult, CommandPaletteFilter, GlobalCommandPaletteInterceptor, init(), Command, CommandPalette, CommandPaletteDelegate, CommandUsage (+21 more)

### Community 282 - "copilot_chat: src/copilot_chat.rs"
Cohesion: 0.03
Nodes (54): CENTS_PER_DOLLAR, COPILOT_API_VERSION, COPILOT_OAUTH_ENV_VAR, DEFAULT_COPILOT_API_ENDPOINT, deserialize_models_skip_errors(), DisabledAi, FunctionChunk, FunctionContent (+46 more)

### Community 283 - "git_ui: CommitDetails"
Cohesion: 0.05
Nodes (19): CommitDetails, build_buffer(), build_buffer_diff(), CommitDiffAddon, CommitView, CommitViewOptions, CommitViewToolbar, folded_files() (+11 more)

### Community 284 - "gpui_macos: gpui_macos/src/display_link.rs"
Cohesion: 0.04
Nodes (58): CVDisplayLink, CVDisplayLinkCreateWithActiveCGDisplays(), CVDisplayLinkOutputCallback, CVDisplayLinkRelease(), CVDisplayLinkRetain(), CVDisplayLinkSetCurrentCGDisplay(), CVDisplayLinkSetOutputCallback(), CVDisplayLinkStart() (+50 more)

### Community 285 - "gpui: .on_action()"
Cohesion: 0.08
Nodes (8): Context, Context<'a, T>, Context<'_, T>, invoke_listener(), invoke_observer(), invoke_observer_in(), invoke_subscriber(), invoke_subscriber_in()

### Community 286 - "settings_content: settings_content/src/theme.rs"
Cohesion: 0.03
Nodes (63): AccentContent, BufferLineHeight, Comfortable, Custom, Standard, CodeFade, default_buffer_font_weight(), DEFAULT_DARK_THEME (+55 more)

### Community 287 - "debugger_ui: AddWatch"
Cohesion: 0.06
Nodes (17): SELECT_DEBUGGER_LABEL, SharedString, DraggedTabPreview, UNKNOWN_BYTE, DapEntry, Scope, Variable, Watcher (+9 more)

### Community 288 - "deployment: AsyncTlsConnector"
Cohesion: 0.04
Nodes (26): Capabilities, CHUNK, civil_from_days(), connect(), days_from_civil(), format_ftp_time(), Ftp, ftp_time_round_trip() (+18 more)

### Community 289 - "gpui_macos: cfdatagetbyteptr"
Cohesion: 0.03
Nodes (28): always_use_command_layout(), BACKSPACE_KEY, BACKSPACE_KEY_EQUIVALENT, chars_for_modified_key(), CMD_MOD, DELETE_KEY_EQUIVALENT, ENTER_KEY, ESCAPE_KEY (+20 more)

### Community 290 - "gpui: AnimationExample"
Cohesion: 0.03
Nodes (33): AnimationExample, bounce(), animate(), AnimatedSvgView, CappedView, Easing, EaseInOut, EaseOutQuint (+25 more)

### Community 291 - "terminal: hyperlinks.rs"
Cohesion: 0.04
Nodes (67): absolute_file_iri(), alacritty_bugs_with_two_columns(), build_term_from_test_lines(), build_test_term(), cargo_hyperlink_benchmark(), CheckHyperlinkMatch, CheckHyperlinkMatch<'a>, colons_galore() (+59 more)

### Community 292 - "deployment: authorization"
Cohesion: 0.05
Nodes (38): norm_remote(), Auth, Basic, Digest, None, body_stream(), ChallengeParams, CHUNK (+30 more)

### Community 293 - "markdown: cowstr"
Cohesion: 0.04
Nodes (81): test_table_checkbox_detection(), assert_code_block_does_not_emit_links(), CodeBlockKind, Fenced, FencedLang, FencedSrc, Indented, CodeBlockMetadata (+73 more)

### Community 294 - "gpui: .keyboard_mapper()"
Cohesion: 0.04
Nodes (41): DummyKeyboardMapper, PlatformKeyboardMapper, AsKeystroke, display_key(), display_modifiers(), InvalidKeystrokeError, is_printable_key(), KeybindingKeystroke (+33 more)

### Community 295 - "sandbox_linux: broker.rs"
Cohesion: 0.06
Nodes (43): AddFd, Broker, handle(), IOPRIO_WHO_PGRP, IOPRIO_WHO_PROCESS, is_controlling_terminal_device(), is_deleted(), is_directory_entry() (+35 more)

### Community 296 - "sandbox_linux: OwnedFd"
Cohesion: 0.06
Nodes (66): serve_tcp(), serve_udp(), add_local_route(), BRIDGE_TIMEOUT, Caller, canonical(), configure_namespace(), connect_and_wait() (+58 more)

### Community 297 - "gpui_macos: appleactivity"
Cohesion: 0.04
Nodes (33): bool, BoolExt, .ENCODING, .ENCODING_REF, APP_CLASS, APP_DELEGATE_CLASS, did_finish_launching(), get_mac_platform() (+25 more)

### Community 298 - "sandbox_linux: asfd_as"
Cohesion: 0.04
Nodes (60): main(), ATTACH_FD, bring_up_loopback(), drop_capabilities(), exit_code(), HELPER_FLAG, install_terminate_handler(), on_terminate() (+52 more)

### Community 299 - "acp_tools: AcpTools"
Cohesion: 0.05
Nodes (39): AcpTools, AcpToolsEvent, AcpToolsToolbarItemView, collapsed_params_md(), expanded_params_md(), init(), MessageType, Notification (+31 more)

### Community 300 - "recent_projects: AddToWorkspace"
Cohesion: 0.06
Nodes (41): assert_pinned_to_bottom(), assert_scroll_top_is(), assert_selected_entry_is_recent_project(), build_picker(), default_open_in_new_window(), delete_recent_project_in_picker(), deleting_last_recent_project_preserves_scroll_position(), deleting_middle_recent_project_preserves_scroll_position() (+33 more)

### Community 301 - "project: ContextServer"
Cohesion: 0.06
Nodes (24): ContextServer, ContextServerConfiguration, Custom, Extension, Http, ContextServerFactory, ContextServerSettingsEntry, ContextServerState (+16 more)

### Community 302 - "search: active_match_index"
Cohesion: 0.11
Nodes (73): assert_all_highlights_match_query(), contains_uppercase(), enable_search_on_type(), init_test(), match_count(), match_texts(), matched_file_names(), perform_incremental_search() (+65 more)

### Community 303 - "deployment: AgentClient"
Cohesion: 0.05
Nodes (39): expand_home(), ConnectHooks, ConnectParams, ExecOutput, InsecureHooks, askpass_env(), auth_keyboard_interactive(), auth_with_agent() (+31 more)

### Community 304 - "deployment: model"
Cohesion: 0.05
Nodes (51): model, AutoUpload, Always, Never, OnExplicitSave, DEFAULT_EXCLUDE_NAMES, GlobalConfig, LogLevel (+43 more)

### Community 305 - "collab: AxumMessage"
Cohesion: 0.03
Nodes (20): CloudflareIpCountryHeader, SystemIdHeader, Executor, Deterministic, Production, AppVersionHeader, build_initial_contacts_update(), ConnectionGuard (+12 more)

### Community 306 - "gpui_apple: Bounds_ScaledPixels"
Cohesion: 0.06
Nodes (74): add_curve_winding(), background_color(), blurred_rounded_rect(), clip_to_mask(), ClippedVertex, position, unit_vertex, corner_dash_velocity() (+66 more)

### Community 307 - "which_key: humanize_action_name()"
Cohesion: 0.07
Nodes (54): humanize_action_name(), advance_past_pending_timeout(), assert_chord_progress_dismisses_popover(), binding(), counted_bindings(), indicator_snapshot(), IndicatorRenderState, IndicatorSnapshot (+46 more)

### Community 308 - "language_model_core: chat_completion.rs"
Cohesion: 0.05
Nodes (66): accumulates_tool_calls_without_indices_by_id(), adopts_a_late_id_for_the_call_at_the_same_index(), adopts_a_late_id_for_the_only_anonymous_call_without_an_index(), AmbiguousToolCallChunk, chat_refusals_preserve_reason_without_explanation(), ChatCompletionEventMapper, ChoiceDelta, chunk_without_choices_or_usage_maps_to_no_events() (+58 more)

### Community 309 - "editor: accepteditprediction"
Cohesion: 0.06
Nodes (45): EditPrediction, Edit, MoveOutside, MoveWithin, accept_completion(), assert_edit_prediction_diff_popover_avoids_right_occluders(), assert_editor_active_edit_completion(), assert_editor_active_move_completion() (+37 more)

### Community 310 - "agent_ui: ContentBlockView"
Cohesion: 0.05
Nodes (11): PermissionSelection, Choice, SelectedPatterns, ToolCallLayout, Embedded, Floating, Standalone, ContextCompaction (+3 more)

### Community 311 - "src: CONTRIBUTING.md"
Cohesion: 0.02
Nodes (78): AI Policy, Before you start, Bird's-eye view of Zed, Contributing to Zed, Packaging Zed, Resources, Sending changes, Things we will (probably) not merge (+70 more)

### Community 312 - "agent_ui: ArchiveListItem"
Cohesion: 0.06
Nodes (16): ArchiveListItem, BucketSeparator, Entry, ProjectPickerDelegate, Header, ProjectPickerModal, ThreadFilter, All (+8 more)

### Community 313 - "component_preview: ActivePageId"
Cohesion: 0.05
Nodes (21): ActivePageId, ComponentPreview, ComponentPreviewPage, init(), PreviewEntry, AllComponents, Component, SectionHeader (+13 more)

### Community 314 - "deployment: Server"
Cohesion: 0.04
Nodes (41): Server, WebdavAuth, Basic, Digest, None, accounts(), Credentials, init_keyring() (+33 more)

### Community 315 - "dev_container: DevContainerError"
Cohesion: 0.07
Nodes (31): DevContainerError, CommandFailed, ContainerNotValid, DevContainerNotFound, DevContainerParseFailed, DevContainerScriptsFailed, DevContainerTemplateApplyFailed, DevContainerValidationFailed (+23 more)

### Community 316 - "editor: FoldChunks"
Cohesion: 0.07
Nodes (39): FoldChunks, assert_max_tab_chunk_bitmaps(), count_chars_in_byte_range(), MAX_EXPANSION_COLUMN, MAX_ROW_SCAN_CHUNKS, MAX_TABS, SPACES, TabChunks (+31 more)

### Community 317 - "workspace: .supported_options()"
Cohesion: 0.07
Nodes (18): AnyView, Direction, Next, Prev, Entity<T>, FilteredSearchRange, Default, Selection (+10 more)

### Community 318 - "editor: AcceptNextLineEditPrediction"
Cohesion: 0.08
Nodes (6): edit_prediction_edit_text(), edit_prediction_fallback_text(), Editor, .EDIT_PREDICTION_POPOVER_PADDING_X, .EDIT_PREDICTION_POPOVER_PADDING_Y, EditPredictionState

### Community 319 - "html_to_markdown: fetch_url_content()"
Cohesion: 0.05
Nodes (27): fetch_url_content(), HtmlElement, inline_elements(), convert_html_to_markdown(), CodeHandler, HeadingHandler, ListHandler, ParagraphHandler (+19 more)

### Community 320 - "dev_container: DevContainerMessage"
Cohesion: 0.06
Nodes (33): DevContainerMessage, ConfirmWriteDevContainer, ErrorRetrievingTemplates, FailedToWriteTemplate, FeaturesRetrieved, FeaturesSelected, GoBack, NeedConfirmWriteDevContainer (+25 more)

### Community 321 - "markdown: empty_render_image_does_not_pa"
Cohesion: 0.06
Nodes (54): empty_render_image_does_not_panic(), ImageId, RenderImage, ParsedSvg, MERMAID_ZOOM_DEBOUNCE, MermaidZoomCallback, build_mermaid_theme(), CachedMermaidDiagram (+46 more)

### Community 322 - "settings_content: ModelMode"
Cohesion: 0.04
Nodes (76): ModelMode, Adaptive, Default, Thinking, get_max_tokens(), Model, AllLanguageModelSettingsContent, AmazonBedrockSettingsContent (+68 more)

### Community 323 - "sandbox_linux: seccomp.rs"
Cohesion: 0.03
Nodes (83): Action, ArgumentHasBits, ArgumentIn, ArgumentNonNull, Return, ALLOW, ALWAYS_NOTIFY, AND_OPCODE (+75 more)

### Community 324 - "gpui_ios: allocated"
Cohesion: 0.03
Nodes (19): MetalDisplayLinkDelegate, MetalDisplayLinkDelegateIvars, MetalFrameSource, application_callbacks_are_not_shared_between_platforms(), browsing_activity_delivers_url_without_losing_query_or_fragment(), IosApplicationState, launch_callback_runs_once_and_can_register_callbacks(), lifecycle_callback_replacement_survives_dispatch() (+11 more)

### Community 325 - "language_model: authenticateerror"
Cohesion: 0.06
Nodes (28): BuiltinProviderHidingFn, ConfigurationError, ModelNotFound, NoProvider, ProviderNotAuthenticated, Event, AddedProvider, CommitMessageModelChanged (+20 more)

### Community 326 - "rpc: FakeServer"
Cohesion: 0.05
Nodes (19): FakeServer, FakeServerState, Model, Relation, Room, Receipt, ConnectionId, ConnectionState (+11 more)

### Community 327 - "http_client: parse_authorization_header()"
Cohesion: 0.04
Nodes (33): parse_authorization_header(), validate_header(), Request, Choice, Secret, cancel_reader(), fetch(), global_fetch() (+25 more)

### Community 328 - "git_ui: ButtonStates"
Cohesion: 0.06
Nodes (18): ButtonStates, init_test(), ProjectDiff, ProjectDiffToolbar, test_deploy_at_respects_active_repository_selection(), test_diff_action_follows_diff_base_setting(), test_excerpts_are_ordered_by_path(), test_excerpts_splitting_after_restoring_the_middle_excerpt() (+10 more)

### Community 329 - "gpui: AnyWindowHandle"
Cohesion: 0.05
Nodes (11): AnyWindowHandle, Entity<T>, Entity<V>, test_memory_warning_subscriptions(), test_mobile_lifecycle_subscriptions(), test_simulate_path_prompt_cancellation(), test_simulate_path_prompt_response(), test_system_notification_body_and_action_responses() (+3 more)

### Community 330 - "gpui: key_dispatch.rs"
Cohesion: 0.06
Nodes (46): capture_observed_keystrokes(), dispatch_altgr_key_down(), DispatchNode, DispatchNodeId, DispatchResult, DispatchTree, KeyListener, ModifiersChangedListener (+38 more)

### Community 331 - "xtask: extension_tests"
Cohesion: 0.06
Nodes (50): run_bump_patch_version(), bump_main(), bump_zed_version(), create_preview_branch(), promote_to_stable(), resolve_versions(), ResolvedOutputs, bump_extension_version() (+42 more)

### Community 332 - "eval_cli: function"
Cohesion: 0.06
Nodes (40): function, eval_cli_with_log_command(), _age_days(), prune_artifacts(), load_json(), safe_extract_archive(), utc_now(), write_json() (+32 more)

### Community 333 - "gpui_web: router()"
Cohesion: 0.05
Nodes (30): router(), AppleDispatcher, calloop_works(), ChannelError, LinuxDispatcher, MIN_THREADS, PriorityQueueCalloopSender, PriorityQueueCalloopSender<T> (+22 more)

### Community 334 - "dev_container: devcontainer_json.rs"
Cohesion: 0.04
Nodes (64): array_lifecycle_command_runs_directly(), ContainerBuild, deserialize_app_port(), deserialize_devcontainer_json(), deserialize_devcontainer_json_from_value(), deserialize_devcontainer_json_to_value(), deserialize_mount_definition(), deserialize_mount_definitions() (+56 more)

### Community 335 - "editor: cursor_animation.rs"
Cohesion: 0.07
Nodes (49): adding_cursor_inherits_previous_newest_geometry_without_removing_it(), ANIMATION_LENGTH_SECONDS, ANIMATION_RESET_THRESHOLD_SECONDS, AnimationPoint, assert_converges_to(), bounds(), bounds_with_width(), Corner (+41 more)

### Community 336 - "editor: .map()"
Cohesion: 0.06
Nodes (19): init_test(), InlayChunks<'a>, InlayOffset, InlayPoint, InlayPointCursor, InlaySnapshot, MultiBufferOffset, Point (+11 more)

### Community 337 - "sandbox: sandbox.rs"
Cohesion: 0.04
Nodes (55): apply_proxy_env(), apply_proxy_env_points_vars_at_proxy_and_blanks_no_proxy(), canonical_path_resolves_a_symlinked_parent(), CommandAndArgs, create_missing_dirs(), creates_missing_dirs_and_discard_removes_only_those(), existing_dir_is_left_alone_and_not_removed_on_discard(), first_nonempty_env_value() (+47 more)

### Community 338 - "tabular_data_preview: render_table.rs"
Cohesion: 0.05
Nodes (36): RowRenderMechanism, UniformList, VariableList, DisplayToDataMapping, apply_and_snapshot(), build_engine(), calculate_available_filters(), FilterEntry (+28 more)

### Community 339 - "gpui_linux: Atom"
Cohesion: 0.05
Nodes (35): Clipboard, CLIPBOARD, ClipboardData, ClipboardKind, Clipboard, Primary, Secondary, Error (+27 more)

### Community 340 - "language_models: CacheTtl"
Cohesion: 0.05
Nodes (23): add_message_content_part(), API_KEY_ENV_VAR, API_KEY_ENV_VAR_NAME, cache_control(), into_open_router(), language_model(), open_router_session_id(), OpenRouterLanguageModelProvider (+15 more)

### Community 341 - "deployment: Engine"
Cohesion: 0.07
Nodes (15): Engine, EngineSettings, FsEvent, Changed, Removed, Renamed, LogLine, OpenBehavior (+7 more)

### Community 342 - "editor: assert_blame_rows()"
Cohesion: 0.05
Nodes (36): assert_blame_rows(), blame_entry(), BlameRenderer, build_blame_entry_sum_tree(), gen_blame_entries(), GitBlame, GitBlameEntry, GitBlameKind (+28 more)

### Community 343 - "gpui_macos: caller_action_ids_do_not_colli"
Cohesion: 0.04
Nodes (21): caller_action_ids_do_not_collide_with_transport_action_ids(), response_action_id(), ResponseCallback, SystemNotificationState, DelegateIvars, NotificationCenter, NotificationCommand, Dismiss (+13 more)

### Community 344 - "image_viewer: displayed_render_image()"
Cohesion: 0.06
Nodes (20): displayed_render_image(), displayed_source_id(), DisplayedImage, draw_window(), image_is_cached(), ImageContentElement, ImageView, ImageViewEvent (+12 more)

### Community 345 - "util: darwin.rs"
Cohesion: 0.07
Nodes (43): Child, Command, create_pipe(), cvt_nz(), environ, invalid_input_error(), open_dev_null(), posix_spawn_file_actions_addchdir_np() (+35 more)

### Community 346 - "workspace: ActivePaneDecorator"
Cohesion: 0.06
Nodes (19): ActivePaneDecorator, ActivePaneDecorator<'a>, LeaderDecoration, Member, Axis, Pane, PaneAxis, PaneGroup (+11 more)

### Community 347 - "settings_ui: contextservercommand"
Cohesion: 0.07
Nodes (66): ContextServerStatus, Authenticating, AuthRequired, ClientSecretRequired, Error, Running, Starting, Stopped (+58 more)

### Community 348 - "editor: InsertSnippet"
Cohesion: 0.06
Nodes (16): InsertSnippet, ToggleComments, MultiBufferOffsetUtf16, AutocloseRegion, comment_delimiter_for_newline(), documentation_delimiter_for_newline(), Editor, is_list_prefix_row() (+8 more)

### Community 349 - "agent_servers: AvailableCommandsUpdate"
Cohesion: 0.04
Nodes (49): byte_limit_is_bounded_and_does_not_poison_later_creations(), InboundSessionNotification, InboundSessionUpdate, AgentMessage, AgentMessageChunk, AgentThought, AgentThoughtChunk, AvailableCommands (+41 more)

### Community 350 - "path: test_deserialize_manifest_with"
Cohesion: 0.04
Nodes (23): test_deserialize_manifest_with_windows_separators(), PathExt, T, try_from_bytes(), &'a RelPath, Arc<RelPath>, rel_path_buf(), RelPathAncestors (+15 more)

### Community 351 - "gpui_macos: .set_initial_windowing()"
Cohesion: 0.03
Nodes (10): activation_policy_for(), MacPlatform, path_from_objc(), ActivationPolicy, Accessory, Regular, WindowingRequest, Headless (+2 more)

### Community 352 - "project: create_gpui_image()"
Cohesion: 0.06
Nodes (23): create_gpui_image(), Entity<LocalImageStore>, Entity<RemoteImageStore>, ImageColorInfo, ImageFormat, Raster, Svg, ImageId (+15 more)

### Community 353 - "src: extensions.md"
Cohesion: 0.03
Nodes (67): Available Extensions, Debugger Extensions, Defining Debug Locators, Defining Debugger Extensions, Testing, Debugging your Rust extension, Developing an Extension Locally, Developing Extensions {#developing-extensions} (+59 more)

### Community 354 - "rope: ArrayString"
Cohesion: 0.07
Nodes (27): is_utf8_char_boundary(), Bitmap, char_offsets(), char_offsets_with_end(), Chunk, .MASK_BITS, ChunkSlice, ChunkSlice<'a> (+19 more)

### Community 355 - "xtask: cargoopt"
Cohesion: 0.04
Nodes (61): Args, SchemaType, IconTheme, Project, Theme, Args, CliCommand, CheckWorkflows (+53 more)

### Community 356 - "acp_thread: .create_terminal()"
Cohesion: 0.04
Nodes (28): create_terminal_entity(), disable_pagers_through_env(), DisplayCommand, LegacyCaption, Reported, Unknown, DisplayTerminalOutput, DisplayTerminalPatch (+20 more)

### Community 357 - "editor: FoldAtLevel"
Cohesion: 0.07
Nodes (4): FoldAtLevel, Editor, EditorSnapshot, GutterDimensions

### Community 358 - "git_ui: branch_mode.rs"
Cohesion: 0.08
Nodes (18): BranchBase, BranchList, ChangesMode, Branch, Uncommitted, DefaultBranch, Found, Loading (+10 more)

### Community 359 - "http_proxy: proxy/connection.rs"
Cohesion: 0.05
Nodes (63): absolute_form_is_rewritten_to_origin_form(), absolute_form_rewrite_keeps_non_default_port_in_host_header(), build_origin_form_request(), ClientStream, Tcp, Unix, CONNECT_TIMEOUT, connect_to_any() (+55 more)

### Community 360 - "scheduler: SessionId"
Cohesion: 0.06
Nodes (13): SessionId, assert_correct_thread(), exclude_wakers_from_trace(), schedule_local_dispatch(), ScheduledRunnable, ScheduledTimer, SchedulerState, SharedRng (+5 more)

### Community 361 - "sandbox_linux: addressfamily"
Cohesion: 0.06
Nodes (31): AccessRequest, Decision, Handler, HandlerId, SandboxPolicy, HelperSetup, PromptId, PromptRequest (+23 more)

### Community 362 - "agent: tools/edit_file_tool.rs"
Cohesion: 0.07
Nodes (62): assert_resolved_path_eq(), DEFAULT_UI_TEXT, EditFileTool, .NAME, EditFileToolInput, init_test(), setup_test(), setup_test_with_fs() (+54 more)

### Community 363 - "text: .fmt()"
Cohesion: 0.04
Nodes (20): ReplicaId, .AGENT, .FIRST_COLLAB_ID, .LOCAL, .LOCAL_BRANCH, .REMOTE_SERVER, ProjectClientState, Collab (+12 more)

### Community 364 - "editor: test_click_on_parameter_inlay_"
Cohesion: 0.06
Nodes (53): test_click_on_parameter_inlay_hint_places_cursor_correctly(), test_inlay_hints_request_timeout(), allowed_hint_kinds_for_editor(), cached_hint_labels(), check_multi_server_identical_hints(), check_none_hint_bias(), Editor, editor_visible_range() (+45 more)

### Community 365 - "extension_host: ExtensionIndex"
Cohesion: 0.07
Nodes (74): ExtensionIndex, ExtensionIndexEntry, ExtensionIndexIconThemeEntry, ExtensionIndexLanguageEntry, ExtensionIndexThemeEntry, MAX_REMOTE_SYNC_ATTEMPTS, MAX_REMOTE_SYNC_RETRY_DELAY, remote_sync_retry_delay() (+66 more)

### Community 366 - "git_hosting_providers: gerrit.rs"
Cohesion: 0.05
Nodes (40): Gerrit, gerrit_instance(), test_build_commit_permalink(), test_build_permalink(), test_build_permalink_with_multi_line_selection(), test_build_permalink_with_single_line_selection(), test_parse_remote_url_given_https_url(), test_parse_remote_url_given_ssh_url() (+32 more)

### Community 367 - "extensions_ui: DevExtensionNotInstalledError"
Cohesion: 0.06
Nodes (19): DevExtensionNotInstalledError, DevExtensionRebuildPickerDelegate, DisplayedExtension, Installed, Remote, ExtensionFetchState, Failed, Fetching (+11 more)

### Community 368 - "project: integration/dynamic_registrati"
Cohesion: 0.09
Nodes (57): all_lsp_data_server_capabilities(), edit_and_receive_change(), fetch_all_lsp_data(), lsp_data_labels(), LspDataLabels, LspRequestCounts, observe_refresh_events(), RecordingProtoClient (+49 more)

### Community 369 - "gpui: whitespace_map"
Cohesion: 0.06
Nodes (43): whitespace_map, space, tab, is_no_action(), is_unbind(), Unbind, binding_is_unbound(), KeyBinding (+35 more)

### Community 370 - "vim: CharClassifier"
Cohesion: 0.06
Nodes (32): CharClassifier, B, Boundary, End, Start, BoundedObject, FuzzyBoundary, Paragraph (+24 more)

### Community 371 - "lints: ui/map_lookup_then_insert.rs"
Cohesion: 0.06
Nodes (70): aliases_of_two_generated_keys(), already_uses_entry_and_modify(), already_uses_entry_insert_entry(), already_uses_entry_key(), already_uses_entry_or_default(), already_uses_entry_or_insert(), already_uses_entry_or_insert_with(), already_uses_entry_or_insert_with_key() (+62 more)

### Community 372 - "net: accept"
Cohesion: 0.04
Nodes (12): init(), UnixListener, BUFFER_SIZE, CLIENT_MESSAGE, SERVER_MESSAGE, UnixSocket, OwnedReadHalf, OwnedWriteHalf (+4 more)

### Community 373 - "agent_ui: build_assist_editor_renderer()"
Cohesion: 0.07
Nodes (27): build_assist_editor_renderer(), EditorInlineAssists, init(), InlineAssist, InlineAssistant, InlineAssistDecorations, InlineAssistGroup, InlineAssistGroupId (+19 more)

### Community 374 - "gpui: example_editor.rs"
Cohesion: 0.06
Nodes (18): cursor_line_and_offset(), Editor, next_boundary(), offset_from_utf16(), offset_to_utf16(), previous_boundary(), range_from_utf16(), range_to_utf16() (+10 more)

### Community 375 - "gpui_platform: AllocationStats"
Cohesion: 0.08
Nodes (65): editor_multi_cursor_input(), editor_render(), editor_render_highlighted(), editor_render_highlighted_minimap(), editor_render_with_editorconfig(), indented_code_text(), init_context(), jetbrains_editorconfig() (+57 more)

### Community 376 - "agent: templates.rs"
Cohesion: 0.07
Nodes (43): contains(), SystemPromptTemplate, .TEMPLATE_NAME, Template, .TEMPLATE_NAME, Templates, test_system_prompt_does_not_render_legacy_zed_rules_section(), test_system_prompt_linux_sandbox_section_omits_tmpdir() (+35 more)

### Community 377 - "debugger_ui: ActiveBreakpointStripMode"
Cohesion: 0.07
Nodes (15): ActiveBreakpointStripMode, Condition, HitCondition, Log, BreakpointEntry, BreakpointEntryKind, DataBreakpoint, ExceptionBreakpoint (+7 more)

### Community 378 - "gpui_apple: metal_renderer.rs"
Cohesion: 0.03
Nodes (31): build_pipeline_state(), Context, GlobalInputIndex, DepthScale, ViewportSize, INSTANCE_BUFFER_ALIGNMENT, MAX_INSTANCE_BUFFER_SIZE, new_renderer() (+23 more)

### Community 379 - "xtask: .when()"
Cohesion: 0.04
Nodes (31): http::request::Builder, HttpRequestExt, RedirectPolicy, FollowAll, FollowLimit, NoFollow, RequestTimeout, BotCommitStep (+23 more)

### Community 380 - "open_ai: ReasoningEffort"
Cohesion: 0.03
Nodes (51): ReasoningEffort, High, Low, Max, Medium, Minimal, None, .OPENAI_COMPATIBLE_SELECTABLE (+43 more)

### Community 381 - "remote_server: AddWorktree"
Cohesion: 0.04
Nodes (5): find_venv_python(), HeadlessProject, prompt_to_proto(), SearchBufferBatch, initial_server_settings_content()

### Community 382 - "http_proxy: SandboxNetworkAccess"
Cohesion: 0.06
Nodes (45): SandboxNetworkAccess, All, None, Restricted, Allowlist, allowlist_allows_member(), ConnectionSlot, MAX_CONCURRENT_CONNECTIONS (+37 more)

### Community 383 - "compliance: compliance/src/git.rs"
Cohesion: 0.05
Nodes (45): automated_change_detects_preview_release_channel(), automated_change_detects_stable_release_channel(), automated_change_detects_version_bump(), automated_change_rejects_trailing_text(), automated_change_rejects_wrong_prefix(), automated_change_returns_none_for_regular_commit(), co_authors_parsed_from_body(), CommitDetails (+37 more)

### Community 384 - "crashes: crasheventresult"
Cohesion: 0.05
Nodes (44): abort_message_address(), abort_message_read_len(), abort_message_read_len_requires_page_rounded_total(), AbortMessageLocation, connect_and_keepalive(), CRASH_HANDLER_CONNECT_TIMEOUT, CRASH_HANDLER_PING_TIMEOUT, CrashInfo (+36 more)

### Community 385 - "git_ui_core: git_ui_core/src/notifications."
Cohesion: 0.05
Nodes (50): format_git_error_toast_message(), git_error_toast_title(), is_non_fast_forward_error(), open_output(), rpc_error_raw_message_from_chain(), show_error_toast(), test_format_git_error_toast_message_prefers_raw_rpc_message(), test_format_git_error_toast_message_prefers_raw_rpc_message_when_wrapped() (+42 more)

### Community 386 - "livekit_client: ConnectionQuality"
Cohesion: 0.05
Nodes (27): ConnectionQuality, Excellent, Good, Lost, Poor, LocalTrack, Audio, Video (+19 more)

### Community 387 - "terminal: mappings/mouse.rs"
Cohesion: 0.04
Nodes (55): grid_point(), grid_point_and_side(), mouse_button_report(), mouse_moved_report(), mouse_report(), MouseButtonCode, LeftButton, LeftMove (+47 more)

### Community 389 - "channel: .link()"
Cohesion: 0.05
Nodes (9): ChannelCreated, ChannelRenamed, ChannelState, ChannelStore, GlobalChannelStore, init(), OpenEntityHandle, Loading (+1 more)

### Community 390 - "git_ui_core: DeleteWorktreeTooltip"
Cohesion: 0.07
Nodes (32): DeleteWorktreeTooltip, deleting_worktree_paths(), init_test(), init_worktree_picker_test(), open_remote_worktree(), picker_contains_worktree(), remove_worktree_command(), repo_contains_worktree() (+24 more)

### Community 391 - "multi_buffer: .forget_transaction()"
Cohesion: 0.06
Nodes (6): History, MultiBuffer, Transaction, History, HistoryEntry, TransactionId

### Community 392 - "repl: clearoutput"
Cohesion: 0.08
Nodes (33): create_execution_view(), Entity<V>, ExecutionView, ExecutionViewFinishedEmpty, ExecutionViewFinishedSmall, init_test(), InputReplyEvent, Output (+25 more)

### Community 393 - "git_ui: CompareWithBranch"
Cohesion: 0.08
Nodes (7): BranchDiff, BranchDiffAddon, BranchDiffToolbar, init_test(), test_branch_diff(), test_branch_diff_action_matches_existing_item_by_base_ref(), test_split_diff_conflict_path_transition_with_dirty_buffer_invalid_anchor_panics()

### Community 394 - "xtask: Concurrency"
Cohesion: 0.05
Nodes (36): Event, Bell, BreadcrumbsChanged, CloseTerminal, OutputReplaced, SelectionsChanged, TitleChanged, Wakeup (+28 more)

### Community 395 - "language_models: ClientTokenProvider"
Cohesion: 0.05
Nodes (18): ClientTokenProvider, CloudLanguageModelProvider, ConfigurationView, init_test(), override_authenticate(), provided_models_surface_disabled_reason(), provider_authenticate_does_not_start_sign_in_when_signed_out(), provider_authenticate_returns_error_when_sign_in_fails() (+10 more)

### Community 396 - "compliance: authorassociation_as_octocraba"
Cohesion: 0.05
Nodes (26): alice_author(), bob_author(), charlie_author(), MockGithubApi, zippy_author(), commit_sha_short_returns_first_8_chars(), CommitSha, build_commit_metadata_query() (+18 more)

### Community 397 - "edit_prediction_types: DataCollectionState"
Cohesion: 0.05
Nodes (12): DataCollectionState, Disabled, Enabled, Unsupported, EditPredictionDelegate, EditPredictionDelegateHandle, EditPredictionIconSet, Entity<T> (+4 more)

### Community 398 - "gpui: .tabbed_windows()"
Cohesion: 0.05
Nodes (18): PlatformOwnedDrag, PlatformOwnedDragState, RestoredInSourceWindow, Suspended, SystemWindowTab, SystemWindowTabController, FrameStateChange, Closed (+10 more)

### Community 399 - "title_bar: AccountButton"
Cohesion: 0.06
Nodes (13): format_stat(), pick_default_screen(), TitleBar, toggle_deafen(), toggle_mute(), toggle_screen_sharing(), apply_title_bar_visibility(), init() (+5 more)

### Community 400 - "gpui_apple: CAMetalDrawable"
Cohesion: 0.08
Nodes (15): build_depth_stencil_state(), display_link_mode_consumes_and_clears_drawables(), DisplayLinkMode, Attached, Detached, InstanceBinding, InstanceBindings, InstanceBuffer (+7 more)

### Community 401 - "copilot: copilot/src/request.rs"
Cohesion: 0.04
Nodes (59): CheckStatus, .METHOD, CheckStatusParams, DidChangeStatus, .METHOD, DidChangeStatusParams, DidFocus, .METHOD (+51 more)

### Community 402 - "debugger_tools: DapLogToolbarItemView"
Cohesion: 0.07
Nodes (15): DapLogToolbarItemView, DapLogView, init(), log_contents(), LogStore, LogStoreEntryIdentifier, LogStoreMessage, MessageKind (+7 more)

### Community 403 - "keymap_editor: keystroke_input.rs"
Cohesion: 0.11
Nodes (50): CLOSE_KEYSTROKE_CAPTURE_END_TIMEOUT, init_dispatch_test(), init_test(), KEY_CONTEXT_VALUE, KeystrokeInputTestHelper, KeystrokeUpdateTracker, simulate_shift_f1(), simulate_standalone_shift() (+42 more)

### Community 404 - "sandbox_linux: examples/end_to_end.rs"
Cohesion: 0.06
Nodes (56): Answer, Allow, Deny, Panic, Remember, answer_the(), Ask, ASK_DELAY (+48 more)

### Community 405 - "terminal_view: alternatescroll"
Cohesion: 0.06
Nodes (49): PathLikeTarget, hover_path_like_target(), init_test(), issue_28339(), issue_28339_remote(), issue_28407_nesting(), issue_28407_siblings(), issue_34027() (+41 more)

### Community 406 - "agent: blameentry"
Cohesion: 0.06
Nodes (35): assert_eq_golden(), Blame, BlameEntry, GIT_BLAME_NO_COMMIT_ERROR, GIT_BLAME_NO_PATH, parse_git_blame(), ParsedCommitMessage, read_test_data() (+27 more)

### Community 407 - "sentry-fetch: collections"
Cohesion: 0.04
Nodes (4): SelectedBreakpointKind, Data, Exception, Source

### Community 408 - "editor: .to_item_events()"
Cohesion: 0.04
Nodes (38): EditorEvent, Blurred, BreadcrumbsChanged, BufferEdited, BufferFoldToggled, BufferRangesUpdated, BuffersEdited, BuffersRemoved (+30 more)

### Community 409 - "edit_prediction_context: editable_context.rs"
Cohesion: 0.08
Nodes (61): anchor_range_for_row_range(), BOUNDARY_SNAP_LINE_COUNT, BRIDGED_GAP_LINE_COUNT, collect_bm25_candidate_context(), collect_bm25_context_ranges(), collect_current_file_context(), collect_cursor_excerpt_context(), collect_edit_history_context() (+53 more)

### Community 410 - "editor: .convert_point_from_companion("
Cohesion: 0.11
Nodes (53): CompanionExcerptPatch, test_split_words(), test_split_words_for_snippet_prefix(), assert_split_content(), assert_split_content_with_widths(), buffer_range_to_base_text_range(), buffer_with_diff(), init_test() (+45 more)

### Community 411 - "editor: bookmark_path()"
Cohesion: 0.08
Nodes (46): bookmark_path(), bookmarked_paths(), BookmarkTestContext, close_bookmarks_editor(), cursor_line_text(), display_row_for(), excerpt_count(), failed_paths() (+38 more)

### Community 412 - "editor: ActiveScrollbarState"
Cohesion: 0.07
Nodes (8): ActiveScrollbarState, Editor, ScrollbarThumbState, Dragging, Hovered, Idle, ScrollManager, SharedScrollAnchor

### Community 413 - "settings: ActionBuildError"
Cohesion: 0.06
Nodes (20): ActionBuildError, BuildError, NotFound, dyn Action, generate_list_of_all_registered_actions(), ActionSequence, can_deserialize_keymap_with_trailing_comma(), keymap_schema_for_unbind_excludes_null_and_unbind_action() (+12 more)

### Community 414 - "mermaid_render: accent_colors.rs"
Cohesion: 0.06
Nodes (36): accent_class_name(), ACCENT_CLASSES, AccentColors, AccentColors<'theme, I>, AccentStackEntry, add_class(), add_to_event(), chart_color_class_name() (+28 more)

### Community 415 - "project: AdaptiveBatcher<T>"
Cohesion: 0.06
Nodes (29): AdaptiveBatcher<T>, FindAllMatchesRequest, FindSearchCandidates, Local, OpenBuffersOnly, Remote, InputPath, is_utf8_prefix() (+21 more)

### Community 416 - "util: disambiguate.rs"
Cohesion: 0.04
Nodes (28): compute_disambiguation_details(), test_deeper_conflict(), test_duplicate_paths_from_multiple_groups(), test_empty_input(), test_identical_items_terminates(), test_mixed_conflicting_and_unique(), test_no_conflicts(), test_simple_two_way_conflict() (+20 more)

### Community 417 - "xtask: gh_workflow"
Cohesion: 0.07
Nodes (50): after_release(), DOCS_CHANNEL, IS_PRERELEASE_ENV, post_to_discord(), publish_winget(), rebuild_releases_page(), RELEASE_BODY, TAG_NAME (+42 more)

### Community 418 - "call: CallCanceled"
Cohesion: 0.08
Nodes (5): ActiveCall, ActiveCallEntity, init(), OneAtATime, test_one_at_a_time()

### Community 419 - "edit_prediction: EditPredictionRejectReason"
Cohesion: 0.05
Nodes (48): EditPredictionRejectReason, Canceled, CurrentPreferred, Discarded, Empty, InterpolatedEmpty, InterpolateFailed, PatchApplyFailed (+40 more)

### Community 420 - "debugger_ui: send_telemetry()"
Cohesion: 0.07
Nodes (15): send_telemetry(), TelemetrySpawnLocation, Custom, Gutter, ScenarioList, AttachMode, ConfigureMode, DebugDelegate (+7 more)

### Community 421 - "language_tools: EnabledLogStreams"
Cohesion: 0.08
Nodes (9): EnabledLogStreams, init(), initialize_new_editor(), log_contents(), LogMenuItem, LspLogToolbarItemView, LspLogView, open() (+1 more)

### Community 422 - "onboarding: go_to_welcome_page()"
Cohesion: 0.05
Nodes (22): go_to_welcome_page(), handle_import_vscode_settings(), init(), Onboarding, show_onboarding_view(), IdType, Existing, New (+14 more)

### Community 423 - "rope: .lines()"
Cohesion: 0.04
Nodes (11): ChunkSummary, DimensionPair, DimensionPair<K, V>, Dimensions<D1, D2, ()>, Lines, OffsetUtf16, Point, PointUtf16 (+3 more)

### Community 424 - "lints: ui/blocking_io_on_foreground.r"
Cohesion: 0.05
Nodes (57): barrier_wait_with_app(), BlockingRenderView, canonicalize_with_app(), child_wait_with_app(), child_wait_with_output_with_app(), command_output_with_app(), command_spawn_with_app(), command_status_with_app() (+49 more)

### Community 425 - "http_client: FetchStep"
Cohesion: 0.05
Nodes (13): FetchStep, Complete, Redirect, FetchTool, .NAME, host_pattern_for_url(), normalize_url(), BlockedHttpClient (+5 more)

### Community 426 - "agent_ui: agent_configuration.rs"
Cohesion: 0.07
Nodes (28): ConfigurationSource, Existing, Extension, ConfigurationTarget, Existing, ExistingHttp, Extension, ConfigureContextServerModal (+20 more)

### Community 427 - "remote_connection: .ask_password()"
Cohesion: 0.06
Nodes (18): AskPassResult, CancelledByUser, Timedout, AskPassSession, dropping_password_request_cancels_prompt(), generate_askpass_script(), PasswordProxy, EncryptedPassword (+10 more)

### Community 428 - "editor: Editor"
Cohesion: 0.05
Nodes (6): Editor, SoftWrap, Bounded, EditorWidth, GitDiff, None

### Community 429 - "vim: Direction"
Cohesion: 0.05
Nodes (30): Direction, Down, Left, Right, Up, register(), test_change_list_delete(), test_change_list_insert() (+22 more)

### Community 430 - "extension: test_debug_adapter_schemas()"
Cohesion: 0.05
Nodes (37): test_debug_adapter_schemas(), CapabilityGranter, extension_manifest(), test_grant_exec(), build_debug_adapter_schema_path(), ContextServerManifestEntry, DebugAdapterManifestEntry, DebugLocatorManifestEntry (+29 more)

### Community 431 - "extension_host: Event"
Cohesion: 0.06
Nodes (17): Event, ExtensionFailedToLoad, ExtensionInstalled, ExtensionsUpdated, ExtensionUninstalled, StartedReloading, ExtensionOperation, Install (+9 more)

### Community 432 - "fs: atomic_replace()"
Cohesion: 0.04
Nodes (19): atomic_replace(), CopyOptions, file_id(), JobEventReceiver, Metadata, path_to_c_string(), RealFs, rename_without_replace() (+11 more)

### Community 433 - "gpui: process_info.rs"
Cohesion: 0.05
Nodes (13): check_notification_lifecycle(), notifications_are_deferred_and_cancelled_on_drop(), observe_power_mode(), observe_thermal_state(), post_notification(), ProcessInfoObserver, thermal_state(), ForegroundExecutor (+5 more)

### Community 434 - "language_models: add_message_content_part()"
Cohesion: 0.06
Nodes (12): add_message_content_part(), authenticate_does_not_fetch_models_when_ai_is_disabled(), ConfigurationView, does_not_fetch_models_when_ai_is_disabled(), enabling_ai_discovers_models_with_stored_credentials(), language_model(), LmStudioLanguageModelProvider, PROVIDER_ID (+4 more)

### Community 435 - "perf: implementation.rs"
Cohesion: 0.05
Nodes (45): CategoryInfo, FailKind, BadMetadata, Profile, Skipped, Triage, VersionMismatch, Importance (+37 more)

### Community 436 - "agent_ui: ActionLogTelemetry"
Cohesion: 0.05
Nodes (16): CatNumberedCodeBlock, full_path_for_empty_project_path(), GeneratingSpinnerElement, highlight_code_runs(), open_markdown_in_workspace(), render_cat_numbered_code_block(), reset_fast_mode_warnings(), ThreadControlsPosition (+8 more)

### Community 437 - "eval_cli: AgentInstallSpec"
Cohesion: 0.05
Nodes (10): add_anthropic_available_models_env(), add_openai_compatible_provider_env(), add_zed_eval_env(), detect_workdir(), patch_command(), populate_context_from_result(), provider_api_env(), ZedAgent (+2 more)

### Community 438 - "eval_cli: ArgumentParser"
Cohesion: 0.08
Nodes (50): _baseline_problems(), command_baseline_list(), command_baseline_record(), command_baseline_show(), _model_slug(), _print_recorded(), _print_refusal(), _record_from_provenance() (+42 more)

### Community 439 - "eval_cli: contextlib"
Cohesion: 0.08
Nodes (47): validate_build_id(), dedupe_preserving_order(), default_namespace(), modal_call_id(), parse_comma_values(), parse_parts(), print_json(), utc_timestamp() (+39 more)

### Community 440 - "edit_prediction_cli: format_prompt.rs"
Cohesion: 0.06
Nodes (48): extract_all_codeblocks(), extract_last_codeblock(), make_example(), resolved_excerpt_ranges_for_format(), run_format_prompt(), TeacherJumpsPrompt, .DEFAULT_RELATED_FILES_BUDGET, .MAX_HISTORY_TOKENS (+40 more)

### Community 441 - "gpui: .or()"
Cohesion: 0.05
Nodes (25): ContextEntry, is_identifier_char(), is_vim_operator_char(), KeyBindingContextPredicate, And, Descendant, Equal, Identifier (+17 more)

### Community 442 - "extension_host: .work_dir()"
Cohesion: 0.04
Nodes (13): convert_request(), convert_response(), dap::TcpArgumentsTemplate, extension::Command, extension::TcpArguments, extension::TcpArgumentsTemplate, ExtensionHttpResponseStream, ExtensionKeyValueStore (+5 more)

### Community 443 - "json_schema_store: $ref"
Cohesion: 0.03
Nodes (68): $ref, description, type, $ref, description, type, $ref, $ref (+60 more)

### Community 444 - "open_path_prompt: DirectoryState"
Cohesion: 0.08
Nodes (30): DirectoryState, Create, List, None, OpenPathDelegate, OpenPathPrompt, build_open_path_prompt(), collect_match_candidates() (+22 more)

### Community 445 - "agent_ui: agent_panel"
Cohesion: 0.07
Nodes (37): fake_worktree_created_at(), record_zed_created_worktree(), stub_agent_connection(), StubAgentServer, StubAgentServer<C>, TestWorkspaceSidebar, AffectedProject, all_open_workspaces() (+29 more)

### Community 446 - "xtask: all"
Cohesion: 0.05
Nodes (44): DependencyKind, Build, Development, Normal, DependencyRequirement, Optional, Required, DependencyTarget (+36 more)

### Community 447 - "project: BinarySettings"
Cohesion: 0.05
Nodes (31): AutoFetchSettings, BlameSettings, BranchPickerSettings, DapBinary, Custom, Default, DapSettings, DiagnosticsSettings (+23 more)

### Community 448 - "collab: Channel"
Cohesion: 0.10
Nodes (13): Channel, ChannelsForUser, InviteMemberResult, MembershipUpdated, Database, max_order(), Database, Database (+5 more)

### Community 449 - "copilot_ui: .status()"
Cohesion: 0.08
Nodes (23): Status, Authorized, Disabled, Error, SignedOut, SigningIn, Starting, Unauthorized (+15 more)

### Community 450 - "edit_prediction_ui: ActivePrediction"
Cohesion: 0.07
Nodes (7): ActivePrediction, FeedbackCompletionProvider, .FAILURE_MODES, RatePredictionsModal, RatePredictionView, RawInput, SuggestedEdits

### Community 451 - "gpui: input()"
Cohesion: 0.09
Nodes (12): input(), InputExample, marked_selection_is_relative_to_inserted_text(), native_selection_and_length_use_utf16(), TextInput, OsAction, Copy, Cut (+4 more)

### Community 452 - "gpui: bench_task_reports_long_task_w"
Cohesion: 0.08
Nodes (19): bench_task_reports_long_task_without_window(), BenchAppContext<'a, 'measurement>, BenchReport, DurationHistogram, foreground_work_excludes_setup_before_trace_scope_starts(), foreground_work_reports_long_task_without_window_draw(), ForegroundWorkSummary, format_duration() (+11 more)

### Community 453 - "language_tools: ServerHealth"
Cohesion: 0.06
Nodes (31): ServerHealth, Error, Ok, Warning, ActiveEditor, health_status(), LanguageServerBinaryStatus, LanguageServerHealthStatus (+23 more)

### Community 454 - "picker_preview: picker_preview.rs"
Cohesion: 0.09
Nodes (21): append_text(), editor_preview(), EditorPreview, EditorPreviewHandle, highlighted_rows(), init_preview(), SearchMatchLineHighlight, show_match() (+13 more)

### Community 455 - "github-check-new-issue-for-duplicates.py: re"
Cohesion: 0.05
Nodes (36): analyze_duplicates(), resolve_matches(), areas_match(), build_comment(), call_claude(), call_claude_tool(), check_team_membership(), _claude_request() (+28 more)

### Community 456 - "bedrock: anytoolchoice_as_bedrockanytoo"
Cohesion: 0.04
Nodes (24): aws_document_to_value(), BedrockError, AccessDenied, InternalServer, Other, RateLimited, ServiceUnavailable, Validation (+16 more)

### Community 457 - "agent: contextserverstatus"
Cohesion: 0.06
Nodes (26): AnyAgentTool, Erased, ContextServerPrompt, ContextServerRegistry, ContextServerRegistryEvent, PromptsChanged, ToolsChanged, ContextServerTool (+18 more)

### Community 458 - "copilot_chat: CopilotChatModel"
Cohesion: 0.06
Nodes (35): Function, Tool, Function, append_reasoning_details_to_response_items(), category_from_copilot_error(), chat_completions_stream_maps_reasoning_data(), compute_thinking_budget(), CopilotResponseMessageMetadata (+27 more)

### Community 459 - "editor: test_client_can_wrap_with_emme"
Cohesion: 0.09
Nodes (54): test_client_can_wrap_with_emmet_abbreviation(), apply_related_actions(), base_indent_for(), build_wrap_target(), confirm_abbreviation(), ELEMENT_NODE_KINDS, enclosing_element_range(), expansion_text() (+46 more)

### Community 460 - "git_ui: ButtonStates"
Cohesion: 0.08
Nodes (7): ButtonStates, init_test(), StagedDiff, StagedDiffToolbar, test_staged_changes_deploy_as_a_separate_staged_diff_item(), test_staged_diff_restores_as_staged_diff(), test_toggle_staged_unstages_from_staged_view()

### Community 461 - "project_panel: DeleteEntryOutcome"
Cohesion: 0.06
Nodes (29): DeleteEntryOutcome, Deleted, Failed, Trashed, explain_move_error(), test_explain_move_error(), already_exists_error(), Change (+21 more)

### Community 462 - "util: truncate_variables()"
Cohesion: 0.04
Nodes (38): truncate_variables(), default(), dev_repo_root(), emoji_regex(), expanded_and_wrapped_usize_range(), extend_sorted(), fs_embed_file_names(), __fs_embed_get() (+30 more)

### Community 463 - "theme: default_colors.rs"
Cohesion: 0.11
Nodes (47): ADDED_COLOR, amber(), black(), blue(), bronze(), brown(), ColorScaleSet, crimson() (+39 more)

### Community 464 - "x_ai_subscribed: src/x_ai_subscribed.rs"
Cohesion: 0.06
Nodes (37): advertised_reasoning_efforts(), authorize_url_uses_public_client_and_s256(), build_authorize_request(), CALLBACK_HOST, CALLBACK_PATH, CALLBACK_PORT, CLIENT_ID, CREDENTIALS_KEY (+29 more)

### Community 465 - "gpui_ios: cgpoint"
Cohesion: 0.07
Nodes (21): allowed_range(), checked_offset(), checked_range(), composed_character_range(), contains_range(), document_range(), insertion_geometry_has_a_visible_caret_width(), native_protocol_smoke_test() (+13 more)

### Community 466 - "edit_prediction: childentriesoptions"
Cohesion: 0.05
Nodes (49): APACHE_2_0_TXT, assert_matches_license(), BSD_0_TXT, canonicalize_license_text(), check_pattern(), detect_license(), init_test(), is_char_count_within_range() (+41 more)

### Community 467 - "file_content: coderresult"
Cohesion: 0.06
Nodes (47): ADDITIONAL_BINARY_HEADERS, analyze_byte_content(), binary_header_lookup_matches_linear_scan(), build_pcm16_wav_bytes(), build_tag_interleaved_binary_bytes(), ByteContent, Binary, Unknown (+39 more)

### Community 468 - "release_channel: should_install_crash_handler()"
Cohesion: 0.05
Nodes (22): should_install_crash_handler(), AppCommitSha, AppVersion, docs_url(), GlobalAppCommitSha, GlobalAppVersion, GlobalReleaseChannel, init() (+14 more)

### Community 469 - "git_ui: ButtonStates"
Cohesion: 0.08
Nodes (3): ButtonStates, UnstagedDiff, UnstagedDiffToolbar

### Community 470 - "zeta_prompt: .tag()"
Cohesion: 0.07
Nodes (62): assemble_patch_from_edits(), assign_tags(), BASE64_URL_SAFE_ALPHABET, build_editable_marker_table(), build_marker_table(), build_marker_table_with_filter(), build_patch_from_spans(), common_prefix_suffix() (+54 more)

### Community 471 - "language_models: provider/open_ai_compatible.rs"
Cohesion: 0.08
Nodes (25): API_KEY_PLACEHOLDER, available_model(), chat_completion_max_tokens_parameter(), chat_completion_max_tokens_parameter_defaults_to_max_completion_tokens(), chat_completion_max_tokens_parameter_uses_max_tokens_when_configured(), chat_completion_reasoning_effort(), chat_completion_reasoning_effort_honors_request_and_configured_effort(), chat_completion_reasoning_effort_omits_missing_effort() (+17 more)

### Community 472 - "sum_tree: Item"
Cohesion: 0.08
Nodes (20): Item, KeyedItem, K, MapEntry, MapEntry<K, V>, MapKey, MapKeyRef, MapKeyRef<'a, K> (+12 more)

### Community 473 - "settings_ui: AnthropicCompatibleAvailableMo"
Cohesion: 0.10
Nodes (40): CompatibleProviderKind, Anthropic, OpenAi, get_or_create_configuration_view(), LlmProviderForm, LlmProviderFormValues, ModelInput, ModelValues (+32 more)

### Community 474 - "audio: apm"
Cohesion: 0.06
Nodes (28): CHANNEL_COUNT, Audio, AudioDeviceInfo, AvailableAudioDevices, BUFFER_SIZE, EchoCanceller, ensure_devices_initialized(), get_available_audio_devices() (+20 more)

### Community 475 - "agent_ui: collect_markdowns()"
Cohesion: 0.07
Nodes (18): collect_markdowns(), compaction_markdowns(), MatchKey, MatchTarget, Editor, Markdown, render_query_input(), ScannedTarget (+10 more)

### Community 476 - "editor: test_setup()"
Cohesion: 0.07
Nodes (5): test_setup(), assert_state_with_diff(), AssertionContextManager, ContextHandle, EditorTestContext

### Community 477 - "remote: .fmt()"
Cohesion: 0.07
Nodes (21): RemoteClientDelegate, connection(), docker_command_error(), DockerConnectionOptions, DockerExecConnection, preserves_argument_boundaries_and_escapes_control_characters(), push_environment(), redact_arguments() (+13 more)

### Community 478 - "gpui: .set_dock_menu()"
Cohesion: 0.06
Nodes (21): key_to_menu_equivalent(), MacPlatformState, Keymap, KeymapVersion, init_app_menus(), Menu, MenuItem, Separator (+13 more)

### Community 479 - "language_models: ConfigurationView"
Cohesion: 0.08
Nodes (8): ConfigurationView, language_model(), merge_settings_into_models(), OllamaLanguageModelProvider, PROVIDER_ID, restart_fetch_models_task_records_error_on_failure(), State, test_merge_settings_preserves_display_names_for_similar_models()

### Community 480 - "markdown: html_parser.rs"
Cohesion: 0.08
Nodes (54): attr_value(), cleanup_html(), consume_children(), consume_paragraph(), extract_html_blockquote(), extract_html_list(), extract_html_table(), extract_styles_from_attributes() (+46 more)

### Community 481 - "settings_ui: settings_ui/src/components.rs"
Cohesion: 0.05
Nodes (11): EnumVariantDropdown, a11y_numeric_value(), a11y_value_to_field_value(), NumberField, NumberField<T>, NumberField<usize>, NumberFieldMode, Edit (+3 more)

### Community 482 - "streaming_diff: src/streaming_diff.rs"
Cohesion: 0.08
Nodes (40): apply_char_operations(), apply_line_operations(), char_ops_to_line_ops(), CharOperation, Delete, Insert, Keep, is_line_end() (+32 more)

### Community 483 - "etw_tracing: bstr"
Cohesion: 0.06
Nodes (37): build_profile_collection(), BUILTIN_PROFILES, cancel_etw_recording(), Command, Cancel, Save, create_wpr(), EtwNotification (+29 more)

### Community 484 - "time_format: cfabsolutetime"
Cohesion: 0.08
Nodes (48): calculate_month_difference(), create_offset_datetime(), CURRENT_LOCALE, format_absolute_date(), format_absolute_date_medium(), format_absolute_time(), format_absolute_timestamp(), format_absolute_timestamp_medium() (+40 more)

### Community 485 - "debugger_ui: Drag"
Cohesion: 0.09
Nodes (10): Drag, MemoryView, render_single_memory_view_line(), SelectedMemoryRange, DragComplete, DragUnderway, ViewState, ViewStateHandle (+2 more)

### Community 486 - "dev_container: MountDefinition"
Cohesion: 0.05
Nodes (34): MountDefinition, append_mount_with_target_override(), mount_definition_from_metadata(), parse_mount_definition_string(), deserialize_compose_top_level_volumes(), deserialize_compose_volumes(), deserialize_environment(), deserialize_labels() (+26 more)

### Community 487 - "diagnostics: init()"
Cohesion: 0.08
Nodes (6): init(), ProjectDiagnosticsEditor, RetainExcerpts, All, Dirty, WeakEntity<ProjectDiagnosticsEditor>

### Community 488 - "settings_content: settings_content/src/terminal."
Cohesion: 0.04
Nodes (55): ActivateScript, Csh, Default, Fish, Nushell, PowerShell, Pyenv, AlternateScroll (+47 more)

### Community 489 - "agent: deserialize_maybe_stringified("
Cohesion: 0.09
Nodes (43): deserialize_maybe_stringified(), append_line_segment(), error_text(), format_with_line_numbers(), init_test(), LINE_TRUNCATION_MARKER, MAX_LINE_LEN, numbered() (+35 more)

### Community 490 - "extension: types/lsp.rs"
Cohesion: 0.03
Nodes (60): Completion, CompletionKind, Class, Color, Constant, Constructor, Enum, EnumMember (+52 more)

### Community 491 - "language: markdown_inline_lang()"
Cohesion: 0.10
Nodes (57): markdown_inline_lang(), assert_capture_ranges(), assert_language_range_count_at_text(), assert_language_range_count_at_text_offset(), assert_layers_for_range(), capture_ref(), check_interpolation(), comment_lang() (+49 more)

### Community 492 - "agent: ResolvedMatch"
Cohesion: 0.09
Nodes (46): ResolvedMatch, assert_location_resolution(), DELETION_COST, find_exact_matches(), finish(), fuzzy_eq(), INSERTION_COST, match_ranges() (+38 more)

### Community 493 - "agent_ui: Diff"
Cohesion: 0.07
Nodes (22): Diff, create_multibuffer_diff_editor(), create_terminal(), diff_editor_text_style_refinement(), AssistantMessage, ToolCall, EntryViewEvent, EntryViewState (+14 more)

### Community 494 - "gpui_macros: feature_flags_macros.rs"
Cohesion: 0.06
Nodes (20): derive_enum_feature_flag(), expand(), has_default_attr(), kebab_case(), space_separated(), to_kebab_case(), to_space_separated(), derive_action() (+12 more)

### Community 495 - "git_ui: stash.rs"
Cohesion: 0.08
Nodes (20): GitStash, parse_stash_index(), parse_stash_line(), parse_stash_message(), StashEntry, test_git_stash_empty_input(), test_git_stash_from_str(), test_parse_stash_index() (+12 more)

### Community 496 - "toolchain_selector: ImageLoadingExample"
Cohesion: 0.09
Nodes (13): ImageLoadingExample, pulsating_between(), AddToolchainState, init(), PathInputState, Resolving, WaitingForPath, SearchState (+5 more)

### Community 497 - "gpui: Context"
Cohesion: 0.07
Nodes (35): Tree, swap_rgba_pa_to_bgra(), BundledFonts, cached_fallback_face(), db_with_bundled_fonts(), EMOJI_FONT_FAMILIES, fallback_faces_are_remembered_until_excluded(), fix_generic_font_families() (+27 more)

### Community 498 - "gpui: AtomicU16"
Cohesion: 0.06
Nodes (13): PlatformDispatcher, dedicated_executor_tasks_share_one_thread(), PlatformClock, PlatformScheduler, SmokeDispatcher, spawn_dedicated_detached_child_outlives_root(), spawn_dedicated_dropping_task_cancels_future(), spawn_dedicated_returns_not_send_future_output() (+5 more)

### Community 499 - "fuzzy: char_bag.rs"
Cohesion: 0.06
Nodes (21): CharBag, simple_lowercase(), ADDITIONAL_DISTANCE_PENALTY, BASE_DISTANCE_PENALTY, match_single_path_query(), match_unicode_path_entries(), Matcher<'a>, MIN_DISTANCE_PENALTY (+13 more)

### Community 500 - "workspace: StoreData"
Cohesion: 0.07
Nodes (17): StoreData, docker_identity_ignores_display_name_and_runtime_fields(), RemoteConnectionIdentity, Docker, Mock, Ssh, Wsl, ssh_identity_distinguishes_persistence_key_fields() (+9 more)

### Community 501 - "sandbox: FsSetup"
Cohesion: 0.06
Nodes (26): FsSetup, CanonicalPathBuf, constructors_enforce_windows_grant_shape(), constructors_reject_relative_paths(), linux_fd_identity(), require_absolute(), require_windows_grant_shape(), canonicalize_allowing_missing_leaf() (+18 more)

### Community 502 - "vim: normal/search.rs"
Cohesion: 0.07
Nodes (37): MoveToNext, MoveToPrevious, ReplaceCommand, Replacement, Search, SearchUnderCursor, SearchUnderCursorPrevious, test_backwards_n() (+29 more)

### Community 503 - "deployment: broadcast"
Cohesion: 0.07
Nodes (32): Cancel, cancel_running_and_queued(), coalesces_queued_uploads_and_limits_concurrency(), Job, MAX_HISTORY, ProgressSink, Queued, QueueEvent (+24 more)

### Community 504 - "repl: ClientControlConnection"
Cohesion: 0.06
Nodes (10): build_python_exec_shell_script(), RunningKernel, start_kernel_tasks(), WslKernelSpecification, LocalKernelSpec, LocalKernelSpecContent, LocalKernelSpecsResponse, peek_ports() (+2 more)

### Community 506 - "collab: ChannelRole"
Cohesion: 0.07
Nodes (16): ChannelRole, Admin, Banned, Guest, Member, Talker, proto::ChannelRole, Model (+8 more)

### Community 507 - "copilot: Copilot"
Cohesion: 0.08
Nodes (25): Copilot, CopilotServer, Disabled, Error, Running, Starting, Event, CopilotAuthSignedIn (+17 more)

### Community 509 - "edit_prediction_metrics: reversal.rs"
Cohesion: 0.08
Nodes (53): apply_diff_to_string_lenient(), char_diff(), compute_excerpt_aware_reversal_overlap(), compute_granular_edits(), compute_history_addition_ranges(), compute_history_deletion_ranges(), compute_lcs_length(), compute_prediction_reversal_ratio() (+45 more)

### Community 510 - "extension_api: Component"
Cohesion: 0.08
Nodes (4): Component, Extension, extension(), LanguageServerId

### Community 511 - "git_ui: copy_branch_name()"
Cohesion: 0.09
Nodes (20): copy_branch_name(), create_tag_at_commit(), create_tag_at_head(), create_test_workspace(), CreateTagModal, file_diff_entry(), GitStatusIcon, init() (+12 more)

### Community 512 - "language_models: BoxStream"
Cohesion: 0.08
Nodes (12): stream_in_background(), default_thinking_reasoning_effort(), language_model(), models_supporting_only_none_have_no_selectable_thinking_effort(), normalize_open_ai_response_thinking_effort(), open_ai_reasoning_effort_is_supported(), OpenAiLanguageModelProvider, State (+4 more)

### Community 513 - "vim: visual.rs"
Cohesion: 0.07
Nodes (35): register(), test_cgn_nomatch(), test_cgn_repeat(), test_dgn_repeat(), test_enter_visual_line_mode(), test_enter_visual_mode(), test_gl(), test_gn() (+27 more)

### Community 514 - "workspace: AutoHideFade"
Cohesion: 0.07
Nodes (6): AutoHideFade, FadingIn, FadingOut, AutoHideState, duration_progress(), MessageNotification

### Community 515 - "github-assign-contributor-issue.py: date"
Cohesion: 0.05
Nodes (25): clean_line(), convert_body(), get_release_info(), add_issue_to_project(), assign_issue(), count_open_assignments(), eligible_areas(), execute_graphql() (+17 more)

### Community 516 - "settings_ui: aisettingitem"
Cohesion: 0.10
Nodes (41): add_custom_agent_settings_entry(), AgentRow, build_settings_from_form(), build_settings_from_values(), builds_custom_agent(), collect_agents(), collect_kv(), custom_agent_settings() (+33 more)

### Community 517 - "fuzzy_nucleo: batchsize"
Cohesion: 0.06
Nodes (33): identical_new_content_inputs(), kept_rate_benchmark(), localized_rename_inputs(), repeated_function_lines(), repetitive_token_inputs(), bench_path_matching(), bench_string_matching(), capitalize_each_word() (+25 more)

### Community 518 - "json_schema_store: borrowappcontext_as"
Cohesion: 0.06
Nodes (38): ACTION_SCHEMA_CACHE, all_lsp_adapter_names(), all_schema_file_associations(), ChangedSchemas, DebugTasks, Settings, denormalize_action_name(), DYNAMIC_SCHEMA_CACHE (+30 more)

### Community 519 - "gpui_windows: cf_dib"
Cohesion: 0.06
Nodes (26): CLIPBOARD_GIF_FORMAT, CLIPBOARD_HASH_FORMAT, CLIPBOARD_JPG_FORMAT, CLIPBOARD_METADATA_FORMAT, CLIPBOARD_PNG_FORMAT, CLIPBOARD_SVG_FORMAT, ClipboardGuard, convert_dib_to_bmp() (+18 more)

### Community 520 - "debugger_ui: .stack_frame_list()"
Cohesion: 0.08
Nodes (11): StackFrameEntry, Collapsed, Label, Normal, StackFrameFilter, All, OnlyUserFrames, StackFrameList (+3 more)

### Community 521 - "extension_host: dap::TcpArguments"
Cohesion: 0.05
Nodes (13): dap::TcpArguments, dap::TcpArgumentsTemplate, ExtensionKeyValueStore, github::GithubRelease, latest::dap::TcpArguments, latest::dap::TcpArgumentsTemplate, latest::DownloadedFileType, latest::github::GithubReleaseOptions (+5 more)

### Community 522 - "gpui: Counter"
Cohesion: 0.08
Nodes (7): Counter, test_basic_usage(), test_entity_creation(), test_globals(), test_simulate_scale_factor_change(), TestApp, TestAppWindow

### Community 523 - "migrator: m_2025_01_02/settings.rs"
Cohesion: 0.06
Nodes (29): replace_deprecated_settings_values(), SETTINGS_PATTERNS, UPDATED_SETTINGS, replace_bash_with_terminal_in_profiles(), SETTINGS_PATTERNS, rename_tools(), SETTINGS_PATTERNS, rename_path_search_tool() (+21 more)

### Community 524 - "vim: end_of_document()"
Cohesion: 0.08
Nodes (3): end_of_document(), start_of_document(), Vim

### Community 525 - "git_ui: date_style"
Cohesion: 0.08
Nodes (15): date_style, show_author_name, branch_picker, CommitModal, ForceMode, Amend, Commit, ModalContainerProperties (+7 more)

### Community 526 - "zed: clone_and_open"
Cohesion: 0.06
Nodes (31): GLOBAL, LOGGER, MARGIN, MAX_STATUS_LINES, ProgressLogger, STATUS_TICK_INTERVAL, authenticate(), build_application() (+23 more)

### Community 527 - "editor: DiagnosticRenderer"
Cohesion: 0.09
Nodes (10): DiagnosticRenderer, Editor, GlobalDiagnosticRenderer, inline_diagnostic_rows(), set_diagnostic_renderer(), setup_inline_diagnostics(), test_disabled_diagnostics_ignore_severity_changes(), test_toggle_diagnostics_refreshes_inline_diagnostics() (+2 more)

### Community 528 - "json_schema_store: $ref"
Cohesion: 0.04
Nodes (57): $ref, additionalProperties, description, type, $ref, description, type, $ref (+49 more)

### Community 529 - "language_tools: DisplayItem"
Cohesion: 0.09
Nodes (7): DisplayItem, Entry, ExcerptSeparator, EditorState, HighlightsTreeToolbarItemView, HighlightsTreeView, init()

### Community 530 - "remote: MessageId"
Cohesion: 0.08
Nodes (14): MessageId, ChannelClient, ResponseChannels, Signal, Signal<T>, StreamResponseChannels, test_channel_client_dropping_stream_request_before_completion(), test_channel_client_dropping_stream_request_before_response_cleans_up_channel() (+6 more)

### Community 531 - "onboarding: indent_guides"
Cohesion: 0.06
Nodes (21): indent_guides, active_line_width, background_coloring, coloring, enabled, line_width, CHILD_RADIUS, ThemePreviewStyle (+13 more)

### Community 532 - "extension_cli: build_debug_adapter_schema_pat"
Cohesion: 0.07
Nodes (45): Args, ExtensionFeatureError, IconThemesMixedWithOtherFeatures, NoFeatures, SlashCommandsDeprecated, ThemesMixedWithOtherFeatures, ExtensionManifestValidationError, DescriptionNotLongerThanName (+37 more)

### Community 533 - "deployment: CertificateDer"
Cohesion: 0.06
Nodes (25): VERSION, ErrorCode, Auth, Cancelled, Certificate, Config, Connection, Exists (+17 more)

### Community 534 - "tabular_data_preview: ClearSelection"
Cohesion: 0.08
Nodes (21): format_performance_metrics(), TableView, dispatch(), PerformanceMetrics, selection_bounds(), selection_cells(), setup_test_view(), TableView (+13 more)

### Community 535 - "agent_ui: .emit_load_error()"
Cohesion: 0.06
Nodes (22): LoadError, Exited, FailedToInstall, Unsupported, MaxOutputTokensError, ActiveAcpConnection, AgentConnectedState, AgentConnectionEntry (+14 more)

### Community 536 - "agent: AssertionFn"
Cohesion: 0.06
Nodes (26): AssertionFn, CommandAssertion, eval_git_log_uses_no_pager(), eval_git_rebase_implied_sets_git_editor(), eval_git_rebase_sets_git_editor(), EvalAssertionOutcome, EvalInput, EvalOutput (+18 more)

### Community 537 - "deployment: .connect()"
Cohesion: 0.05
Nodes (39): DataProtection, Clear, Private, ServerType, .ALL, Ftp, Ftps, FtpsImplicit (+31 more)

### Community 538 - "hang_telemetry: .queued_events()"
Cohesion: 0.08
Nodes (39): HangMonitorConfig, HangMonitorError, AlreadyStarted, Spawn, HangMonitorPoll, HangMonitorPollReason, Flush, Interval (+31 more)

### Community 539 - "extensions_ui: ExtensionStatus"
Cohesion: 0.06
Nodes (20): ExtensionStatus, Installed, Installing, NotInstalled, Removing, Upgrading, ContextMenuBuilder, ExtensionCard (+12 more)

### Community 540 - "project: resolve_git_worktree_to_main_r"
Cohesion: 0.08
Nodes (45): resolve_git_worktree_to_main_repo(), test_resolve_git_worktree_to_main_repo_ignores_submodule(), assert_repositories(), build_project(), check_git_statuses(), CONFLICT, init_test(), set_file_scan_depth() (+37 more)

### Community 541 - "sandbox_linux: RequestedAccess"
Cohesion: 0.08
Nodes (26): RequestedAccess, Fs, Net, OpenFile, AskOutcome, Decided, Retry, CONTROL_FD (+18 more)

### Community 542 - "theme: init()"
Cohesion: 0.07
Nodes (23): init(), zed_default_dark(), zed_default_themes(), ChevronIcons, default_icon_theme(), DEFAULT_ICON_THEME, DEFAULT_ICON_THEME_NAME, DirectoryIcons (+15 more)

### Community 543 - "eval_cli: base64"
Cohesion: 0.07
Nodes (28): prepare_build_request(), resolve_source(), base_sha_on_main(), build_toolchain_info(), commit_present(), compute_build_id(), current_base_sha(), current_ref_name() (+20 more)

### Community 544 - "agent: decide_permission_for_paths()"
Cohesion: 0.07
Nodes (31): decide_permission_for_paths(), CopyPathTool, .NAME, CopyPathToolInput, init_test(), test_copy_path_global_skill_directory_to_project(), test_copy_path_project_directory_to_global_skill_directory(), test_copy_path_symlink_escape_confirm_requires_single_approval() (+23 more)

### Community 545 - "db: OnboardingUpsell"
Cohesion: 0.06
Nodes (32): OnboardingUpsell, .KEY, TrialEndUpsell, .KEY, blocks_have_user_content(), clean_mention_links(), display_label_for_draft(), draft_has_user_content() (+24 more)

### Community 546 - "collab: collab.template.yml"
Cohesion: 0.04
Nodes (54): service.beta.kubernetes.io/do-loadbalancer-certificate-id, service.beta.kubernetes.io/do-loadbalancer-disable-lets-encrypt-dns-records, service.beta.kubernetes.io/do-loadbalancer-name, service.beta.kubernetes.io/do-loadbalancer-size-unit, service.beta.kubernetes.io/do-loadbalancer-tls-ports, args, env, image (+46 more)

### Community 547 - "collections: vecmap.rs"
Cohesion: 0.07
Nodes (17): Entry, Entry<'a, K, V>, Occupied, Vacant, EntryRef, EntryRef<'key, 'map, K, V>, Occupied, Vacant (+9 more)

### Community 548 - "miniprofiler_ui: ProfilingCollector"
Cohesion: 0.07
Nodes (20): ProfilingCollector, SerializedTaskTiming, SerializedThreadTaskTimings, ThreadTimingsDelta, append_to_thread(), filter_timings(), init(), kway_merge() (+12 more)

### Community 549 - "language_models: AnthropicLanguageModelProvider"
Cohesion: 0.09
Nodes (11): AnthropicLanguageModelProvider, direct_anthropic_explicit_compaction_uses_paused_completion(), direct_anthropic_supports_explicit_compaction_after_minimum_input(), direct_anthropic_test_model(), direct_anthropic_test_provider(), language_model(), pick_preferred_model(), PROVIDER_NAME (+3 more)

### Community 550 - "language_models: BedrockAuthMethodContent"
Cohesion: 0.06
Nodes (20): AmazonBedrockSettings, BedrockAuth, ApiKey, Automatic, IamCredentials, NamedProfile, SingleSignOn, BedrockAuthMethod (+12 more)

### Community 551 - "edit_prediction_cli: PlainLlmClient"
Cohesion: 0.08
Nodes (42): PlainLlmClient, ensure_repo_cloned(), parse_repo_url(), repo_path_for_url(), run_git(), CACHE_DIR, DATA_DIR, FAILED_EXAMPLES_DIR (+34 more)

### Community 552 - "edit_prediction_metrics: src/kept_rate.rs"
Cohesion: 0.07
Nodes (48): analyze_masked_units(), annotate_kept_rate_tokens(), AnnotatedToken, build_comparison_units(), ComparisonUnit, compute_kept_rate(), count_unmasked_unit_chars(), dp_index() (+40 more)

### Community 553 - "extensions_ui: RELOAD_DEBOUNCE_DURATION"
Cohesion: 0.15
Nodes (47): RELOAD_DEBOUNCE_DURATION, disable_extension_suggestions(), dismiss_suggestion(), EMMET_EXTENSION_ID, ExtensionSuggestionNotification, init(), init_test(), install_emmet_on_disk() (+39 more)

### Community 554 - "json_schema_store: definitions"
Cohesion: 0.04
Nodes (54): definitions, fundingUrl, license, packageExportsEntry, packageExportsEntryOrFallback, packageExportsEntryPath, packageImportsEntry, packageImportsEntryOrFallback (+46 more)

### Community 555 - "language_models: provider/x_ai.rs"
Cohesion: 0.08
Nodes (13): API_KEY_ENV_VAR, API_KEY_ENV_VAR_NAME, default_thinking_reasoning_effort(), grok_43_request_uses_none_when_thinking_is_disabled(), grok_43_request_uses_selected_reasoning_effort(), grok_43_supports_selectable_thinking_effort_levels(), language_model(), PROVIDER_ID (+5 more)

### Community 556 - "language_tools: language_tools.rs"
Cohesion: 0.09
Nodes (7): get_or_create_tool(), init(), BufferState, EditorState, init(), SyntaxTreeToolbarItemView, SyntaxTreeView

### Community 557 - "lsp_locations: build_location_matches()"
Cohesion: 0.09
Nodes (21): build_location_matches(), Entry, Header, Match, Separator, handle_definition_locations(), handle_nav_action(), init() (+13 more)

### Community 558 - "gpui_platform: BenchWindowContext"
Cohesion: 0.07
Nodes (22): Churn, Id, Remount, Replace, Text, ChurnPanel, ChurnRoot, layout_churn() (+14 more)

### Community 559 - "explorer_command_injector: class_e_classnotavailable"
Cohesion: 0.06
Nodes (11): DLL_INSTANCE, DllGetClassObject(), DllMain(), ExplorerCommandInjector, ExplorerCommandInjector_Impl, ExplorerCommandInjectorFactory, ExplorerCommandInjectorFactory_Impl, get_zed_exe_path() (+3 more)

### Community 560 - "agent: path_matches_always_confirm()"
Cohesion: 0.09
Nodes (48): path_matches_always_confirm(), authorize_dirty_buffer(), authorize_file_edit(), authorize_symlink_access(), authorize_symlink_escapes(), canonical_global_skills_dir(), canonicalize_with_ancestors(), DirtyBufferDecision (+40 more)

### Community 561 - "agent_ui: all_models()"
Cohesion: 0.09
Nodes (24): all_models(), assert_models_eq(), create_models(), create_models_with_favorites(), FavoritesIndex, GetActiveModel, GroupedModels, LanguageModelPickerDelegate (+16 more)

### Community 562 - "context_server: AnyNotification"
Cohesion: 0.07
Nodes (12): AnyNotification, Client, ContextServerId, Error, NotificationHandler, NotificationSubscription, NotificationSubscriptionSet, RequestCanceled (+4 more)

### Community 563 - "eval_cli: rejudge.py"
Cohesion: 0.07
Nodes (18): detect_part(), JudgeProxy, load_json(), patch_result_rewards(), proxy_environment(), recompute_qna_rewards(), recompute_rf_rewards(), rejudge_job() (+10 more)

### Community 564 - "project: lsp_log_view_tests.rs"
Cohesion: 0.06
Nodes (33): assert_rpc_registration(), init_test(), rpc_registration_fixture(), test_local_views_and_downstream_peers_own_rpc_streams_independently(), test_log_store_does_not_retain_language_servers(), test_log_store_removes_unavailable_copilot_server(), test_lsp_log_view(), test_lsp_log_view_filters_servers_from_other_projects() (+25 more)

### Community 565 - "project_panel: Trash"
Cohesion: 0.14
Nodes (22): Trash, batch_trash_warns_about_unsaved_changes(), cancel_partial_trash_batch(), create_dir_undo(), create_undo_redo(), cut_paste_undo(), drag_multiple_undo_redo(), drag_undo_redo() (+14 more)

### Community 566 - "sidebar: App"
Cohesion: 0.08
Nodes (8): ThreadSwitcher, ThreadSwitcherEntry, Terminal, Thread, ThreadSwitcherEvent, Confirmed, Dismissed, Preview

### Community 567 - "github-track-duplicate-bot-effectiveness.py: matplotlib_pyplot"
Cohesion: 0.06
Nodes (23): add_or_update_project_item(), add_project_item(), bot_version_for_time(), classify_as_assist(), classify_as_missed_opportunity(), classify_as_success(), classify_closed(), classify_non_author_closed() (+15 more)

### Community 568 - "gpui: binaryheap"
Cohesion: 0.08
Nodes (16): cancel_pending_timers_wakes_waiters_without_waiting_for_deadline(), IdleTracker, is_idle_tracks_queued_work_but_ignores_undue_timers(), MIN_THREADS, run_ready_main_tasks_advances_requeuing_work_one_batch_per_call(), run_ready_main_tasks_does_not_wait_for_background_handoffs(), run_until_idle_completes_background_to_main_handoffs(), run_until_returns_at_readiness_despite_requeuing_main_work() (+8 more)

### Community 569 - "editor: debugger_hover.rs"
Cohesion: 0.10
Nodes (18): build_debugger_hover_view(), DEBUGGER_HOVER_MIN_WIDTH, debugger_hover_row_selector(), debugger_hover_toggle_selector(), debugger_hover_type_suffix(), debugger_hover_variable_colors(), DebuggerHoverChildren, Failed (+10 more)

### Community 570 - "git_ui: git_picker.rs"
Cohesion: 0.12
Nodes (9): GitPicker, GitPickerTab, Branches, Stashes, open_branches(), open_stash(), open_with_tab(), popover() (+1 more)

### Community 571 - "search: buffer_search_query()"
Cohesion: 0.09
Nodes (11): buffer_search_query(), QuerySeed, Query, Text, load_last_search(), query_language_name(), remove_project_search_tab(), SearchSeed (+3 more)

### Community 572 - "settings_json: settings_json.rs"
Cohesion: 0.07
Nodes (32): append_top_level_array_value_in_json_text(), construct_json_value(), find_value_range_in_json_text(), handle_possible_array_value(), infer_json_indent_size(), object_remove_and_rename_find_an_escaped_key_by_its_own_range(), object_replace_escapes_new_key(), parse_index_key() (+24 more)

### Community 573 - "tabular_data_preview: tabular_data_preview/src/parse"
Cohesion: 0.08
Nodes (38): compact_json(), create_table_row(), editor_file_extension(), from_buffer_with_delimiter(), from_json_lines(), parse_delimited_text_with_positions(), REPARSE_DEBOUNCE, snapshot() (+30 more)

### Community 574 - "theme_importer: vscode/syntax.rs"
Cohesion: 0.04
Nodes (45): VsCodeTokenColor, VsCodeTokenColorSettings, VsCodeTokenScope, Many, One, ZedSyntaxToken, Attribute, Boolean (+37 more)

### Community 575 - "agent_ui: loading_contents_spinner()"
Cohesion: 0.06
Nodes (10): loading_contents_spinner(), onboarding_event!, set_edit_prediction_provider(), PatternExample, linear_color_stop(), linear_gradient(), LinearColorStop, test_background_linear_gradient() (+2 more)

### Community 576 - "sqlez: AppMigrator"
Cohesion: 0.06
Nodes (22): AppMigrator, backup_works(), bool_round_trips(), Connection, literal_named_memory_paths(), multi_step_statement_works(), named_memory_connections_do_not_create_literal_backing_files(), NamedMemoryPathGuard (+14 more)

### Community 577 - "git_ui: BufferSubscriptions"
Cohesion: 0.11
Nodes (3): BufferSubscriptions, DiffMultibuffer, project_diff_path_key()

### Community 578 - "gpui: gpui/src/queue.rs"
Cohesion: 0.09
Nodes (16): all_tasks_get_yielded(), blocked_receivers_end_when_the_sender_drops(), Iter, Iter<T>, new_high_prio_task_get_scheduled_quickly(), PriorityQueueReceiver, PriorityQueueReceiver<T>, PriorityQueues (+8 more)

### Community 579 - "language_model_core: deserialize()"
Cohesion: 0.05
Nodes (31): deserialize(), LanguageModelCustomToolFormat, Grammar, Text, LanguageModelCustomToolGrammarSyntax, Lark, Regex, LanguageModelImage (+23 more)

### Community 580 - "gpui_windows: closethreadpooltimer"
Cohesion: 0.06
Nodes (7): TimerResolutionGuard, run_timer_callback(), run_work_callback(), WindowsDispatcher, WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD, SafeCursor, SafeHwnd

### Community 581 - "sandbox: commandandargs"
Cohesion: 0.10
Nodes (30): check_env_forwarding(), check_interop_blocked(), check_windows_drive_writable(), Checks, Cleanup, connect_script(), CREATE_NO_WINDOW, discover_peer() (+22 more)

### Community 582 - "project: context_server_store"
Cohesion: 0.14
Nodes (34): create_fake_transport(), assert_server_events(), FakeContextServerDescriptor, initialize_response(), json_response(), notification_accepted_response(), ServerEvents, set_context_server_configuration() (+26 more)

### Community 583 - "deployment: HostKeyPolicy"
Cohesion: 0.07
Nodes (35): HostKeyPolicy, AcceptNew, Ask, No, Yes, check_host_key(), fingerprint_format(), fingerprint_sha256() (+27 more)

### Community 584 - "title_bar: .get_menus()"
Cohesion: 0.08
Nodes (18): OwnedMenu, TitleBarSettings, ActivateDirection, Left, Right, ApplicationMenu, has_item(), init_test() (+10 more)

### Community 585 - "livekit_client: livekit_client/src/lib.rs"
Cohesion: 0.05
Nodes (33): convert_sample_data(), default_device(), get_sample_data(), Participant, Local, Remote, RoomEvent, ActiveSpeakersChanged (+25 more)

### Community 586 - "workspace: AnyElement"
Cohesion: 0.07
Nodes (5): StatusToast, DismissTimer, Entity<V>, ToastAction, ToastLayer

### Community 587 - "ui: gpui::AnimationElement<T>"
Cohesion: 0.09
Nodes (10): gpui::AnimationElement<T>, popover_opens_below_its_trigger_or_above_when_it_does_not_fit(), PopoverMenu<M>, PopoverMenuElementState, PopoverMenuElementState<M>, PopoverMenuFrameState, PopoverMenuHandle<M>, PopoverMenuHandleState (+2 more)

### Community 588 - "vim: helix/paste.rs"
Cohesion: 0.07
Nodes (37): HelixPaste, replacement_texts(), test_helix_paste_honors_system_clipboard_by_default(), test_helix_paste_ignores_system_clipboard_when_disabled(), test_line_mode_paste(), test_multi_cursor_paste(), test_paste(), test_paste_in_expanded_deleted_hunk() (+29 more)

### Community 589 - "collab: ActiveModel"
Cohesion: 0.11
Nodes (16): Database, LeftChannelBuffer, Operation, operation_from_storage(), operation_from_wire(), operation_to_storage(), QueryOperationSerializationVersion, OperationSerializationVersion (+8 more)

### Community 590 - "agent: BreakpointEventReason"
Cohesion: 0.05
Nodes (17): ActiveDiagnostic, All, Group, None, ActiveDiagnosticGroup, BreakpointPromptEditAction, Condition, HitCondition (+9 more)

### Community 591 - "debugger_ui: CompletionItemType"
Cohesion: 0.09
Nodes (5): assert_completion_range(), Console, ConsoleQueryBarCompletionProvider, test_determine_completion_replace_range(), OutputToken

### Community 592 - "agent: .handle_focus_out()"
Cohesion: 0.05
Nodes (35): EditorEvent, Blurred, BufferEdited, BufferFoldToggled, Closed, CursorShapeChanged, DiffBaseChanged, DirtyChanged (+27 more)

### Community 593 - "copilot_chat: ChatLocation"
Cohesion: 0.09
Nodes (17): ChatLocation, Agent, EditingSession, Editor, Other, Panel, Terminal, copilot_request_headers() (+9 more)

### Community 594 - "livekit_api: Client"
Cohesion: 0.08
Nodes (20): Client, LiveKitClient, admin_token_not_before_remains_unset(), assert_claims_timestamp(), ClaimGrants, create(), create_with_issued_at(), create_with_timestamp_source() (+12 more)

### Community 595 - "lsp_command_selector: active_buffer()"
Cohesion: 0.09
Nodes (13): active_buffer(), available_commands(), AvailableCommand, Execution, Failed, Idle, Running, init() (+5 more)

### Community 596 - "util: quote_cmd_command_arg_for_oute"
Cohesion: 0.07
Nodes (16): quote_cmd_command_arg_for_outer_shell(), quote_prepared_task_arg_for_activation(), ShellKind, Cmd, Csh, Elvish, Fish, Nushell (+8 more)

### Community 597 - "src: freebsd.md"
Cohesion: 0.04
Nodes (44): Building from source, Building Zed for FreeBSD, Cargo errors claiming that a dependency is using unstable features, Dependencies, Repository, Troubleshooting, WebRTC Notice, Download Zed (+36 more)

### Community 598 - "repl: addcodeblock"
Cohesion: 0.04
Nodes (22): CellEvent, FocusedIn, Run, CODE_BLOCK_INSET, CONTROL_SIZE, GUTTER_WIDTH, LARGE_SPACING_SIZE, MAX_TEXT_BLOCK_WIDTH (+14 more)

### Community 599 - "dap: AsyncSeek"
Cohesion: 0.06
Nodes (21): AdapterVersion, DapDelegate, download_adapter_from_github(), DownloadedFileType, GzipTar, Vsix, Zip, WorktreeDelegateAdapter (+13 more)

### Community 600 - "ztracing: Attributes"
Cohesion: 0.08
Nodes (24): init(), absolute_now(), CompletedMeasure, Event, EVENT_BUFFER_CAPACITY, Close, Enter, Exit (+16 more)

### Community 601 - "action_log: ActionLog"
Cohesion: 0.12
Nodes (11): ActionLog, apply_non_conflicting_edits(), ChangeAuthor, Agent, User, ChangedBuffer, diff_snapshots(), LastRejectUndo (+3 more)

### Community 602 - "agent: ActionLog"
Cohesion: 0.14
Nodes (18): ActionLog, ChangeAuthor, Agent, User, ChangedBuffer, init_test(), test_creating_files(), test_deleting_files() (+10 more)

### Community 603 - "agent: .new()"
Cohesion: 0.05
Nodes (12): ChangeList, ContextMenuPlacement, Above, Below, Full, SoftWrap, Bounded, Column (+4 more)

### Community 604 - "agent_ui: ChooseProfileMode"
Cohesion: 0.12
Nodes (14): ChooseProfileMode, ManageProfilesModal, Mode, ChooseProfile, ConfigureDefaultModel, ConfigureMcps, ConfigureTools, NewProfile (+6 more)

### Community 605 - "collab: ChannelMember"
Cohesion: 0.08
Nodes (10): ChannelMember, User, proto::User, channel_member_to_proto(), CloudUserService, FakeUserService, FakeUserServiceState, NewUserParams (+2 more)

### Community 606 - "edit_prediction_metrics: edit_prediction_metrics/src/ma"
Cohesion: 0.09
Nodes (46): applies_patch_in_file_mode(), applies_patch_in_json_excerpt_mode(), applies_patch_with_excerpt_relative_line_numbers(), apply_patch_to_excerpt(), CliInput, Files, Json, filter_hunk_to_excerpt() (+38 more)

### Community 607 - "outline: assert_single_caret_at_row()"
Cohesion: 0.10
Nodes (18): assert_single_caret_at_row(), highlighted_display_rows(), init(), init_test(), open_outline_view(), outline_for_editor(), outline_names(), OutlineView (+10 more)

### Community 608 - "settings_ui: page_data.rs"
Cohesion: 0.11
Nodes (37): ai_page(), appearance_page(), collaboration_page(), debugger_page(), DEFAULT_AUDIO_INPUT, DEFAULT_AUDIO_OUTPUT, DEFAULT_EMPTY_AUDIO_INPUT, DEFAULT_EMPTY_AUDIO_OUTPUT (+29 more)

### Community 609 - "vim: repeat.rs"
Cohesion: 0.08
Nodes (35): register(), repeatable_insert(), Replayer, ReplayerState, should_replay(), test_dot_repeat(), test_dot_repeat_after_macro_change_motion(), test_dot_repeat_registers_paste() (+27 more)

### Community 610 - "workflows: extension_auto_bump.yml"
Cohesion: 0.04
Nodes (47): if, needs, permissions, secrets, strategy, uses, with, concurrency (+39 more)

### Community 611 - "workflows: nix_build.yml"
Cohesion: 0.05
Nodes (47): continue-on-error, env, if, runs-on, steps, timeout-minutes, continue-on-error, env (+39 more)

### Community 612 - "notifications: AddNotification"
Cohesion: 0.08
Nodes (13): AddNotificationsOptions, Count, GlobalNotificationStore, init(), NotificationEntry, NotificationEvent, NewNotification, NotificationRead (+5 more)

### Community 613 - "agent: EditFileToolPartialInput"
Cohesion: 0.10
Nodes (37): EditFileToolPartialInput, Edit, PartialEdit, boundary_sensitive_text(), EditEvent, NewTextChunk, OldTextChunk, EditStreamState (+29 more)

### Community 614 - "agent_ui: AgentRegistryCard"
Cohesion: 0.09
Nodes (10): AgentRegistryCard, AgentRegistryPage, RegistryFilter, All, Installed, NotInstalled, RegistryInstallStatus, InstalledCustom (+2 more)

### Community 615 - "bench_metrics: Counter"
Cohesion: 0.07
Nodes (22): Counter, CounterState, CounterTarget, CallingThread, ThreadAndDescendants, generic_event(), HardwareCounter, HardwareEvent (+14 more)

### Community 616 - "collab_ui: ChannelMembership"
Cohesion: 0.09
Nodes (8): ChannelMembership, MembershipSortKey, LegacyUserId, ChannelModal, ChannelModalDelegate, Mode, InviteMembers, ManageMembers

### Community 617 - "eval_cli: benchmarks.py"
Cohesion: 0.09
Nodes (24): Benchmark, benchmark_metadata(), DatasetRef, get_benchmark(), is_benchmark_selector(), resolve_benchmark_selector(), resolve_benchmarks(), get_judge() (+16 more)

### Community 618 - "extension_host: AsyncFn"
Cohesion: 0.06
Nodes (9): convert_request(), convert_response(), ExtensionHttpResponseStream, ExtensionKeyValueStore, ::http_client::Method, latest::DownloadedFileType, latest::LanguageServerInstallationStatus, latest::SettingsLocation (+1 more)

### Community 619 - "gpui_windows: gpui_windows/src/keyboard.rs"
Cohesion: 0.06
Nodes (9): CANDIDATE_VKEYS, generate_key_char(), get_key_from_vkey(), get_keystroke_key(), get_shifted_key(), get_vkey_from_key_with_us_layout(), need_to_convert_to_shifted_key(), test_keyboard_mapper() (+1 more)

### Community 620 - "json_schema_store: schemas/tsconfig.json"
Cohesion: 0.04
Nodes (46): allOf, allowTrailingCommas, $comment, description, type, properties, description, type (+38 more)

### Community 621 - "language_models: available_model_to_google_mode"
Cohesion: 0.09
Nodes (7): available_model_to_google_model(), catalog_models_drop_xhigh_effort_unless_supported(), GoogleLanguageModelProvider, language_model(), PROVIDER_ID, PROVIDER_NAME, State

### Community 622 - "markdown: html_minifier.rs"
Cohesion: 0.09
Nodes (16): contains_code(), Context<'a>, EntityFn, is_block_element(), is_block_element_name(), is_self_closing(), is_whitespace(), Minifier (+8 more)

### Community 623 - "settings_ui: tool_permissions_setup.rs"
Cohesion: 0.13
Nodes (39): const_str_eq(), decision_to_mode(), delete_pattern(), evaluate_test_input(), find_matched_patterns(), get_tool_render_fn(), get_tool_rules(), HARDCODED_RULES_DESCRIPTION (+31 more)

### Community 624 - "src: Action Reference"
Cohesion: 0.04
Nodes (47): Action Reference, AI Support in Git, Branch Management, Changing the diff view, Commit Message Line Length, Committing, Configuration, Configuring Commit Line Length (+39 more)

### Community 625 - "workflows: if"
Cohesion: 0.06
Nodes (47): if, runs-on, steps, SENTRY_AUTH_TOKEN, SENTRY_ORG, SENTRY_PROJECT, SLACK_MESSAGE, SLACK_WEBHOOK (+39 more)

### Community 626 - "proto: architecture"
Cohesion: 0.06
Nodes (11): BINARY_NAME, PACKAGE_NAME, SERVER_PATH, BufLsp, .SERVER_NAME, ProtobufLanguageServer, .SERVER_NAME, ProtoLs (+3 more)

### Community 627 - "github_releases: asyncseekext"
Cohesion: 0.08
Nodes (23): archive_digest_mismatch_prevents_extraction_and_cleans_up_staging(), download_and_extract(), download_server_asset(), download_server_raw_binary(), download_to_writer(), DownloadedAsset, Archive, RawBinary (+15 more)

### Community 628 - "task: cli/build.rs"
Cohesion: 0.07
Nodes (30): compile_shader_for_module(), compile_shader_impl(), compile_shaders(), find_fxc_compiler(), find_latest_windows_sdk_binary(), generate_rust_binding(), main(), ZED_MANIFEST (+22 more)

### Community 629 - "deployment_ui: deployment/src/project.rs"
Cohesion: 0.07
Nodes (13): CONFIG_PATH, ConfigLocation, existing_config(), import_iremote_config(), imports_iremote_config_once(), IREMOTE_CONFIG_PATH, locate_config(), missing_config() (+5 more)

### Community 630 - "editor: Crease"
Cohesion: 0.07
Nodes (15): Crease, Block, Inline, Crease<T>, CreaseItem, CreaseMap, CreaseMetadata, CreaseSnapshot (+7 more)

### Community 631 - "language_models: language_model()"
Cohesion: 0.09
Nodes (6): language_model(), PROVIDER_ID, PROVIDER_NAME, restart_fetch_models_task_records_error_on_failure(), State, VercelAiGatewayLanguageModelProvider

### Community 632 - "languages: ClientCommand"
Cohesion: 0.06
Nodes (15): RustLspAdapter, .ARCH_SERVER_NAME, .GITHUB_ASSET_KIND, SERVER_NAME, test_convert_rust_analyzer_schema(), test_process_rust_diagnostics(), test_rust_label_for_completion(), detect_host_libc_type() (+7 more)

### Community 633 - "mermaid_render: css_color()"
Cohesion: 0.10
Nodes (29): css_color(), postprocess(), attribute(), class_diagram_labels_keep_sixteen_pixel_font_size(), clipped_group_count(), diagram_directives_cannot_restyle_diagrams(), diagrams_with_too_much_text_to_measure_fail_to_render(), edge_label_backgrounds_use_theme_color() (+21 more)

### Community 634 - "path: abs_path()"
Cohesion: 0.07
Nodes (7): abs_path(), AbsPath, AbsPathBuf, Arc<AbsPath>, PathBuf, Rc<AbsPath>, test_canonicalize_restores_on_disk_spelling()

### Community 635 - "project: compiled_selector()"
Cohesion: 0.09
Nodes (33): compiled_selector(), CompiledDocumentFilter, CompiledDocumentSelector, document_selector_absolute_pattern_normalizes_windows_drive_letter(), document_selector_matches(), document_selector_pattern_matches_document_path(), document_selector_pattern_matches_paths_in_the_host_path_style(), document_selector_pattern_matches_windows_remote_worktree() (+25 more)

### Community 636 - "remote_server: check_pid_file()"
Cohesion: 0.06
Nodes (37): check_pid_file(), CheckPidError, execute_proxy(), ExecuteProxyError, CheckPidFile, KillRunningServer, ServerNotRunning, ServerPath (+29 more)

### Community 637 - "tabular_data_preview: Context"
Cohesion: 0.06
Nodes (23): TableView, append_dev_only_entries(), settings_popover_menu(), toggle_entry(), create_table_cell(), TableView, FilterSortOrder, AlphaThenCount (+15 more)

### Community 638 - "util: shell_from_proto()"
Cohesion: 0.08
Nodes (22): shell_from_proto(), shell_to_proto(), does_not_quote_sole_command_only(), non_interactive_omits_interactive_flag(), redirect_stdin_to_dev_null_fish(), redirect_stdin_to_dev_null_precedence(), redirect_stdin_to_dev_null_preserves_heredoc(), ShellBuilder (+14 more)

### Community 639 - "terminal: InternalEvent"
Cohesion: 0.05
Nodes (34): InternalEvent, Clear, Copy, MoveViCursorToPoint, Resize, Scroll, ScrollToPoint, SetSelection (+26 more)

### Community 640 - "compliance: checks.rs"
Cohesion: 0.16
Nodes (35): approval_comment_by_org_member_succeeds(), approved_review_by_org_member_succeeds(), co_author_org_member_succeeds(), comment(), comment_takes_precedence_over_co_author(), comment_without_approval_pattern_is_not_accepted(), commit_without_pr_number_is_no_pr_found(), group_approval_comment_by_org_member_succeeds() (+27 more)

### Community 641 - "language_model: image_format_from_external_con"
Cohesion: 0.07
Nodes (29): image_format_from_external_content(), ImageFormat, Bmp, Gif, Ico, Jpeg, Png, Pnm (+21 more)

### Community 642 - "eval_cli: report.py"
Cohesion: 0.10
Nodes (23): build_report(), command_report(), extract_trial(), _fill_run_location_from_index(), find_trial_dirs(), _fmt(), _fmt_rate(), format_report() (+15 more)

### Community 643 - "snippet_provider: test_snippets()"
Cohesion: 0.10
Nodes (22): test_snippets(), SnippetRegistryProxy, file_stem_to_key(), file_to_snippets(), GlobalSnippetWatcher, initial_scan(), process_updates(), Snippet (+14 more)

### Community 644 - "gpui: .read()"
Cohesion: 0.13
Nodes (3): BenchWindowContext, BenchWindowContext<'a, 'measurement>, Reservation

### Community 645 - "language_core: highlight_cache.rs"
Cohesion: 0.10
Nodes (20): APPROXIMATE_LRU_NODE_BYTES, ChunkHighlightCache, chunks_match_text(), CostBudgetedLru, CostBudgetedLru<K, V>, .ENTRY_OVERHEAD_BYTES, MAX_CHUNK_HIGHLIGHT_CACHE_BYTES, MAX_TEXT_HIGHLIGHT_CACHE_BYTES (+12 more)

### Community 646 - "outline_panel: CachedEntry"
Cohesion: 0.07
Nodes (22): CachedEntry, CollapsedEntry, Excerpt, ExternalFile, File, Outline, directory_element_id(), file_name() (+14 more)

### Community 647 - "recent_projects: DefaultState"
Cohesion: 0.12
Nodes (35): DefaultState, build_entries(), build_filter_results(), build_server_result(), CandidateMeta, FilterData, FilteredProject, FilteredServer (+27 more)

### Community 648 - "remote: App"
Cohesion: 0.09
Nodes (8): run_wsl_command_impl(), run_wsl_command_with_output_impl(), sanitize_path(), windows_path_to_wsl_path_impl(), wsl_command_impl(), wsl_path_to_windows_path(), WslConnectionOptions, WslRemoteConnection

### Community 649 - "workflows: bump_version.yml"
Cohesion: 0.04
Nodes (44): if, needs, permissions, secrets, uses, with, concurrency, cancel-in-progress (+36 more)

### Community 650 - "gpui_macos: autoreleasepool"
Cohesion: 0.09
Nodes (9): Pasteboard, simulate_external_file_copy(), test_custom_types_survive_creation_autorelease_pool(), test_read_external_path(), test_read_external_paths_with_spaces(), test_read_image(), test_read_multiple_external_paths(), test_string() (+1 more)

### Community 651 - "gpui: bitflags"
Cohesion: 0.05
Nodes (40): KeyboardInteractivity, Exclusive, None, OnDemand, Layer, Background, Bottom, Overlay (+32 more)

### Community 652 - "gpui_apple: CFData"
Cohesion: 0.07
Nodes (21): add_credentials(), check_status(), create_reports_existing_items_without_error(), create_status(), delete_credentials(), delete_preserves_macos_missing_item_error(), KeychainError, InvalidResponse (+13 more)

### Community 653 - "settings: Colors"
Cohesion: 0.08
Nodes (14): IndexMap, FontFeaturesContent, FontSize, FontStyleContent, FontWeightContent, IntoGpui, ModifiersContent, PixelSetting (+6 more)

### Community 654 - "collab: connectionid"
Cohesion: 0.08
Nodes (21): Database, DatabaseTestOptions, assert_channel_tree_matches(), test_channel_buffers(), test_channel_buffers_last_operations(), update_buffer(), assert_channel_tree(), assert_channel_tree_order() (+13 more)

### Community 655 - "context_server: OAuthTokenProvider"
Cohesion: 0.10
Nodes (15): OAuthTokenProvider, FakeTokenProvider, HttpTransport, json_response(), make_fake_http_client(), test_401_after_successful_refresh_still_returns_auth_required(), test_401_returns_auth_required_when_refresh_fails(), test_401_returns_auth_required_without_provider() (+7 more)

### Community 656 - "go_to_line: UserCaretPosition"
Cohesion: 0.10
Nodes (21): UserCaretPosition, assert_single_caret_at_buffer_row(), assert_single_caret_at_row(), current_position(), go_to_point(), GoToLine, highlighted_display_rows(), init() (+13 more)

### Community 657 - "gpui: underline_width()"
Cohesion: 0.11
Nodes (20): underline_width(), TabIndex, TabStopMap, TabStopMapTest, TabStopNode, TabStopOperation, Group, GroupEnd (+12 more)

### Community 658 - "outline_panel: active_outline_panel()"
Cohesion: 0.18
Nodes (38): active_outline_panel(), add_diff_editor(), add_multi_buffer_editor(), assert_directory_paths(), assert_tree(), click_outline_row(), click_test_selection(), ClickTestRow (+30 more)

### Community 659 - "search: SearchResults"
Cohesion: 0.07
Nodes (23): SearchResults, apply_reused_chunk(), consume_search_stream(), ProjectSearch, ReusedResults, SearchActivity, Searching, WaitingForScan (+15 more)

### Community 660 - "settings_ui: App"
Cohesion: 0.11
Nodes (25): SettingsSectionHeader, add_network_host(), add_write_path(), canonicalize_host(), DOMAINS_DESCRIPTION, insert_write_path_subtree(), raw_sandbox_lists(), remove_network_host() (+17 more)

### Community 661 - "sqlez: binding_multiple_statements_wi"
Cohesion: 0.08
Nodes (13): binding_multiple_statements_with_parameter_gaps(), blob_round_trips(), maybe_returns_options(), SqlType, Blob, Float, Integer, Null (+5 more)

### Community 662 - "workspace: SaveStrategy"
Cohesion: 0.11
Nodes (23): SaveStrategy, All, Current, None, add_test_item(), create_fixture(), Fixture, ScheduledTaskResult (+15 more)

### Community 663 - "workflows: comment_on_potential_duplicate"
Cohesion: 0.05
Nodes (43): concurrency, cancel-in-progress, group, ANTHROPIC_API_KEY, GITHUB_TOKEN, ISSUE_NUMBER, if, permissions (+35 more)

### Community 664 - "workflows: community_pr_board.yml"
Cohesion: 0.05
Nodes (43): concurrency, cancel-in-progress, group, GITHUB_TOKEN, MANUAL_PR_NUMBER, PROJECT_NUMBER, pr_number, types (+35 more)

### Community 665 - "workflows: community_pr_board_refresh.yml"
Cohesion: 0.05
Nodes (43): concurrency, cancel-in-progress, group, GITHUB_TOKEN, PROJECT_NUMBER, PUBLISH_HIGH_MR, REFRESH_ALL, SNOWFLAKE_ACCOUNT (+35 more)

### Community 666 - "keymap_editor: ClearKeystrokes"
Cohesion: 0.11
Nodes (6): CloseKeystrokeResult, Close, None, Partial, KeystrokeInput, .KEYSTROKE_COUNT_MAX

### Community 667 - "settings_ui: codestral_api_key_state"
Cohesion: 0.09
Nodes (25): ApiKeyDocs, Custom, Link, codestral_settings(), copilot_settings(), mercury_settings(), OLLAMA_API_URL_PLACEHOLDER, OLLAMA_MODEL_PLACEHOLDER (+17 more)

### Community 668 - "editor: DiffHunkSecondaryStatus"
Cohesion: 0.08
Nodes (10): DiffHunkSecondaryStatus, HasSecondaryHunk, NoSecondaryHunk, OverlapsWithSecondaryHunk, SecondaryHunkAdditionPending, SecondaryHunkRemovalPending, DiffHunkStatus, InlineBlamePopover (+2 more)

### Community 669 - "client: calculate_json_checksum()"
Cohesion: 0.08
Nodes (16): calculate_json_checksum(), HistoricalEvents, init_test(), is_empty_state(), os_name(), os_version(), Telemetry, TelemetrySubscription (+8 more)

### Community 670 - "dev_container: devcontainer_api.rs"
Cohesion: 0.10
Nodes (29): apply_devcontainer_template(), check_for_docker(), DevContainerApply, DevContainerConfig, DevContainerUpFailed, expand_template_options(), find_configs_in_snapshot(), find_devcontainer_configs() (+21 more)

### Community 671 - "edit_prediction_context: bm25_context.rs"
Cohesion: 0.09
Nodes (36): add_query_terms(), add_term_frequencies(), BM25_B, BM25_CONTEXT_CHUNK_COUNT, BM25_CONTEXT_CHUNK_LINE_COUNT, BM25_CONTEXT_CHUNK_OVERLAP_LINE_COUNT, BM25_CONTEXT_EDIT_HISTORY_QUERY_ENTRY_COUNT, BM25_CONTEXT_MAX_CHUNKS_PER_FILE (+28 more)

### Community 672 - "inspector_ui: assert_inspector_watchers()"
Cohesion: 0.11
Nodes (16): assert_inspector_watchers(), DivInspector, flush_teardown(), guess_rust_code_from_style(), InspectorTarget, render_layout_state(), RustStyleCompletionProvider, State (+8 more)

### Community 673 - "json_schema_store: description"
Cohesion: 0.05
Nodes (43): description, items, markdownDescription, type, uniqueItems, x-intellij-html-description, description, items (+35 more)

### Community 674 - "project: pending_op.rs"
Cohesion: 0.06
Nodes (15): GitStatus, Reverted, RevertedUnstaged, Staged, Unchanged, Unstaged, JobStatus, Error (+7 more)

### Community 675 - "sandbox: macos_seatbelt.rs"
Cohesion: 0.10
Nodes (39): escape_sandbox_path(), escape_sandbox_string(), generate_seatbelt_config(), NetworkAccess, All, LocalhostPort, None, protected_file_name_regex() (+31 more)

### Community 676 - "theme: scale.rs"
Cohesion: 0.08
Nodes (17): ColorScale, ColorScales, ColorScaleSet, ColorScaleStep, .ALL, .EIGHT, .ELEVEN, .FIVE (+9 more)

### Community 677 - "skills: humanizer/SKILL.md"
Cohesion: 0.05
Nodes (42): 10. Rule of Three Overuse, 11. Synonym Cycling, 12. False Ranges, 13. Em Dash Overuse, 14. Boldface Overuse, 15. Inline-Header Lists, 16. Title Case Headings, 17. Emojis in Professional Writing (+34 more)

### Community 678 - "workflows: welcome_first_contribution.yml"
Cohesion: 0.05
Nodes (42): cancel-in-progress, group, default, description, type, GITHUB_TOKEN, PR_NUMBER, dry-run (+34 more)

### Community 679 - "gpui: alloc"
Cohesion: 0.09
Nodes (15): connect_and_write_prompt(), main(), main_from_args(), Arena, ArenaBox, ArenaElement, Chunk, test_arena() (+7 more)

### Community 680 - "settings: agent"
Cohesion: 0.05
Nodes (42): agent, auto_compact, auto_generate_thread_titles, button, cancel_generation_on_terminal_stop, commit_message_include_project_rules, default_height, default_model (+34 more)

### Community 681 - "project_bookmarks: bookmarkstore"
Cohesion: 0.11
Nodes (14): add_bookmark(), assert_active_pane_items_len(), build_bookmarks_picker(), init(), init_test(), Match, open_bookmarks_picker(), ProjectBookmarksDelegate (+6 more)

### Community 682 - "acp_thread: .patch_preview()"
Cohesion: 0.14
Nodes (15): build_buffer_diff(), build_finalized(), Diff, Finalized, Pending, FinalizedDiff, FinalizedDiffState, Deferred (+7 more)

### Community 683 - "language_selector: active_picker()"
Cohesion: 0.11
Nodes (15): active_picker(), assert_selected_language_for_editor(), close_selector(), init(), init_test(), LanguageSelector, LanguageSelectorDelegate, open_empty_editor() (+7 more)

### Community 684 - "migrator: m_2025_10_17/settings.rs"
Cohesion: 0.07
Nodes (20): make_file_finder_include_ignored_an_enum(), migrate_one(), migrate_one(), move_edit_prediction_provider_to_edit_predictions(), make_play_sound_when_agent_done_an_enum(), migrate_one(), make_git_gutter_width_an_enum(), migrate_one() (+12 more)

### Community 685 - "recent_projects: remote_connections.rs"
Cohesion: 0.11
Nodes (23): Connection, DevContainer, Ssh, Wsl, determine_paths_with_positions(), init_remote_server(), init_test(), navigate_to_positions() (+15 more)

### Community 686 - "settings_content: .fmt()"
Cohesion: 0.05
Nodes (24): CenteredPaddingSettings, .DEFAULT_PADDING, .MAX_PADDING, .MIN_PADDING, CurrentLanguageServerContent, LanguageServer, Formatter, Auto (+16 more)

### Community 687 - "settings_sync: account_button.rs"
Cohesion: 0.08
Nodes (19): account_avatar(), AccountButton, add_account_entries(), add_sign_out_entry(), observe_account(), status_label(), sync_state(), SyncStatus (+11 more)

### Community 688 - "workspace: workspace_error.rs"
Cohesion: 0.08
Nodes (14): ActionIcon, anyhow::Error, ErrorAction, ErrorActionHandler, Action, Dismiss, ErrorSeverity, Critical (+6 more)

### Community 689 - "workflows: bump_patch_version.yml"
Cohesion: 0.05
Nodes (41): description, required, type, concurrency, cancel-in-progress, group, defaults, run (+33 more)

### Community 690 - "lints: applicability"
Cohesion: 0.08
Nodes (13): AsyncBlockWithoutAwait, AwaitVisitor, call_def_id(), emit_shared_string(), extract_literal_source(), gpui_fixture_rustc_flags(), is_interesting_shared_string_constructor(), is_shared_string() (+5 more)

### Community 691 - "action_log: clock"
Cohesion: 0.21
Nodes (32): init_test(), test_creating_files(), test_deleting_files(), test_deletions(), test_file_read_time_not_forwarded_to_linked_action_log(), test_file_read_time_recorded_on_buffer_created(), test_file_read_time_recorded_on_buffer_edited(), test_file_read_time_recorded_on_buffer_read() (+24 more)

### Community 692 - "tab_switcher: CloseSelectedItem"
Cohesion: 0.13
Nodes (4): init(), TabMatch, TabSwitcher, TabSwitcherDelegate

### Community 693 - "sandbox_linux: controlmessage"
Cohesion: 0.06
Nodes (28): SocketKind, Diagnostics, TcpListener, Udp, body_memfd(), FactoryRequest, CreateSocket, FetchFd (+20 more)

### Community 694 - "gpui_windows: controls"
Cohesion: 0.07
Nodes (20): current_capslock(), DrawCoordinator, DrawWindowGuard, get_frame_thicknessy(), ImeContext, is_virtual_key_pressed(), parse_ime_composition_string(), parse_immutable() (+12 more)

### Community 695 - "benchmarks: edit_range()"
Cohesion: 0.09
Nodes (35): edit_range(), EditPattern, InsertHelperBlocks, LocalizedRewrite, ManySmallChanges, insert_helper_blocks(), large_function_lines(), make_fixture() (+27 more)

### Community 696 - "editor: BookmarksTabEvent"
Cohesion: 0.13
Nodes (6): BookmarksTabEvent, Populated, BookmarksTabState, BookmarkTarget, Editor, set_bookmark_excerpts()

### Community 697 - "zed: WatchSnapshot"
Cohesion: 0.11
Nodes (21): WatchSnapshot, create_editor(), editors_allow_copying_and_preserve_selections_on_updates(), event_rollover_preserves_retained_selection(), Export, format_event(), formats_local_times_event_names_and_plain_paths(), formats_pathless_rescans_and_errors() (+13 more)

### Community 698 - "gpui: examples/testing.rs"
Cohesion: 0.12
Nodes (16): basic_testing(), Counter, CounterEvent, main(), MockNetwork, MockNetworkState, NetworkClient, NetworkedCounter (+8 more)

### Community 699 - "util: path_to_string()"
Cohesion: 0.09
Nodes (11): path_to_string(), insert_subtree(), path_within_subtree(), PathMatcher, test_lenient_path_matcher(), test_lenient_path_matcher_preserves_escaping(), WslPath, file_scan_inclusion_matchers() (+3 more)

### Community 700 - "json_schema_store: description"
Cohesion: 0.06
Nodes (41): description, $ref, description, $ref, packageExportsEntryObject, packageImportsEntryObject, additionalProperties, properties (+33 more)

### Community 701 - "project: Bookmark"
Cohesion: 0.16
Nodes (11): Bookmark, BookmarkEntry, Loaded, Unloaded, BookmarkStore, BookmarkStoreEvent, BookmarksChanged, LabelChanged (+3 more)

### Community 702 - "text: Locator"
Cohesion: 0.07
Nodes (7): Locator, test_locators(), test_sequential_forward_append_stays_at_depth_1(), test_typing_at_cursor_stays_at_depth_2(), DebugRange, GlobalDebugRanges, Key

### Community 703 - "which_key: text_for_keybinding_keystrokes"
Cohesion: 0.13
Nodes (26): text_for_keybinding_keystrokes(), binding(), binding_after_first_keystroke(), binding_row(), group_bindings(), HEADER_LABEL, MAX_KEY_COLUMN_WIDTH, MIN_KEY_COLUMN_WIDTH (+18 more)

### Community 704 - "workspace: AnyView"
Cohesion: 0.13
Nodes (6): AnyView, Entity<T>, render_hideable_item(), SidebarStatus, StatusBar, StatusItemViewHandle

### Community 705 - "workspace: OpenRecentProject"
Cohesion: 0.10
Nodes (5): OpenRecentProject, Section<COLS>, SectionButton, SectionHeader, WelcomePage

### Community 706 - "workflows: track_duplicate_bot_effectiven"
Cohesion: 0.06
Nodes (40): if, runs-on, steps, timeout-minutes, if, runs-on, steps, timeout-minutes (+32 more)

### Community 707 - "xtask: release.rs"
Cohesion: 0.10
Nodes (31): add_compliance_steps(), auto_release_preview(), compliance_check(), COMPLIANCE_REPORT_ARTIFACT_PATH, COMPLIANCE_REPORT_PATH, COMPLIANCE_STEP_ID, create_sentry_release(), CURRENT_ACTION_RUN_URL (+23 more)

### Community 708 - "gpui_linux: action"
Cohesion: 0.06
Nodes (4): current_platform(), linux_platform(), wait_for_clipboard_handovers(), Timeout

### Community 709 - "xtask: annotationkind"
Cohesion: 0.09
Nodes (21): first_line_span(), PermissionsError, ExcessiveDefault, Missing, validate_permissions(), GITHUB_INPUT_PATTERN, RunValidationError, validate_run_command() (+13 more)

### Community 710 - "agent: skill_tool.rs"
Cohesion: 0.13
Nodes (27): create_test_skill(), init_test(), LanguageModelToolResultContent, neutralize_envelope_tags(), render_skill_envelope(), SkillBodyResolver, SkillsResolver, SkillTool (+19 more)

### Community 711 - "edit_prediction_cli: teacher_jumps.md"
Cohesion: 0.05
Nodes (39): 1. User Edit History, 2. Related excerpts, 3. Current File, Current File, Current File, Current File, Current File, Current File (+31 more)

### Community 712 - "editor: CodeActionSource"
Cohesion: 0.10
Nodes (11): CodeActionSource, Indicator, QuickActionBar, RunMenu, ToggleCodeActions, CodeActionProvider, Editor, Entity<Project> (+3 more)

### Community 713 - "project_panel: ui_scrollbar_settings_from_raw"
Cohesion: 0.08
Nodes (16): ui_scrollbar_settings_from_raw(), IndentGuidesSettings, OutlinePanelSettings, OutlinePanelSettingsScrollbarProxy, ScrollbarSettings, AutoOpenSettings, IndentGuidesSettings, ProjectPanelScrollbarProxy (+8 more)

### Community 714 - "project: .initial_graph_data()"
Cohesion: 0.07
Nodes (28): LogOrder, AuthorDateOrder, DateOrder, ReverseChronological, TopoOrder, LogSource, All, Path (+20 more)

### Community 715 - "git_ui_core: build_buffer_diff()"
Cohesion: 0.11
Nodes (3): build_buffer_diff(), FileDiffView, test_diff_view()

### Community 716 - "gpui: gpui/src/subscription.rs"
Cohesion: 0.13
Nodes (20): SingleSubscriberSet, SingleSubscriberSet<Callback>, SingleSubscriberSetState, Subscriber, Subscriber<Callback>, SubscriberSet, SubscriberSet<EmitterKey, Callback>, SubscriberSetState (+12 more)

### Community 717 - "path: Component"
Cohesion: 0.12
Nodes (12): Component, Component<'a>, CurDir, Normal, ParentDir, Prefix, RootDir, Components<'a> (+4 more)

### Community 718 - "project: src/debugger.rs"
Cohesion: 0.11
Nodes (18): MappedPageContents, Memory, MemoryAddress, MemoryCell, MemoryIterator, MemoryPageBuilder, page_contents_into_iter(), PAGE_SIZE (+10 more)

### Community 719 - "remote: handle_io()"
Cohesion: 0.09
Nodes (27): handle_io(), message_len_from_buffer(), MESSAGE_LEN_SIZE, MessageLen, read_message(), read_message_raw(), read_message_with_len(), write_message() (+19 more)

### Community 720 - "search: search/src/search.rs"
Cohesion: 0.06
Nodes (18): EXCLUDE_PLACEHOLDER, INCLUDE_PLACEHOLDER, init(), REPLACE_PLACEHOLDER, SearchOption, Backwards, CaseSensitive, IncludeIgnored (+10 more)

### Community 721 - "theme_selector: init()"
Cohesion: 0.11
Nodes (9): init(), open_theme_selector(), previewed_theme_name(), selected_theme_name(), test_theme_selector_preserves_selection_on_empty_filter(), ThemeSelector, ThemeSelectorDelegate, toggle_icon_theme_selector() (+1 more)

### Community 723 - "workspace: AppNotifications"
Cohesion: 0.13
Nodes (14): AppNotifications, dismiss_app_notification(), show_app_notification(), std::result::Result<T, E>, test_language_server_prompt_deduplication(), test_language_server_prompt_deduplication_preserves_dismissal(), test_notification_auto_dismiss_turned_off(), test_notification_auto_dismiss_with_custom_duration() (+6 more)

### Community 724 - "x_ai_subscribed: do_oauth_flow()"
Cohesion: 0.11
Nodes (21): do_oauth_flow(), fake_token_response(), FakeCredentialsProvider, get_fresh_credentials(), make_expired_credentials(), make_fresh_credentials(), make_state(), make_state_with_credentials_provider() (+13 more)

### Community 725 - "xtask: PublishPlanEntry"
Cohesion: 0.09
Nodes (26): PublishPlanEntry, APACHE_LICENSE, APACHE_LICENSE_FILE, ApacheLicensed, CONFLICTING_LICENSE_FILES, is_symlinked_license(), license_violation(), LicenseViolation (+18 more)

### Community 726 - "util: assignprocesstojobobject"
Cohesion: 0.09
Nodes (9): assert_process_exits(), Child, JobObject, process_is_alive(), spawn_process_tree(), test_drop_terminates_grandchildren(), test_job_tracks_grandchild_after_direct_child_exits(), test_kill_terminates_grandchildren() (+1 more)

### Community 727 - "gpui_macros: braced"
Cohesion: 0.14
Nodes (30): try_expand_macro(), border_prefixes(), border_style_methods(), border_suffixes(), BorderStylePrefix, BorderStyleSuffix, box_prefixes(), box_shadow_style_methods() (+22 more)

### Community 728 - "extension: compilationconcurrency"
Cohesion: 0.08
Nodes (16): extension_benchmarks(), extension_builder(), init(), manifest(), wasm_bytes(), wasm_host(), CompilationConcurrency, Bounded (+8 more)

### Community 729 - "agent: PermissionPattern"
Cohesion: 0.11
Nodes (36): PermissionPattern, CommandPrefix, common_parent_dir(), escape_for_pattern(), extract_all_terminal_patterns(), extract_command_prefix(), extract_copy_move_pattern(), extract_copy_move_pattern_display() (+28 more)

### Community 730 - "channel: src/channel.rs"
Cohesion: 0.08
Nodes (10): ACKNOWLEDGE_DEBOUNCE_INTERVAL, ChannelBuffer, ChannelBufferEvent, BufferEdited, ChannelChanged, CollaboratorsChanged, Connected, Disconnected (+2 more)

### Community 731 - "cloud_api_types: KnownOrUnknown"
Cohesion: 0.07
Nodes (18): KnownOrUnknown, Known, Unknown, Plan, ZedBusiness, ZedFree, ZedPro, ZedProTrial (+10 more)

### Community 732 - "dev_container: command_json.rs"
Cohesion: 0.10
Nodes (17): CommandRunner, DefaultCommandRunner, deserialize_json_output(), deserialize_yaml_output(), evaluate_json_command(), evaluate_yaml_command(), success_output(), test_deserialize_empty_object() (+9 more)

### Community 733 - "terminal: extension_api/src/process.rs"
Cohesion: 0.09
Nodes (10): process_map_stays_bounded(), ProcessIdGetter, ProcessInfo, PtyProcessInfo, PtyResources, Active, Released, TerminalType (+2 more)

### Community 734 - "git_ui: commit_message_editor()"
Cohesion: 0.09
Nodes (18): commit_message_editor(), GitPanelViewMode, Flat, Tree, register(), register_git_commit_language(), SerializedCommitMessage, SerializedGitPanel (+10 more)

### Community 735 - "scheduler: .drop()"
Cohesion: 0.06
Nodes (10): capture_execution_order(), test_nondeterministic_waker_drop_detection(), test_randomize_order(), test_scheduler_drops_with_never_polled_dedicated_task(), test_scheduler_drops_with_stalled_detached_background_task(), test_scheduler_drops_with_stalled_detached_foreground_task(), test_send_local_executor_retains_scheduler(), test_send_local_executor_worker_submission_and_wakeup() (+2 more)

### Community 736 - "text: ContextLessSummary"
Cohesion: 0.07
Nodes (12): ContextLessSummary, NoSummary, MapKey<K>, Operation, OperationItem, OperationItem<T>, OperationKey, OperationQueue (+4 more)

### Community 737 - "zed: EventWrapper"
Cohesion: 0.13
Nodes (9): EventWrapper, collapsed_params_md(), expanded_params_md(), init(), TelemetryLogEntry, TelemetryLogEvent, ShowToast, TelemetryLogToolbarItemView (+1 more)

### Community 738 - "vim: increment.rs"
Cohesion: 0.09
Nodes (34): BOOLEAN_PAIRS, Decrement, find_target(), Increment, increment_binary_string(), increment_decimal_string(), increment_hex_string(), increment_toggle_string() (+26 more)

### Community 739 - ".doc-examples: complex-feature.md"
Cohesion: 0.05
Nodes (38): Action Reference {#action-reference}, AI Support in Git {#ai-support}, Branch Management {#branch-management}, Commit Message Line Length, Committing {#committing}, Configuration {#configuration}, Configuring Commit Line Length, Creating and Switching Branches {#create-switch-branches} (+30 more)

### Community 740 - "workflows: after_release.yml"
Cohesion: 0.05
Nodes (38): default, description, type, defaults, run, if, permissions, secrets (+30 more)

### Community 741 - "workflows: community_close_stale_issues.y"
Cohesion: 0.05
Nodes (38): default, description, type, debug-only, operations-per-run, jobs, stale, name (+30 more)

### Community 742 - "workflows: community_pr_cleanup.yml"
Cohesion: 0.05
Nodes (38): if, runs-on, steps, timeout-minutes, concurrency, cancel-in-progress, group, default (+30 more)

### Community 743 - "workflows: compliance_check.yml"
Cohesion: 0.05
Nodes (38): defaults, run, env, ARTIFACT_URL, CARGO_TERM_COLOR, COMPLIANCE_OUTCOME, COMPLIANCE_TAG, GITHUB_APP_ID (+30 more)

### Community 744 - "workflows: guild_assignment_status.yml"
Cohesion: 0.05
Nodes (38): concurrency, cancel-in-progress, group, GITHUB_TOKEN, GUILD_MODE, PROJECT_NUMBER, SLACK_WEBHOOK_GUILD_INTERNAL, if (+30 more)

### Community 745 - "workflows: wezel.yml"
Cohesion: 0.05
Nodes (38): description, required, type, cancel-in-progress, group, description, required, type (+30 more)

### Community 746 - "vim: ChildStdin"
Cohesion: 0.07
Nodes (11): encode_ranges(), NeovimConnection, NeovimData, Exec, Get, Key, Put, ReadRegister (+3 more)

### Community 747 - "agent: ProjectSnapshot"
Cohesion: 0.10
Nodes (23): ProjectSnapshot, DetailedSummaryState, Generated, Generating, NotGenerated, LegacySerializedMessage, LegacySerializedThread, MessageId (+15 more)

### Community 748 - "benchmarks: benches/edit_file_tool.rs"
Cohesion: 0.09
Nodes (18): BenchmarkHarness, block_on_executor(), chunk_ends(), edit_file_tool_streaming(), EditFixture, EditOp, FILE_ABS_PATH, FILE_PROJECT_PATH (+10 more)

### Community 749 - "context_server: AnyRequest"
Cohesion: 0.08
Nodes (14): AnyRequest, AnyResponse, RequestId, Int, Str, McpServer, McpServerTool, .NAME (+6 more)

### Community 750 - "sqlez: open_fallback_db()"
Cohesion: 0.09
Nodes (18): open_fallback_db(), open_main_db(), open_test_db(), background_thread_queue(), connection_initialize_query_retries_transient_schema_lock(), CONNECTION_INITIALIZE_RETRIES, CONNECTION_INITIALIZE_RETRY_DELAY, is_schema_lock_error() (+10 more)

### Community 751 - "edit_prediction_cli: BatchingOpenAiClient"
Cohesion: 0.09
Nodes (19): BatchingOpenAiClient, content_to_string(), message_content_to_string(), message_role_to_string(), Batch, Plain, Choice, ImageUrl (+11 more)

### Community 752 - "gpui_windows: directx_devices.rs"
Cohesion: 0.08
Nodes (10): check_debug_layer_available(), DirectXDevices, get_adapter(), get_device(), get_dxgi_factory(), try_to_recover_from_device_lost(), report_live_objects(), get_module_handle() (+2 more)

### Community 753 - "project: BranchDiffEvent"
Cohesion: 0.09
Nodes (11): BranchDiffEvent, DiffBaseChanged, FileListChanged, DiffBase, Head, Index, Merge, Staged (+3 more)

### Community 754 - "search: registrar.rs"
Cohesion: 0.12
Nodes (17): ActionExecutor, DidHandleAction, DivRegistrar, DivRegistrar<'a, 'b, T>, DivRegistrar<'_, '_, T>, ForDeployed, ForDeployed<A>, ForDismissed (+9 more)

### Community 755 - "tabular_data_preview: ColumnFilterDelegate"
Cohesion: 0.11
Nodes (8): ColumnFilterDelegate, ColumnFilterListEntry, Header, Row, ColumnFilterRow, natural_content(), NaturallyOrdered, TableView

### Community 756 - ".conventions: CONVENTIONS.md"
Cohesion: 0.05
Nodes (37): Action and Keybinding References, Add to an existing page when:, Anchor IDs, Callouts, Code Formatting, Create a new page when:, Cross-Linking, Document (+29 more)

### Community 757 - "workflows: guild_stale_assignments.yml"
Cohesion: 0.05
Nodes (37): concurrency, cancel-in-progress, group, GITHUB_TOKEN, GUILD_MODE, PROJECT_NUMBER, SLACK_WEBHOOK_GUILD_INTERNAL, jobs (+29 more)

### Community 758 - "workflows: guild_weekly_shipped.yml"
Cohesion: 0.05
Nodes (37): concurrency, cancel-in-progress, group, if, runs-on, steps, timeout-minutes, GITHUB_TOKEN (+29 more)

### Community 759 - "workflows: slack_notify_first_responders."
Cohesion: 0.05
Nodes (37): cancel-in-progress, group, env, GH_REPO, GH_TOKEN, ISSUE_NUMBER, ISSUE_TITLE, ISSUE_URL (+29 more)

### Community 760 - "compliance: ReviewResult"
Cohesion: 0.12
Nodes (23): ReviewResult, IssueKind, Error, NotReviewed, make_commit(), PULL_REQUEST_BASE_URL, Report, report_summary_all_reviewed_is_no_issues() (+15 more)

### Community 761 - "gpui_windows: core"
Cohesion: 0.07
Nodes (16): apply_alpha_correction(), ClusterAnalyzer, color_brightness(), ColorGlyphBitmap, ColorLayer, DEFAULT_LOCALE_NAME, enhance_contrast(), font_weight_from_dwrite() (+8 more)

### Community 762 - "agent_ui: .active_initial_content()"
Cohesion: 0.08
Nodes (17): AgentPanelSiblingHost, SourcePanelInitialization, AgentInitialContent, FromExternalSource, ThreadSummary, NewNativeAgentThreadFromSummary, collapses_newline_padding(), ExternalSourcePrompt (+9 more)

### Community 763 - "ai_onboarding: App"
Cohesion: 0.13
Nodes (7): SignInStatus, SignedIn, SignedOut, SigningIn, ZedAiOnboarding, content_column(), rems_from_px()

### Community 764 - "call: call_impl/diagnostics.rs"
Cohesion: 0.12
Nodes (18): CallDiagnostics, CallDiagnosticsReport, CallDiagnosticsSnapshot, CallStats, compute_network_stats(), compute_remote_audio_stats(), ComputedNetworkStats, ConnectionQualityDTO (+10 more)

### Community 765 - "feature_flags: DebuggerHistoryFeatureFlag"
Cohesion: 0.07
Nodes (25): DebuggerHistoryFeatureFlag, .NAME, EditPredictionJumpsFeatureFlag, .NAME, FeatureFlag, .NAME, AcpBetaFeatureFlag, .NAME (+17 more)

### Community 766 - "edit_prediction: Anchor"
Cohesion: 0.09
Nodes (14): ZedEditPredictionDelegate, EditPredictionRequestTrigger, BufferEdit, DiagnosticNavigation, EditorCreated, Explicit, LSPCompletionAccepted, Other (+6 more)

### Community 767 - "editor: editor/src/clipboard.rs"
Cohesion: 0.11
Nodes (7): ClipboardSelection, edit_for_markdown_paste(), Editor, is_standalone_url(), kill_ring_metadata_for_text(), KillRing, unused_image_path()

### Community 768 - "json_schema_store: default"
Cohesion: 0.05
Nodes (37): default, description, markdownDescription, type, x-intellij-html-description, properties, properties, default (+29 more)

### Community 769 - "language_models: ConfigurationView"
Cohesion: 0.08
Nodes (3): ConfigurationView, test_authenticate_awaits_initial_load(), XAiSubscribedProvider

### Community 770 - "language: bogus_candidates()"
Cohesion: 0.16
Nodes (10): bogus_candidates(), BracketDelimiter, BracketDelimiterPair, BracketMatch, BracketMatchCandidate, BracketPatternKey, BufferSnapshot, ChunkBrackets (+2 more)

### Community 771 - "migrator: migrator/src/migrations.rs"
Cohesion: 0.09
Nodes (16): make_auto_indent_an_enum(), migrate_auto_indent(), flatten_code_actions_formatters(), remove_formatters_on_save(), remove_formatters_on_save_inner(), restore_code_actions_on_format(), restore_code_actions_on_format_inner(), make_relative_line_numbers_an_enum() (+8 more)

### Community 772 - "svg_preview: App"
Cohesion: 0.15
Nodes (4): SvgPreviewMode, Default, Follow, SvgPreviewView

### Community 773 - "terminal: assert_wakeups_and_notifies()"
Cohesion: 0.14
Nodes (12): assert_wakeups_and_notifies(), Expected, HoveredWord, HyperlinkVisualTestContext<'a, 'b>, init_ctrl_hover_hyperlink_test_with_window(), Notifies, test_ctrl_hover_with_changing_bounds(), test_ctrl_hover_with_changing_content() (+4 more)

### Community 774 - "src: Any Bracket Functionality"
Cohesion: 0.05
Nodes (37): Any Bracket Functionality, AnyQuotes and AnyBrackets (Traditional Vim behavior), Changing vim mode settings, Choosing Between Approaches, Command mnemonics, Command palette, Core differences, Customizing key bindings (+29 more)

### Community 775 - "workflows: guild_new_pr_notify.yml"
Cohesion: 0.05
Nodes (36): concurrency, cancel-in-progress, group, GITHUB_TOKEN, GUILD_MODE, PROJECT_NUMBER, SLACK_WEBHOOK_GUILD_INTERNAL, jobs (+28 more)

### Community 777 - "theme_settings: AccentContent"
Cohesion: 0.11
Nodes (25): DARK_DIFF_HUNK_FILLED_OPACITY, DARK_DIFF_HUNK_HOLLOW_BACKGROUND_OPACITY, DARK_DIFF_HUNK_HOLLOW_BORDER_OPACITY, diff_hunk_colors_fallback_to_version_control_colors(), diff_hunk_fallbacks_are_absent_when_status_and_version_control_colors_are_missing(), diff_hunk_opacity_fallbacks_use_correct_values_for_light_and_dark_themes(), ensure_non_opaque(), ensure_opaque() (+17 more)

### Community 778 - "agent_ui: agentthreadentry"
Cohesion: 0.10
Nodes (19): acp_subagent_card(), AcpSubagentCard, an_acp_parents_label_wins_over_the_subagents_title(), card(), fullscreen_opens_a_stopped_card_at_the_start_of_its_part(), has_user_canceled_output(), preview_range_shows_the_last_entries_the_card_covers(), reported_states_map_to_card_statuses() (+11 more)

### Community 779 - "gpui_web: applicationhandle"
Cohesion: 0.08
Nodes (24): ACCENT_BLUE, ACCENT_GREEN, ACCENT_MAUVE, ACCENT_YELLOW, BG_BASE, BG_OVERLAY, BG_SURFACE, ChunkResult (+16 more)

### Community 780 - "acp_thread: .clear_completed_plan_entries("
Cohesion: 0.10
Nodes (19): Plan, plan_entry_from_v1(), PlanContent, Items, Markdown, PlanContentUpdate, Items, Markdown (+11 more)

### Community 781 - "agent: reindent.rs"
Cohesion: 0.13
Nodes (18): compute_indent_delta(), compute_rest_indent_delta(), IndentDelta, Spaces, Tabs, line_indent(), Reindenter, test_clamp_negative_indent() (+10 more)

### Community 782 - "editor: .clear_highlights()"
Cohesion: 0.06
Nodes (28): HighlightKey, BufferSearchHighlights, ColorizeBracket, ConsoleAnsiHighlight, DebugStackFrameLine, DocumentHighlightRead, DocumentHighlightWrite, Editor (+20 more)

### Community 783 - "git_ui: common_prefix()"
Cohesion: 0.11
Nodes (5): common_prefix(), Entry, load_entries(), MultiDiffView, register_entry()

### Community 784 - "git_ui: AnyEntity"
Cohesion: 0.12
Nodes (3): SoloDiffButtonStates, SoloDiffView, test_changes_only_multibuffer_has_one_buffer_and_expand_controls()

### Community 785 - "git_ui: build_clipboard_buffer()"
Cohesion: 0.11
Nodes (4): build_clipboard_buffer(), selection_location_text(), TextDiffView, update_diff_buffer()

### Community 786 - "gpui: ActiveTextContext"
Cohesion: 0.09
Nodes (11): ActiveTextContext, App, CharacterGrid, GlobalTextContext, main(), run_example(), Specimen, SpecimenTheme (+3 more)

### Community 787 - "outline_panel: add_outline_panel()"
Cohesion: 0.24
Nodes (30): add_outline_panel(), display_entries(), init_test(), mark_deleted(), outline_panel(), populate_with_test_ra_project(), snapshot(), test_buffer_search() (+22 more)

### Community 788 - "project: SerializedBookmark"
Cohesion: 0.26
Nodes (33): SerializedBookmark, add_bookmarks(), add_labeled_bookmark(), assert_bookmark_labels(), assert_bookmark_rows(), build_serialized(), clear_bookmarks(), get_all_bookmarks() (+25 more)

### Community 789 - "project: Buffer"
Cohesion: 0.13
Nodes (4): SearchInputs, SearchQuery, Regex, Text

### Community 790 - "repl: plain.rs"
Cohesion: 0.15
Nodes (12): init_test(), max_width_for_columns(), terminal_size(), TerminalOutput, test_append_text_preserves_split_ansi_sequence(), test_full_text_reads_terminal_output(), test_initial_text_uses_repl_terminal_size(), test_max_width_for_columns_matches_cell_width() (+4 more)

### Community 791 - "theme_selector: .focus_handle()"
Cohesion: 0.12
Nodes (5): IconThemeSelectorDelegate, open_icon_theme_selector(), previewed_theme_name(), selected_theme_name(), test_icon_theme_selector_preserves_selection_on_empty_filter()

### Community 792 - "vim: surround.rs"
Cohesion: 0.12
Nodes (25): apply_helix_surround_edits(), helix_multibuffer_context(), preserved_selection_anchors(), selection_cursor(), SurroundEdits, SurroundSelections, Anchors, Surrounded (+17 more)

### Community 794 - "workspace: multi_workspace_tests.rs"
Cohesion: 0.12
Nodes (33): init_test(), setup_multi_workspace(), test_adding_worktree_updates_project_group_key(), test_close_workspace_opens_unloaded_local_neighbor(), test_close_workspace_prefers_already_loaded_neighboring_workspace(), test_close_workspace_prefers_workspace_in_same_project_group(), test_close_workspace_with_remote_neighbor_does_not_create_local_workspace(), test_find_or_create_local_workspace_reuses_active_workspace_after_sidebar_open() (+25 more)

### Community 795 - "workspace: Entity<T>"
Cohesion: 0.12
Nodes (4): Entity<T>, PaneSearchBarCallbacks, Toolbar, ToolbarItemViewHandle

### Community 796 - "settings: profiles"
Cohesion: 0.08
Nodes (35): profiles, name, tools, enable_all_context_servers, name, tools, ask, minimal (+27 more)

### Community 797 - "snippets_ui: ConfigureSnippets"
Cohesion: 0.11
Nodes (8): snippets_dir(), configure_snippets(), init(), open_folder(), register(), ScopeFileName, ScopeName, ScopeSelectorDelegate

### Community 798 - "activity_indicator: ActivityIcon"
Cohesion: 0.11
Nodes (10): ActivityIcon, Icon, LoadingSpinner, ActivityIndicator, Content, DeferredScanMessage, Dismissed, Pending (+2 more)

### Community 799 - "agent_ui: message_queue.rs"
Cohesion: 0.10
Nodes (7): MessageQueue, ProcessingState, AbsorbingCancel, AutoProcess, Paused, QueueEntry, QueueEntryId

### Community 800 - "collab: Database"
Cohesion: 0.13
Nodes (9): Database, Database, model_to_proto(), notification_kind_from_proto(), Notification, ChannelInvitation, ContactRequest, ContactRequestAccepted (+1 more)

### Community 801 - "edit_prediction_cli: teacher.md"
Cohesion: 0.06
Nodes (34): 1. User Edit History, 2. Related excerpts, 3. Current File, Current File, Current File, Current File, Current File, Current File (+26 more)

### Community 802 - "gpui_shared_string: gpui_shared_string.rs"
Cohesion: 0.08
Nodes (3): Arc<str>, SharedString, String

### Community 803 - "gpui_web: SendForegroundExecutor"
Cohesion: 0.07
Nodes (14): SendForegroundExecutor, FetchCredentials, Include, Omit, SameOrigin, FetchHttpClient, ConsoleLogger, init_logging() (+6 more)

### Community 804 - "markdown: markdown/src/selection.rs"
Cohesion: 0.13
Nodes (31): assert_marked_selection(), assert_snapped(), boundary_block_events(), collect_inline_spans(), inline_code_full_range(), inline_spans(), InlineSpan, is_inline_span_tag() (+23 more)

### Community 805 - "proto: proto/src/error.rs"
Cohesion: 0.08
Nodes (7): anyhow::Error, code_for_untyped_error(), ErrorCodeExt, ErrorExt, RpcError, test_other_untyped_errors_stay_internal(), test_rpc_error_code_takes_precedence()

### Community 806 - "repl: local_kernel_specifications()"
Cohesion: 0.09
Nodes (9): local_kernel_specifications(), LocalKernelSpecification, NativeRunningKernel, read_kernels_dir(), read_kernelspec_at(), test_get_kernelspecs(), NotebookMode, Command (+1 more)

### Community 808 - "rpc: rpc/src/auth.rs"
Cohesion: 0.11
Nodes (15): assert_printable(), EncryptionFormat, V0, V1, keypair(), oaep_sha256_padding(), PrivateKey, PublicKey (+7 more)

### Community 809 - "lints: expr_or_init"
Cohesion: 0.16
Nodes (16): clippy_covers_get_then_check(), emit_lint(), find_matching_insert(), has_inherent_entry_method(), is_assoc_of_diag_trait(), is_std_set(), is_string(), keys_match() (+8 more)

### Community 810 - "xtask: vars.rs"
Cohesion: 0.06
Nodes (18): BWRAP_LINUX_AARCH64, BWRAP_LINUX_X86_64, LINUX_AARCH64, LINUX_X86_64, MAC_AARCH64, MAC_X86_64, secret!, var! (+10 more)

### Community 811 - "vim: ChangeCase"
Cohesion: 0.14
Nodes (16): ConvertTarget, LowerCase, OppositeCase, Rot13, Rot47, UpperCase, test_change_case(), test_change_case_helix_mode() (+8 more)

### Community 812 - "agent_skills: agent_skills/README.md"
Cohesion: 0.06
Nodes (33): Activation, Activation requires authorization, agent_skills, Catalog, `disable-model-invocation` filters this list, `disable-model-invocation` (we support), Discovery, Edits to skill files (+25 more)

### Community 813 - "agent: default_context_lines()"
Cohesion: 0.13
Nodes (23): default_context_lines(), default_max_ancestor_lines(), extract_paths_from_results(), GrepTool, .NAME, GrepToolInput, init_test(), run_grep_tool() (+15 more)

### Community 814 - "editor: scroll_amount.rs"
Cohesion: 0.10
Nodes (20): ScrollAmount, Column, Line, Page, PageWidth, ScrollDirection, Downwards, Leftwards (+12 more)

### Community 815 - "git_ui: .editor_position()"
Cohesion: 0.11
Nodes (7): display_name_of(), open(), RepositorySelector, RepositorySelectorDelegate, PickerEditorPosition, End, Start

### Community 816 - "input_latency_ui: histogram_bucket_counts()"
Cohesion: 0.10
Nodes (21): histogram_bucket_counts(), InputLatencySnapshot, format_input_latency_report(), format_report(), InputLatencyReportData, insert_buckets(), MIN_DRAWS_TO_REPORT, MIN_FRAMES_TO_REPORT (+13 more)

### Community 817 - "gpui: AbsoluteLength"
Cohesion: 0.11
Nodes (13): AbsoluteLength, border_widths_to_taffy(), border_widths_to_taffy_use_stroke_snapping(), DefiniteLength, Edges<T>, Length, Point<T2>, Size<T> (+5 more)

### Community 818 - "json_schema_store: description"
Cohesion: 0.06
Nodes (34): description, items, oneOf, type, description, properties, type, description (+26 more)

### Community 819 - "language: .modeline()"
Cohesion: 0.13
Nodes (28): EMACS_MODELINE_RE, ModelineSettings, parse_emacs_key_value(), parse_emacs_local_variables(), parse_emacs_modeline(), parse_modeline(), parse_modelines(), parse_vim_modeline() (+20 more)

### Community 820 - "sandbox_linux: Action"
Cohesion: 0.06
Nodes (34): Action, AbstractUnixServer, Append, AssertNamespaces, AssertNoCapabilities, BindUnix, Connect, ConnectByName (+26 more)

### Community 821 - "settings_content: fallible_options.rs"
Cohesion: 0.09
Nodes (17): deserialize(), FallibleOption, Foo, flattened_deserialize!, Option<T>, parse_json(), recover_deserialization_error(), section() (+9 more)

### Community 822 - "workspace: Dismiss"
Cohesion: 0.13
Nodes (6): Dismiss, Entity<V>, focus_handle_is_reopenable(), ModalLayer, ModalOpenedEvent, ModalViewHandle

### Community 823 - "src: Choosing Language Servers"
Cohesion: 0.06
Nodes (34): Choosing Language Servers, Code Actions, Code Completion, Configuring Formatters, Configuring Language Servers, Configuring Supported Languages, Customizing Syntax Highlighting, Diagnostics (+26 more)

### Community 824 - "tab_switcher: activatepreviousitem"
Cohesion: 0.18
Nodes (22): test_reorder_channels_independently_of_favorites(), test_reorder_favorite_channels_independently_of_channels(), assert_match_at_position(), assert_match_selection(), assert_tab_switcher_is_closed(), get_active_tab_switcher(), init_test(), open_buffer() (+14 more)

### Community 825 - "shell_command_parser: Assignment"
Cohesion: 0.09
Nodes (24): extract_commands_from_assignment(), extract_commands_from_command_prefix(), extract_commands_from_command_suffix(), extract_commands_from_parameter_expr(), extract_commands_from_prefix_or_suffix_item(), extract_commands_from_program(), extract_commands_from_simple_command(), extract_commands_from_word() (+16 more)

### Community 826 - "agent: basestrategy_as"
Cohesion: 0.08
Nodes (19): Author, BUILD_HOST, BUILD_TARGET, deserialize_parser_directories(), EMSCRIPTEN_TAG, GRAMMAR_NAME_REGEX, LanguageConfigurationJSON, Links (+11 more)

### Community 827 - "gpui_apple: cascade_list_for_languages"
Cohesion: 0.08
Nodes (4): append_system_fallbacks(), apply_features_and_fallbacks(), generate_fallback_array(), generate_feature_array()

### Community 828 - "gpui: conditional"
Cohesion: 0.11
Nodes (5): FunctionReflection<T>, InspectedElement, Inspector, InspectorElementRegistry, InspectorElementRenderer

### Community 829 - "language: .file_read_time()"
Cohesion: 0.07
Nodes (8): BenchFile, MTime, proto::Timestamp, DiskState, Deleted, Historic, New, Present

### Community 830 - "agent: decide_permission_for_path_all"
Cohesion: 0.06
Nodes (33): decide_permission_for_path_allows_when_no_traversal_issue(), decide_permission_for_path_confirms_traversal_to_confirmed_dir(), decide_permission_for_path_denies_edit_file_traversal_to_dotenv(), decide_permission_for_path_denies_traversal_to_denied_dir(), decide_permission_for_path_dot_segment_only(), decide_permission_for_path_most_restrictive_wins(), decide_permission_for_path_no_change_when_already_simple(), decide_permission_for_path_raw_deny_still_works() (+25 more)

### Community 831 - "agent: collapse_multiline_range()"
Cohesion: 0.09
Nodes (11): collapse_multiline_range(), GotoDefinitionKind, Declaration, Implementation, Symbol, Type, Navigated, No (+3 more)

### Community 832 - "agent: CompileConfig"
Cohesion: 0.12
Nodes (4): CompileConfig, CompileConfig<'a>, Loader, replace_dashes_with_underscores()

### Community 834 - "agent_ui: App"
Cohesion: 0.11
Nodes (10): SandboxGroup, SandboxRow, Domain, Message, Path, SandboxSection, SandboxStatusTooltip, DisabledForThread (+2 more)

### Community 835 - "dap_adapters: DebugpyLaunchMode"
Cohesion: 0.10
Nodes (11): DebugpyLaunchMode, AttachWithConnect, Normal, PythonDebugAdapter, .ADAPTER_NAME, .DEBUG_ADAPTER_NAME, .LANGUAGE_NAME, test_attach_with_connect_mode_generates_correct_arguments() (+3 more)

### Community 836 - "edit_prediction_metrics: summary.rs"
Cohesion: 0.09
Nodes (9): compute_summary(), LineFileAggregate, MeanAggregate, PredictionSummaryInput, PrfAggregate, QaSummaryData, RateAggregate, SumAggregate (+1 more)

### Community 837 - "extension_host: ExtensionKeyValueStore"
Cohesion: 0.08
Nodes (6): ExtensionKeyValueStore, ExtensionProject, latest::DownloadedFileType, latest::LanguageServerInstallationStatus, latest::SettingsLocation, WasmState

### Community 839 - "platform_title_bar: WindowButton"
Cohesion: 0.10
Nodes (13): WindowButton, Close, Maximize, Minimize, create_window_button(), LinuxWindowControls, WindowControl, WindowControlStyle (+5 more)

### Community 840 - "gpui_windows: Apartment"
Cohesion: 0.08
Nodes (11): Apartment, Cancellation, CancellationTimer, cancelling_one_request_does_not_cancel_its_owner_or_other_requests(), closed_owner_waits_for_active_dialog_but_queued_requests_stay_cancelled(), closing_owner_cancels_every_outstanding_dialog(), DialogOwner, dialogs_serialize_per_owner() (+3 more)

### Community 841 - "project: InnerTreeNode"
Cohesion: 0.16
Nodes (6): InnerTreeNode, LanguageServerTree, LanguageServerTreeNode, LaunchDisposition, ServersForWorktree, ServerTreeRebase

### Community 842 - "repl: build_grouped_entries()"
Cohesion: 0.11
Nodes (8): build_grouped_entries(), KernelPickerDelegate, KernelPickerEntry, Kernel, SectionHeader, KernelSelector, KernelSelector<T, TT>, OnSelect

### Community 843 - "sandbox_linux: dns.rs"
Cohesion: 0.10
Nodes (27): answer(), CLASS_IN, encode_name(), exists_on_host(), FAKE_BASE, FAKE_SIZE, FORMAT_ERROR, HOST_LOOKUP_CACHE (+19 more)

### Community 844 - "settings_ui: pages.rs"
Cohesion: 0.09
Nodes (13): SkillCreatorEvent, Dismissed, Saved, SkillCreatorOpenMode, Form, Install, Url, copy_skill_share_link() (+5 more)

### Community 845 - "onboarding: agentsetupbutton"
Cohesion: 0.14
Nodes (20): DARK_THEMES, FAMILY_NAMES, FEATURED_AGENT_IDS, LIGHT_THEMES, render_ai_section(), render_base_keymap_section(), render_basics_page(), render_import_settings_section() (+12 more)

### Community 846 - "github-find-top-duplicated-bugs.py: argparse"
Cohesion: 0.10
Nodes (5): build_markdown_body(), extract_duplicate_info(), fetch_canonical_issues_with_duplicates(), parse_args(), update_github_issue()

### Community 847 - "extension_api: attachrequest"
Cohesion: 0.07
Nodes (6): EXTENSION, register_extension!, register_extension(), set_language_server_installation_status(), ZED_API_VERSION, extension::SlashCommandOutputSection

### Community 849 - "collab_ui: CallDiagnosticsReport"
Cohesion: 0.12
Nodes (9): audio_issue_score(), call_diagnostics(), CallStatsModal, ExportedCallDiagnostics, format_audio_duration(), input_lag_rating(), metric_rating(), packet_loss_rating() (+1 more)

### Community 850 - "edit_prediction: fixed_line_window_around_curso"
Cohesion: 0.12
Nodes (27): fixed_line_window_around_cursor(), build_prompt(), build_prompt_input(), build_recent_change_blocks(), build_related_file_blocks(), clean_response_text(), limit_change_lines(), MAX_RECENT_CHANGE_BLOCKS (+19 more)

### Community 851 - "extension_host: ExtensionKeyValueStore"
Cohesion: 0.08
Nodes (6): ExtensionKeyValueStore, ExtensionProject, latest::DownloadedFileType, latest::LanguageServerInstallationStatus, latest::SettingsLocation, WasmState

### Community 852 - "extension_host: ExtensionKeyValueStore"
Cohesion: 0.08
Nodes (6): ExtensionKeyValueStore, ExtensionProject, latest::DownloadedFileType, latest::LanguageServerInstallationStatus, latest::SettingsLocation, WasmState

### Community 853 - "extension_host: ExtensionKeyValueStore"
Cohesion: 0.08
Nodes (6): ExtensionKeyValueStore, ExtensionProject, latest::DownloadedFileType, latest::LanguageServerInstallationStatus, latest::SettingsLocation, WasmState

### Community 854 - "gpui_windows: DirectManipulationEventHandler"
Cohesion: 0.10
Nodes (8): DirectManipulationEventHandler, DirectManipulationEventHandler_Impl, DirectManipulationHandler, float_equals(), GestureKind, None, Pinch, Scroll

### Community 856 - "ui: PreviewLikeButtons"
Cohesion: 0.14
Nodes (3): PreviewLikeButtons, TestTooltip, UpdateButton

### Community 857 - "zlog: sink.rs"
Cohesion: 0.09
Nodes (24): ANSI_BLUE, ANSI_BOLD, ANSI_GREEN, ANSI_MAGENTA, ANSI_RED, ANSI_RESET, ANSI_YELLOW, ENABLED_SINKS_FILE (+16 more)

### Community 858 - "compliance: .fmt()"
Cohesion: 0.12
Nodes (12): Reporter, ReviewFailure, NoPullRequestFound, Other, UnexpectedZippyAction, Unreviewed, ReviewSuccess, ApprovingComment (+4 more)

### Community 859 - "editor: addselectiontothread"
Cohesion: 0.11
Nodes (8): Rename, deploy_context_menu(), MenuPosition, PinnedToEditor, PinnedToScreen, MouseContextMenu, test_mouse_context_menu(), test_mouse_context_menu_at_pixel_snapped_scroll_position()

### Community 861 - "ui: GeneratingSpinner"
Cohesion: 0.08
Nodes (6): GeneratingSpinner, SpinnerLabel, SpinnerVariant, Dots, DotsVariant, Sand

### Community 862 - "dev_container: FeatureOptions"
Cohesion: 0.08
Nodes (11): FeatureOptions, Bool, Options, String, FeatureOptionValue, Bool, String, resolve_feature_order() (+3 more)

### Community 863 - "diagnostics: diagnostics_tests.rs"
Cohesion: 0.14
Nodes (21): active_diagnostics_dismiss_after_invalidation(), cycle_through_same_place_diagnostics(), go_to_diagnostic_with_severity(), init_test(), random_lsp_diagnostic(), randomly_update_diagnostics_for_path(), test_buffer_diagnostics(), test_buffer_diagnostics_multiple_servers() (+13 more)

### Community 864 - "editor: editor/src/edit_prediction.rs"
Cohesion: 0.07
Nodes (24): all_edits_insertions_or_deletions(), EditDisplayMode, DiffPopover, Inline, TabAccept, EditPredictionKeybindAction, Accept, Preview (+16 more)

### Community 865 - "fuzzy_nucleo: fuzzy_nucleo/src/strings.rs"
Cohesion: 0.21
Nodes (29): candidates(), empty_query_results(), match_strings_async(), options(), test_basic_match(), test_cancellation(), test_char_bag_prefilter(), test_empty_candidates() (+21 more)

### Community 867 - "vim: duplicate.rs"
Cohesion: 0.13
Nodes (11): Direction, Above, Below, test_selection_duplication(), test_selection_duplication_backwards(), test_selection_duplication_count(), test_selection_duplication_multibyte(), test_selection_duplication_multiline_multibyte() (+3 more)

### Community 868 - "src: Basedpyright"
Cohesion: 0.06
Nodes (31): Basedpyright, Basedpyright Configuration, Code Formatting & Linting, Configuration files, Configure Python Language Servers in Zed, Configuring Formatting, Configuring Ruff, Create a Virtual Environment (+23 more)

### Community 869 - "src: AI Assistance"
Cohesion: 0.06
Nodes (31): AI Assistance, All Terminal Settings, Alternate Scroll Mode, Clear Terminal, Configuring the Shell, Copy and Paste, Copy on Select, Cursor (+23 more)

### Community 870 - "workflows: danger.yml"
Cohesion: 0.06
Nodes (30): if, runs-on, steps, defaults, run, DANGER_GITHUB_API_BASE_URL, GITHUB_TOKEN, jobs (+22 more)

### Community 871 - "lints: render_consumer/src/lib.rs"
Cohesion: 0.17
Nodes (11): ClosureView, Counter, CounterOnce, Editor, notify_outside_render(), NotifyClosureView, NotifyView, ReaderView (+3 more)

### Community 872 - "xtask: autofix_pr.rs"
Cohesion: 0.09
Nodes (17): autofix_pr(), commit_changes(), download_patch_artifact(), PATCH_ARTIFACT_NAME, PATCH_FILE_PATH, upload_patch_artifact(), download_artifact(), DownloadArtifactStep (+9 more)

### Community 873 - "acp_thread: chunk_from_v1()"
Cohesion: 0.13
Nodes (23): chunk_from_v1(), legacy_compaction_preserves_statuses_patches_and_metadata_scopes(), update_from_v1(), accepts_empty_icons_without_discarding_resource_data(), annotations(), annotations_from_v1(), annotations_to_v1(), can_convert_to_v1() (+15 more)

### Community 874 - "agent_ui: cycling_modes_preserves_choice"
Cohesion: 0.12
Nodes (4): cycling_modes_preserves_choice_order_and_wraps(), setting_mode_saves_selected_mode_as_default(), TestAgentServer, TestSessionModes

### Community 875 - "benchmarks: markdown_renderer.rs"
Cohesion: 0.17
Nodes (21): build_markdown(), choose(), generate_markdown(), init_context(), markdown_language_registry(), markdown_render(), MarkdownBenchView, push_block_quote() (+13 more)

### Community 876 - "deployment: mapper.rs"
Cohesion: 0.15
Nodes (12): excludes(), is_absolute_local(), longest_local_mapping_wins(), names(), normalize_local(), PathMapper, relative_inside(), Resolved (+4 more)

### Community 877 - "edit_prediction: ts_error_count.rs"
Cohesion: 0.15
Nodes (23): build_case(), css_source(), css_source_with_errors(), go_source(), go_source_with_errors(), json_source(), json_source_with_errors(), parsed_cases() (+15 more)

### Community 878 - "edit_prediction_context: CachedDefinition"
Cohesion: 0.15
Nodes (12): CachedDefinition, CachedRelatedFile, CacheEntry, DefinitionTask, CacheHit, CacheMiss, is_tombi_lsp_in_toml(), process_definition() (+4 more)

### Community 879 - "editor: inline_input.rs"
Cohesion: 0.14
Nodes (12): Editor, INLINE_INPUT_PREVIEW_MIN_LINES, InlineInputHistoryDirection, Newer, Older, InlineInputPreview, Error, Text (+4 more)

### Community 880 - "gpui_apple: gpui_apple/src/dispatcher.rs"
Cohesion: 0.08
Nodes (3): AppleActivity, set_audio_thread_priority(), trampoline()

### Community 881 - "gpui_macros: derive_app_context.rs"
Cohesion: 0.13
Nodes (22): derive_app_context(), derive_visual_context(), bench(), border_style_methods(), box_shadow_style_methods(), cursor_style_methods(), derive_action(), derive_app_context() (+14 more)

### Community 882 - "http_proxy: allowlist.rs"
Cohesion: 0.10
Nodes (21): covers_exact_only_matches_identical(), covers_subdomain_wildcard_subsumes_descendants(), display_round_trips_through_parse(), empty_allowlist_denies_everything(), Exact, Subdomain, HostPatternError, Empty (+13 more)

### Community 884 - "ui: AiSettingItem"
Cohesion: 0.09
Nodes (13): AiSettingItem, AiSettingItemSource, Custom, Extension, Registry, AiSettingItemStatus, Authenticating, AuthRequired (+5 more)

### Community 885 - "workspace: init()"
Cohesion: 0.16
Nodes (5): init(), ThemePreview, ThemePreviewPage, Overview, Typography

### Community 886 - "zlog: src/filter.rs"
Cohesion: 0.15
Nodes (22): default_filter_crate(), DEFAULT_FILTERS, LEVEL_ENABLED_MAX_CONFIG, LEVEL_ENABLED_MAX_DEFAULT, LEVEL_ENABLED_MAX_STATIC, level_filter_from_str(), precedence(), refresh_from_settings() (+14 more)

### Community 887 - "src: Check your current scale facto"
Cohesion: 0.07
Nodes (30): Check your current scale factor, Clicking links isn't working, Community, Could not start inotify, Downloading manually, Font rendering parameters, Forcing X11 scale factor, Forcing Zed to use a specific GPU (+22 more)

### Community 888 - "workflows: add_commented_closed_issue_to_"
Cohesion: 0.07
Nodes (29): if, runs-on, steps, timeout-minutes, COMMENT_USER_LOGIN, ISSUE_NUMBER, types, jobs (+21 more)

### Community 889 - "workflows: with"
Cohesion: 0.07
Nodes (30): with, app-id, assignees, author, base, body, branch, cache (+22 more)

### Community 890 - "compliance: OctocrabAuthorAssociation"
Cohesion: 0.08
Nodes (15): Approvable, AuthorAssociation, Collaborator, Contributor, FirstTimeContributor, FirstTimer, Mannequin, Member (+7 more)

### Community 891 - "settings: horizontal"
Cohesion: 0.07
Nodes (29): horizontal, vertical, show, outline_panel, auto_fold_dirs, auto_reveal_entries, button, default_width (+21 more)

### Community 892 - "editor: cancel_flycheck"
Cohesion: 0.13
Nodes (11): test_client_can_query_lsp_ext(), serialize_anchor(), serialize_selection(), apply_related_actions(), cancel_flycheck_action(), clear_flycheck_action(), expand_macro_recursively(), go_to_parent_module() (+3 more)

### Community 893 - "agent: InvalidRegexPattern"
Cohesion: 0.10
Nodes (23): InvalidRegexPattern, ToolPermissions, ToolRules, check_commands(), check_hardcoded_security_rules(), check_invalid_patterns(), decide_permission_for_path(), decide_permission_for_path_no_dots_early_return() (+15 more)

### Community 894 - "agent_ui: .find_thread_sandbox_error()"
Cohesion: 0.09
Nodes (4): ClickHandler, TerminalSandboxWarning, TerminalToolHeader, duration_alt_display()

### Community 895 - "debugger_ui: .focus_handle()"
Cohesion: 0.15
Nodes (7): AttachModalDelegate, Candidate, get_processes_for_project(), ModalIntent, AttachToProcess, ResolveProcessId, set_candidates()

### Community 897 - "editor: assert_code_lens_block_count()"
Cohesion: 0.14
Nodes (11): assert_code_lens_block_count(), build_code_lens_renderer(), CodeLensItem, CodeLensLine, displayed_title(), Editor, group_lenses_by_row(), rendered_text_matches() (+3 more)

### Community 899 - "editor: Editor"
Cohesion: 0.14
Nodes (7): Editor, SignatureHelpHiddenBy, AutoClose, Escape, Selection, SignatureHelpPopover, SignatureHelpState

### Community 901 - "encoding_selector: available_encodings()"
Cohesion: 0.16
Nodes (4): available_encodings(), EncodingSelector, EncodingSelectorDelegate, init()

### Community 902 - "extension_host: latest::DownloadedFileType"
Cohesion: 0.08
Nodes (3): latest::DownloadedFileType, latest::LanguageServerInstallationStatus, WasmState

### Community 903 - "gpui_linux: .get_serial()"
Cohesion: 0.15
Nodes (17): ActivationContext, Press, raw_selection_serial(), SelectionSerial, Serial, SerialKind, DataDevice, InputMethod (+9 more)

### Community 904 - "gpui_macros: gpui_macros/src/bench.rs"
Cohesion: 0.13
Nodes (10): bench(), error_to_stream(), is_std_rng(), Args, error_to_stream(), error_with_message(), generate_test_function(), parse_u64_array() (+2 more)

### Community 905 - "gpui_windows: ClusterAnalyzer<'t>"
Cohesion: 0.10
Nodes (5): ClusterAnalyzer<'t>, slice_from_nullable(), StringIndexConverter<'a>, test_cluster_map(), TextRenderer_Impl

### Community 906 - "json_schema_store: description"
Cohesion: 0.07
Nodes (29): description, markdownDescription, type, x-intellij-html-description, properties, default, description, markdownDescription (+21 more)

### Community 907 - "open_ai: StreamEvent"
Cohesion: 0.07
Nodes (29): StreamEvent, Completed, ContentPartAdded, ContentPartDone, Created, CustomToolCallInputDelta, CustomToolCallInputDone, Error (+21 more)

### Community 908 - "project: DefaultPrettier"
Cohesion: 0.17
Nodes (9): DefaultPrettier, format_with_prettier(), PrettierInstallation, Installed, NotInstalled, PrettierInstance, PrettierStore, LanguageServerRemoved (+1 more)

### Community 909 - "src: `-a`, `--add`"
Cohesion: 0.07
Nodes (29): `-a`, `--add`, Bash, CLI Reference, `--completions <SHELL>`, `--diff <OLD_PATH> <NEW_PATH>`, `-e`, `--existing`, Elvish, Exit Codes (+21 more)

### Community 910 - "workflows: with"
Cohesion: 0.07
Nodes (29): with, app-id, assignees, author, base, body, branch, cache (+21 more)

### Community 911 - "xtask: release_job"
Cohesion: 0.13
Nodes (17): build_docs_book(), BUILD_OUTPUT_DIR, check_docs(), deploy_docs(), deploy_docs_job(), deploy_docs_workflow_call(), deploy_nightly_docs(), docs_build_steps() (+9 more)

### Community 912 - "terminal: AlacHyperlink"
Cohesion: 0.08
Nodes (5): AlacHyperlink, Cell, Hyperlink, terminal_hyperlink_from_alacritty(), terminal_hyperlink_from_alacritty_keeps_alacritty_storage()

### Community 913 - "settings: on_create"
Cohesion: 0.07
Nodes (28): on_create, on_drop, on_paste, project_panel, auto_fold_dirs, auto_open, auto_reveal_entries, bold_folder_labels (+20 more)

### Community 914 - "acp_thread: diff_change_label()"
Cohesion: 0.12
Nodes (22): diff_change_label(), diff_change_paths(), check_coordinate_bounds(), leaves_nontext_operations_to_structured_changes(), normalize_quoted_paths(), parse_patch(), parses_add_delete_and_missing_newline(), parses_git_quoted_paths_and_file_operations() (+14 more)

### Community 915 - "agent: create_directory_tool.rs"
Cohesion: 0.17
Nodes (16): create_out_of_project_directory(), CreateDirectoryTool, .NAME, CreateDirectoryToolInput, init_test(), resolve_absolute_path(), test_create_directory_allows_global_skill_directory(), test_create_directory_out_of_project_creates_and_grants() (+8 more)

### Community 916 - "agent: init_test()"
Cohesion: 0.15
Nodes (16): init_test(), ListDirectoryTool, .NAME, platform_paths(), test_list_directory_empty_directory(), test_list_directory_error_cases(), test_list_directory_intra_project_symlink_no_authorization(), test_list_directory_no_authorization_for_normal_paths() (+8 more)

### Community 917 - "eval_utils: eval_cant_do()"
Cohesion: 0.11
Nodes (20): eval_cant_do(), eval_empty_buffer(), eval_single_cursor_edit(), eval_unclear(), exact_buffer_match(), InlineAssistantOutput, Failure, Malformed (+12 more)

### Community 918 - "cloud_api_types: cloud_api_types/src/extension."
Cohesion: 0.09
Nodes (19): ExtensionApiManifest, ExtensionMetadata, ExtensionProvides, AgentServers, ContextServers, DebugAdapters, Grammars, IconThemes (+11 more)

### Community 919 - "copilot_chat: IncompleteDetails"
Cohesion: 0.08
Nodes (28): IncompleteDetails, IncompleteReason, ContentFilter, MaxOutputTokens, ItemStatus, Completed, Incomplete, InProgress (+20 more)

### Community 920 - "dap_adapters: JsDebugAdapter"
Cohesion: 0.10
Nodes (7): JsDebugAdapter, .ADAPTER_NAME, .ADAPTER_NPM_NAME, .ADAPTER_PATH, .DWARF_DEBUGGING_PACKAGE_NAME, .DWARF_DEBUGGING_PACKAGE_VERSION, normalize_task_type()

### Community 921 - "editor: DiffHunkKey"
Cohesion: 0.13
Nodes (3): DiffHunkKey, DiffReviewOverlay, StoredReviewComment

### Community 922 - "eval_cli: run_index.py"
Cohesion: 0.18
Nodes (11): _entry_from_request(), index_path(), _load_entries(), lookup(), most_recent(), _normalize_entry(), recent(), record_run() (+3 more)

### Community 923 - "gpui: hover_targets()"
Cohesion: 0.08
Nodes (21): hover_targets(), MixedWorkload, RandomizedElementTreeMutationKind, .ALL, ChildBorder, ChildBounds, ChildClipping, ChildColor (+13 more)

### Community 924 - "gpui: BoundsTree<U>"
Cohesion: 0.11
Nodes (17): BoundsTree<U>, Key, NodeChildren, NodeKind, Internal, Leaf, test_insert(), test_random_iterations() (+9 more)

### Community 926 - "json_schema_store: fundingWay"
Cohesion: 0.08
Nodes (28): fundingWay, type, additionalProperties, description, properties, required, type, properties (+20 more)

### Community 927 - "language_models: authenticate_does_not_fetch_mo"
Cohesion: 0.12
Nodes (9): authenticate_does_not_fetch_models_when_ai_is_disabled(), authenticate_fetches_models_after_loading_api_key(), disables_model_event_stream_when_ai_is_disabled(), does_not_fetch_models_when_ai_is_disabled(), LlamaCppLanguageModelProvider, LoadingProgress, PROVIDER_ID, registry_refreshes_models_while_credentials_are_pending() (+1 more)

### Community 929 - "settings_profile_selector: display_name()"
Cohesion: 0.16
Nodes (5): display_name(), init(), SettingsProfileSelector, SettingsProfileSelectorDelegate, toggle_settings_profile_selector()

### Community 930 - "tabular_data_preview: EditorState"
Cohesion: 0.17
Nodes (7): EditorState, init(), init_test(), TabularDataPreviewPane, test_csv_preview_mapping_stays_valid_after_deleting_multiline_rows(), test_detects_tabular_files_outside_the_project(), test_json_lines_preview_recovers_after_edits()

### Community 932 - "workflows: catch_blank_issues.yml"
Cohesion: 0.07
Nodes (27): if, runs-on, steps, timeout-minutes, ISSUE_NUMBER, types, jobs, add-triage-label (+19 more)

### Community 933 - "workflows: congrats.yml"
Cohesion: 0.07
Nodes (27): if, outputs, runs-on, steps, if, needs, secrets, uses (+19 more)

### Community 934 - "workflows: env"
Cohesion: 0.10
Nodes (28): env, needs, runs-on, timeout-minutes, env, needs, runs-on, timeout-minutes (+20 more)

### Community 935 - "workflows: steps"
Cohesion: 0.10
Nodes (28): steps, if, needs, runs-on, steps, needs, runs-on, steps (+20 more)

### Community 936 - "fuzzy_nucleo: atomkind"
Cohesion: 0.11
Nodes (15): Cancelled, count_case_mismatches(), is_cancelled(), LengthPenalty, Off, On, MaxResults, All (+7 more)

### Community 937 - "zed: CliResponse"
Cohesion: 0.07
Nodes (20): CliResponse, Exit, Ping, PromptOpenBehavior, Stderr, Stdout, ipc::IpcSender<CliResponse>, IpcHandshake (+12 more)

### Community 938 - "edit_prediction_ui: ContextRetrievalStartedDebugEv"
Cohesion: 0.15
Nodes (3): ContextRetrievalStartedDebugEvent, EditPredictionContextView, RetrievalRun

### Community 939 - "git: .summary()"
Cohesion: 0.09
Nodes (11): GitSummary, .CONFLICT, .UNCHANGED, .UNTRACKED, TrackedSummary, .ADDED, .DELETED, .MODIFIED (+3 more)

### Community 941 - "gpui_util: ArcCow"
Cohesion: 0.12
Nodes (8): ArcCow, ArcCow<'a, str>, ArcCow<'a, T>, ArcCow<'a, [u8]>, Borrowed, Owned, ArcCow<'_, str>, ArcCow<'_, T>

### Community 942 - "http_proxy: pinned_host.rs"
Cohesion: 0.14
Nodes (14): forbidden_ip_covers_v4_ranges(), forbidden_ip_covers_v6_ranges(), is_forbidden_ip(), is_forbidden_ipv4(), PinnedHost, PinnedHostError, AllAddressesForbidden, NoAddresses (+6 more)

### Community 943 - "http_proxy: upstream.rs"
Cohesion: 0.13
Nodes (16): display_hides_credentials(), first_nonempty_env(), ignores_trailing_path(), no_proxy_bypasses_listed_host(), no_proxy_bypasses_loopback_implicitly(), no_proxy_with_cidr(), parse(), parses_bare_host_port() (+8 more)

### Community 944 - "language_core: SymbolKind"
Cohesion: 0.07
Nodes (27): SymbolKind, Array, Boolean, Class, Constant, Constructor, Enum, EnumMember (+19 more)

### Community 945 - "migrator: m_2025_01_29/keymap.rs"
Cohesion: 0.14
Nodes (18): CONTEXT_REPLACE, KEYMAP_PATTERNS, rename_context_key(), replace_action_argument_object_with_single_value(), replace_array_with_single_string(), replace_string_action(), STRING_REPLACE, TRANSFORM_ARRAY (+10 more)

### Community 946 - "onboarding: BaseKeymapSelector"
Cohesion: 0.14
Nodes (4): BaseKeymapSelector, BaseKeymapSelectorDelegate, init(), toggle()

### Community 947 - "settings_content: ModelMode"
Cohesion: 0.09
Nodes (13): ModelMode, OpenAiReasoningEffort, Box<T>, collections::BTreeMap<K, V>, collections::BTreeSet<T>, collections::HashMap<K, V>, collections::HashSet<T>, collections::IndexMap<K, V> (+5 more)

### Community 948 - "zed: GlobalMigrationNotification"
Cohesion: 0.13
Nodes (12): GlobalMigrationNotification, MigrationBanner, MigrationEvent, ContentChanged, MigrationNotification, MigrationType, Keymap, Settings (+4 more)

### Community 949 - "src: Additional AI Options"
Cohesion: 0.07
Nodes (27): Additional AI Options, Advanced Config and Productivity Tweaks, Collaboration in Zed vs. PyCharm, Common Shared Keybindings, Configuring GitHub Copilot, Debugging, Differences in Keybindings, Differences in User Interfaces (+19 more)

### Community 950 - "workflows: with"
Cohesion: 0.07
Nodes (27): with, app-id, assignees, author, base, body, branch, clean (+19 more)

### Community 951 - "workflows: pr_issue_labeler.yml"
Cohesion: 0.07
Nodes (26): if, runs-on, steps, timeout-minutes, types, jobs, check-authorship-and-label, name (+18 more)

### Community 952 - "workflows: with"
Cohesion: 0.07
Nodes (27): with, app-id, assignees, author, base, body, branch, cache (+19 more)

### Community 953 - "workflows: release.yml"
Cohesion: 0.07
Nodes (26): needs, runs-on, timeout-minutes, if, runs-on, timeout-minutes, if, runs-on (+18 more)

### Community 954 - "workflows: steps"
Cohesion: 0.19
Nodes (27): steps, steps, steps, steps, steps, steps, steps, steps (+19 more)

### Community 955 - "github-pr-cleanup.py: is_community_pull_request()"
Cohesion: 0.18
Nodes (19): is_community_pull_request(), author_commented_at(), author_login(), author_pushed_since(), bot_commented_at(), changes_requested_action(), cla_action(), clock_start() (+11 more)

### Community 956 - "crates_io_client: ApiToken"
Cohesion: 0.16
Nodes (3): ApiToken, RealCratesIoClient, RequestThrottle

### Community 957 - "lints: ()"
Cohesion: 0.13
Nodes (14): (), App, AppContext, Context, Context<'_, T>, Entity, Entity<T>, IntoElement (+6 more)

### Community 958 - "watch: ArcWake"
Cohesion: 0.16
Nodes (15): clone_copies_seen_version_but_fresh_receiver_sees_current_version(), coalesces_publications_to_the_latest_snapshot(), concurrent_send_poll_register_and_drop_completes_within_deadline(), constant_receiver_never_changes(), counting_waker(), dropping_changed_unregisters_its_waker_and_repoll_replaces_it(), equal_values_are_distinct_publications(), final_publication_precedes_channel_closure() (+7 more)

### Community 959 - "gpui_windows: clsctx_inproc_server"
Cohesion: 0.12
Nodes (7): add_dock_menu(), add_recent_folders(), create_destination_list(), create_shell_link(), JumpList, PKEY_TITLE, update_jump_list()

### Community 960 - "acp_thread: src/elicitation.rs"
Cohesion: 0.11
Nodes (13): content_value_to_v1(), elicitation_request_adapter_preserves_schema_scopes_and_extensions(), elicitation_response_and_error_adapter_preserves_payloads(), enum_option_from_v1(), items_from_v1(), property_from_v1(), request_from_v1(), request_id_from_v1() (+5 more)

### Community 961 - "acp_thread: ForegroundActivity"
Cohesion: 0.10
Nodes (7): ForegroundActivity, Idle, RequiresAction, Running, Unconfirmed, Unknown, SessionActivity

### Community 962 - "client: event_coalescer.rs"
Cohesion: 0.12
Nodes (13): COALESCE_TIMEOUT, EventCoalescer, PeriodData, SIMULATED_DURATION_FOR_SINGLE_EVENT, test_different_environment_under_timeout(), test_same_context_exceeding_timeout(), test_switching_environment_while_exceeding_timeout(), test_switching_environment_while_within_timeout() (+5 more)

### Community 963 - "collab: .can_see_channel()"
Cohesion: 0.11
Nodes (12): ChannelVisibility, Members, Public, i32, proto::ChannelVisibility, Model, Relation, Buffer (+4 more)

### Community 964 - "db: db/src/db.rs"
Cohesion: 0.10
Nodes (12): ALL_FILE_DB_FAILED, AppDatabase, CONNECTION_INITIALIZE_QUERY, DB_FILE_NAME, DB_INITIALIZE_QUERY, DomainMigration, FALLBACK_DB_NAME, static_connection! (+4 more)

### Community 965 - "deployment: servers/compose.yml"
Cohesion: 0.08
Nodes (25): USERS, build, ports, interval, retries, test, name, services (+17 more)

### Community 967 - "line_ending_selector: init()"
Cohesion: 0.17
Nodes (3): init(), LineEndingSelector, LineEndingSelectorDelegate

### Community 968 - "livekit_client: examples/test_app.rs"
Cohesion: 0.17
Nodes (4): LivekitWindow, main(), ParticipantState, quit()

### Community 969 - "project_symbols: init()"
Cohesion: 0.14
Nodes (6): init(), init_test(), ProjectSymbolsDelegate, symbol(), test_project_symbols(), test_project_symbols_renders_utf8_match()

### Community 970 - "repl: outputs/json.rs"
Cohesion: 0.17
Nodes (4): JsonView, test_json_view_from_value_root_expanded(), test_json_view_is_expanded_unknown_path(), test_json_view_toggle_path()

### Community 971 - "settings_content: ActionName"
Cohesion: 0.11
Nodes (6): ActionName, ActionWithArguments, add_deprecation(), add_description(), build_schema_produces_anyof_of_consts_per_name(), CommandAliasTarget

### Community 972 - "shell_command_parser: and_or_list_validation()"
Cohesion: 0.18
Nodes (24): and_or_list_validation(), assignment_validation(), combine_validations(), command_prefix_validation(), command_suffix_validation(), command_validation(), compound_command_validation(), compound_list_validation() (+16 more)

### Community 973 - "zlog: init_env_filter()"
Cohesion: 0.10
Nodes (23): init_env_filter(), scope_alloc_from_scope_str(), init_output_stdout(), get_env_config(), init(), init_test(), crate_name!, debug! (+15 more)

### Community 974 - "src: Adding Context {#adding-contex"
Cohesion: 0.08
Nodes (26): Adding Context {#adding-context}, Agent Panel, Changing Models {#changing-models}, Context Menu {#context-menu}, Creating New Threads {#new-thread}, Editing Messages {#editing-messages}, Errors and Debugging {#errors-and-debugging}, Favoriting Models (+18 more)

### Community 975 - "src: A Note on Licensing and Teleme"
Cohesion: 0.08
Nodes (26): A Note on Licensing and Telemetry, Additional AI Options, Advanced Config and Productivity Tweaks, Collaboration in Zed vs. RustRover, Common Shared Keybindings, Configuring GitHub Copilot, Debugging, Differences in Keybindings (+18 more)

### Community 976 - "src: Additional AI Options"
Cohesion: 0.08
Nodes (26): Additional AI Options, Advanced Config and Productivity Tweaks, Collaboration in Zed vs. WebStorm, Common Shared Keybindings, Configuring GitHub Copilot, Debugging, Differences in Keybindings, Differences in User Interfaces (+18 more)

### Community 977 - "workflows: steps"
Cohesion: 0.15
Nodes (26): steps, steps, steps, steps, steps, steps, steps, steps (+18 more)

### Community 979 - "lints: ui/entity_update_in_render.rs"
Cohesion: 0.19
Nodes (9): ClosureView, Counter, FakeEntity, FakeView, MutatingView, OnceMutatingView, ReadingView, update_outside_render() (+1 more)

### Community 980 - "mermaid_render: bytestext"
Cohesion: 0.17
Nodes (18): MermaidTheme, accent_css(), ACCENT_FILL_OPACITY, adjust_lightness(), blend_over_background(), build_injected_css(), chart_color_css(), git_branch_css() (+10 more)

### Community 981 - "ui: codefade"
Cohesion: 0.10
Nodes (16): impl_newtype_numeric_stepper_float!, impl_newtype_numeric_stepper_int!, impl_numeric_stepper_float!, impl_numeric_stepper_int!, impl_numeric_stepper_nonzero_int!, ValueChangeDirection, Decrement, Increment (+8 more)

### Community 982 - "gpui_macros: Comma"
Cohesion: 0.19
Nodes (11): Args, is_background_executor(), is_path_with_last_segment(), is_std_rng(), is_test_cx(), parse_args(), ParsedArgs, remove_background_executor() (+3 more)

### Community 983 - "ztracing: consume_all_tokens_as_debug_sp"
Cohesion: 0.08
Nodes (4): GLOBAL, __consume_all_tokens!, MAX_CALLSTACK_DEPTH, Span

### Community 984 - "agent: react.py"
Cohesion: 0.24
Nodes (3): ComputeCell, InputCell, ReactTest

### Community 985 - "agent_ui: MentionCrease"
Cohesion: 0.16
Nodes (8): MentionCrease, open_mention_uri(), open_migrated_rule(), open_skill_content_buffer(), open_skill_file(), open_thread(), reveal_in_project_panel(), skill_content_buffer_title()

### Community 986 - "deployment: copy_with_progress()"
Cohesion: 0.17
Nodes (6): copy_with_progress(), local_contract(), LocalClient, map_err(), stat_path(), to_stat()

### Community 987 - "edit_prediction_context: byte_offset_to_position()"
Cohesion: 0.16
Nodes (8): byte_offset_to_position(), byte_range_to_lsp_range(), DefinitionIndex, extract_declarations_from_tree(), extract_extra_definition_locations(), FileEntry, register_fake_definition_server(), word_at_position()

### Community 988 - "extension_host: ExtensionVersion"
Cohesion: 0.16
Nodes (4): ExtensionVersion, HeadlessExtensionStore, LoadedExtension, notify_extensions_changed()

### Community 989 - "extension_host: latest::DownloadedFileType"
Cohesion: 0.11
Nodes (4): latest::DownloadedFileType, latest::LanguageServerInstallationStatus, latest::SettingsLocation, WasmState

### Community 990 - "gpui: hot_reload.rs"
Cohesion: 0.16
Nodes (8): Child, Component, HotReload, message(), patch_for_process(), receive_patches(), receives_only_matching_patches_over_the_websocket(), Root

### Community 991 - "language_models: ApiCompatibleProviderConfigura"
Cohesion: 0.18
Nodes (4): ApiCompatibleProviderConfigurationView, ApiCompatibleProviderConfigurationView<S>, ApiCompatibleProviderState, ApiCompatibleProviderState<S>

### Community 993 - "settings_ui: push_a11y_text_runs()"
Cohesion: 0.13
Nodes (3): push_a11y_text_runs(), SettingsInputField, text_field_a11y_state()

### Community 994 - "ui: format_distance.rs"
Cohesion: 0.17
Nodes (13): DateTimeType, Local, Naive, distance_in_seconds(), distance_string(), format_distance(), format_distance_from_now(), FormatDistance (+5 more)

### Community 995 - "vim: digraph.rs"
Cohesion: 0.17
Nodes (14): DEFAULT_DIGRAPHS_MAP, Literal, lookup_digraph(), test_ctrl_v(), test_ctrl_v_control(), test_ctrl_v_escape(), test_digraph_custom(), test_digraph_find() (+6 more)

### Community 996 - "src: Cleaning Up Older Keymap Entri"
Cohesion: 0.08
Nodes (25): Cleaning Up Older Keymap Entries, Codestral {#codestral}, Configuring Edit Prediction Keybindings {#edit-predictions-keybinding}, Configuring Other Providers {#other-providers}, Configuring the Prediction Debounce, Configuring Zeta, Default Key Bindings, Disabling Automatic Edit Prediction (+17 more)

### Community 997 - "src: Configuration"
Cohesion: 0.08
Nodes (25): Configuration, Configuring a language server, Debug a Ruby script, Debug Rails server, Debugging, `erb-formatter`, Examples, Formatters (+17 more)

### Community 998 - "lints: exprkind"
Cohesion: 0.18
Nodes (8): EntityUpdateInRender, NotifyInRender, is_directly_in_render_method(), is_gpui_context(), is_gpui_entity_or_weak(), is_gpui_render_trait(), is_render_trait_impl(), is_unit_or_result_unit()

### Community 999 - "crates_io_client: CratesIoState"
Cohesion: 0.18
Nodes (4): CratesIoState, SimulatedCratesIoClient, CrateName, UserId

### Community 1000 - "xtask: workflows/extension_tests.rs"
Cohesion: 0.18
Nodes (20): cache_zed_extension_cli(), cargo_fmt_package(), check(), check_extension(), check_rust(), download_zed_extension_cli(), extension_job_defaults(), EXTENSION_RUST_TARGET (+12 more)

### Community 1001 - "skills: gpui-bench/SKILL.md"
Cohesion: 0.08
Nodes (23): Before-and-after method, `bench_batched_task`: per-iteration setup outside timing, `bench_iter`: synchronous functions and compute work, `bench_renderer`: frame production, `bench_task`: asynchronous work to completion, Benchmark-only APIs, Benchmarks outside GPUI, Choose the right GPUI measurement API (+15 more)

### Community 1002 - "aws_http_client: AwsConnector"
Cohesion: 0.11
Nodes (3): AwsHttpConnector, convert_to_async_body(), convert_to_sdk_body()

### Community 1003 - "bundle-windows.ps1: blob_store"
Cohesion: 0.09
Nodes (6): BuildRemoteServer(), BuildZedAndItsFriends(), UploadToBlobStore(), UploadToBlobStorePublic(), UploadToBlobStoreWithACL(), ParseZedWorkspace()

### Community 1004 - "agent_ui: .sandbox_confusable_findings()"
Cohesion: 0.16
Nodes (20): classify(), deduplicates_repeated_characters(), detects_bidi_override(), detects_cyrillic_homoglyph(), detects_zero_width_space(), is_bidi_control(), is_invisible(), is_non_ascii_space() (+12 more)

### Community 1005 - "agent_ui: ModelIcon"
Cohesion: 0.14
Nodes (4): ModelIcon, Name, Path, ModelSelectorListItem

### Community 1006 - "collab_ui: .update_entries()"
Cohesion: 0.09
Nodes (22): ListEntry, CallParticipant, Channel, ChannelEditor, ChannelInvite, ChannelNotes, Contact, ContactPlaceholder (+14 more)

### Community 1007 - "fs: .record_watcher_diagnostics()"
Cohesion: 0.13
Nodes (12): DiagnosticRecorder, recording_is_opt_in_and_bounded(), recording_keeps_raw_events_errors_and_rescans(), recording_snapshots_existing_roots_and_registration_errors(), recording_snapshots_retained_roots_without_registrations(), RecordingState, rescan_report_contains_ten_preceding_paths_and_reason(), RescanHistory (+4 more)

### Community 1008 - "gpui: window/a11y.rs"
Cohesion: 0.28
Nodes (15): active_descendant_honored_for_deep_descendant(), active_descendant_honored_when_container_focused(), active_descendant_ignored_when_focus_in_other_subtree(), active_descendant_ignored_when_nothing_focused(), active_descendant_in_unfocused_subtree_keeps_real_focus(), focus_is_ancestor_excludes_self_and_non_ancestors(), multiple_active_descendant_claims_panic_in_debug(), new_a11y() (+7 more)

### Community 1010 - "language_models: capability_cells_update_when_a"
Cohesion: 0.14
Nodes (11): capability_cells_update_when_a_model_loads(), CapabilityCells, compute_effective_models(), config_uses_live_capabilities_for_a_stable_model_id(), LiveCapabilities, merge_settings_into_models(), PROVIDER_NAME, read_recover() (+3 more)

### Community 1011 - "openai_subscribed: ChatGptModel"
Cohesion: 0.13
Nodes (12): ChatGptModel, Discovered, Gpt54, Gpt54Mini, Gpt55, Gpt56Luna, Gpt56Sol, Gpt56Terra (+4 more)

### Community 1012 - "remote_server: tests/command.rs"
Cohesion: 0.19
Nodes (20): install_helper(), launcher(), shell_launcher(), test_remote_command_inherits_default_shell_startup_environment(), test_remote_command_inherits_shell_environment(), test_remote_command_launcher_expands_home_in_nushell(), test_remote_command_launcher_expands_home_in_shell(), test_remote_command_launcher_quoting_in_tcsh() (+12 more)

### Community 1013 - "ui: E"
Cohesion: 0.13
Nodes (9): E, StyledTypography, TextSize, Default, Editor, Large, Small, Ui (+1 more)

### Community 1014 - "src: Aligning clauses with related"
Cohesion: 0.08
Nodes (24): Aligning clauses with related blocks, Auto-indentation, Bracket matching, Code injections, Code outline/structure, Customizing Semantic Token Styles, Extension-Provided Semantic Token Rules, Fallback captures (+16 more)

### Community 1015 - "src: Additional AI Options"
Cohesion: 0.08
Nodes (24): Additional AI Options, Advanced Config and Productivity Tweaks, Collaboration in Zed vs. IntelliJ, Common Shared Keybindings (Zed with JetBrains keymap ↔ IntelliJ), Configuring GitHub Copilot, Debugging, Differences in Keybindings, Differences in User Interfaces (+16 more)

### Community 1016 - "workflows: community_update_all_top_ranki"
Cohesion: 0.09
Nodes (23): GITHUB_TOKEN, jobs, update_top_ranking_issues, name, on, schedule, workflow_dispatch, permissions (+15 more)

### Community 1017 - "workflows: community_update_weekly_top_ra"
Cohesion: 0.09
Nodes (23): GITHUB_TOKEN, jobs, update_top_ranking_issues, name, on, schedule, workflow_dispatch, permissions (+15 more)

### Community 1018 - "workflows: good_first_issue_notifier.yml"
Cohesion: 0.08
Nodes (23): ISSUE_AUTHOR, ISSUE_NUMBER, ISSUE_TITLE, ISSUE_URL, if, runs-on, steps, types (+15 more)

### Community 1019 - "workflows: env"
Cohesion: 0.17
Nodes (24): env, env, env, env, env, env, ACCOUNT_NAME, AZURE_CLIENT_ID (+16 more)

### Community 1020 - "workflows: with"
Cohesion: 0.08
Nodes (24): with, advanced-security, app-id, authToken, cache, cachixArgs, check-latest, clean (+16 more)

### Community 1021 - "watch: ArcSwap"
Cohesion: 0.21
Nodes (9): NoSenderError, Changed, channel(), Receiver, Receiver<T>, Sender, Snapshot, State (+1 more)

### Community 1022 - "settings_ui: audioinputdevicename"
Cohesion: 0.17
Nodes (4): AudioTestWindow, open_audio_test_window(), open_test_microphone(), start_test_playback()

### Community 1023 - "cli: bash"
Cohesion: 0.10
Nodes (12): main(), Shell, Bash, Elvish, Fish, Nushell, PowerShell, Zsh (+4 more)

### Community 1024 - "collab: ChannelChatParticipantId"
Cohesion: 0.11
Nodes (11): Model, Relation, Channel, Model, Relation, Collaborators, HostUser, LanguageServers (+3 more)

### Community 1025 - "agent: AddSelectionsState"
Cohesion: 0.12
Nodes (10): AddSelectionsState, FormatTarget, Buffers, Ranges, SelectionHistory, SelectionHistoryEntry, SelectionHistoryMode, Normal (+2 more)

### Community 1026 - "channel: Channel"
Cohesion: 0.11
Nodes (4): Channel, channel_path_sorting_key(), ChannelIndex, ChannelPathsInsertGuard

### Community 1027 - "editor: editor/src/rewrap.rs"
Cohesion: 0.11
Nodes (16): char_len_with_expanded_tabs(), Editor, is_grapheme_ideographic(), is_grapheme_whitespace(), is_non_breaking_whitespace(), should_stay_with_preceding_ideograph(), test_string_size_with_expanded_tabs(), test_word_breaking_tokenizer() (+8 more)

### Community 1028 - "eval_cli: eval_cli/README.md"
Cohesion: 0.09
Nodes (21): Building, eval-cli, Exit codes, Linux x86_64, for Harbor/Pier sandboxes, Native, for local testing on the same OS, Running benchmarks, Standalone usage, Baselines (+13 more)

### Community 1029 - "gpui: Node"
Cohesion: 0.19
Nodes (4): Node, A11yNodeBuilder, A11ySubtreeBuilder<'a>, ROOT_NODE_ID

### Community 1030 - "gpui_util: gpui_util/src/lib.rs"
Cohesion: 0.11
Nodes (12): assert_logged(), CREATE_NO_WINDOW, defer(), Deferred, get_powershell(), get_windows_system_shell(), logging_preserves_diagnostics_and_callers(), maybe! (+4 more)

### Community 1031 - "json_schema_store: tsType"
Cohesion: 0.09
Nodes (23): tsType, type, x-intellij-language-injection, dependency, devDependency, optionalDependency, peerDependency, additionalProperties (+15 more)

### Community 1032 - "livekit_client: mock_client.rs"
Cohesion: 0.10
Nodes (12): AudioStream, LocalAudioTrack, LocalParticipant, LocalTrackPublication, LocalVideoTrack, play_remote_video_track(), RemoteAudioTrack, RemoteParticipant (+4 more)

### Community 1033 - "markdown: .events()"
Cohesion: 0.10
Nodes (20): build_footnote_definitions(), build_heading_slugs(), MarkdownEvent, Code, End, FootnoteReference, HardBreak, Html (+12 more)

### Community 1034 - "mermaid_render: postprocess.rs"
Cohesion: 0.13
Nodes (10): extract_svg_id(), ReaderIter, ReaderIter<'a>, collect_native_text_contents(), FALLBACK_TEXT_CLASS, is_fallback_group(), is_fallback_text(), process() (+2 more)

### Community 1035 - "project: job_debug_queue.rs"
Cohesion: 0.18
Nodes (10): CompletedJob, CompletedJobStatus, Finished, Skipped, format_duration(), GitJobDebugQueue, MAX_COMPLETED_JOBS, PendingJob (+2 more)

### Community 1036 - "project: CodeLensActionId"
Cohesion: 0.18
Nodes (7): CodeLensActionId, CodeLensData, CodeLensResolveTask, CodeLensTask, flatten_cache(), LspStore, Project

### Community 1037 - "project: .search_history()"
Cohesion: 0.12
Nodes (9): QueryInsertionBehavior, AlwaysInsert, ReplacePreviousIfContains, SearchHistory, SearchHistoryCursor, SearchInputKind, Exclude, Include (+1 more)

### Community 1038 - "settings_content: ExtendingSet<T>"
Cohesion: 0.11
Nodes (6): ExtendingSet<T>, ExtendingVec, ExtendingVec<T>, FeatureFlagsMap, SplicingVec, .REST

### Community 1039 - "theme: theme_settings_provider.rs"
Cohesion: 0.15
Nodes (10): GlobalThemeSettingsProvider, set_theme_settings_provider(), theme_settings(), ThemeSettingsProvider, String, UiDensity, Comfortable, Compact (+2 more)

### Community 1040 - "ui: styled_ext.rs"
Cohesion: 0.15
Nodes (4): E, elevated(), elevated_borderless(), StyledExt

### Community 1042 - "src: Adding Skills {#adding-skills}"
Cohesion: 0.09
Nodes (23): Adding Skills {#adding-skills}, Agent Path Boundaries {#agent-path-boundaries}, Bundled Resources {#bundled-resources}, Create your own {#create-your-own}, Editing Skill Files {#editing-skill-files}, Folder Structure {#folder-structure}, From the skills.sh Registry {#from-the-registry}, Frontmatter Fields {#frontmatter-fields} (+15 more)

### Community 1043 - "src: Anthropic {#anthropic}"
Cohesion: 0.09
Nodes (23): Anthropic {#anthropic}, Anthropic-Compatible Endpoints {#anthropic-compatible}, API Keys and Environment Variables {#api-keys}, Custom Anthropic Models {#anthropic-custom-models}, Custom DeepSeek Models {#deepseek-custom-models}, Custom Google AI Models {#google-ai-custom-models}, Custom Headers {#custom-headers}, Custom Mistral Models {#mistral-custom-models} (+15 more)

### Community 1044 - "src: Agent Panel"
Cohesion: 0.09
Nodes (23): Agent Panel, Collaboration Panels, Edit Predictions {#editor-ai}, Editor, Editor Completions, Snippets, Actions, Diagnostics {#editor-lsp}, Editor Inlay Hints, Editor Scrollbar and Minimap {#editor-scrollbar}, Editor Tabs (+15 more)

### Community 1045 - "xtask: DocumentMut"
Cohesion: 0.17
Nodes (17): check_git_clean(), check_workspace_root(), publish_crate(), publish_dependencies(), publish_gpui(), PublishGpuiArgs, read_gpui_version(), run_command() (+9 more)

### Community 1046 - "workflows: deploy_nightly_docs.yml"
Cohesion: 0.09
Nodes (22): defaults, run, if, permissions, secrets, uses, with, jobs (+14 more)

### Community 1047 - "tests: a11y_atspi_test.py"
Cohesion: 0.14
Nodes (12): assert_count(), click(), do_action_by_name(), find_app(), find_by_role(), find_by_role_and_label(), get_counter(), get_numeric_value() (+4 more)

### Community 1048 - "crates_io_client: bad_request()"
Cohesion: 0.15
Nodes (14): bad_request(), simulated_user(), SimulatedCrate, Owner, Team, User, OwnerLogin, CratesIoUser (+6 more)

### Community 1049 - "AGENTS.md: AGENTS.md"
Cohesion: 0.09
Nodes (21): Actions, After any agentic session, Build guidelines, Concurrency, Context, Crash Investigation, Elements, Entities (+13 more)

### Community 1050 - "edit_prediction_cli: AnthropicResponse"
Cohesion: 0.16
Nodes (4): Message, Batch, BatchingLlmClient, message_content_to_string()

### Community 1051 - "extension_api: fetch"
Cohesion: 0.13
Nodes (3): fetch, HttpRequest, HttpRequestBuilder

### Community 1052 - "settings: BaseKeymapContent"
Cohesion: 0.10
Nodes (11): BaseKeymap, Atom, Cursor, Emacs, JetBrains, None, .OPTIONS, SublimeText (+3 more)

### Community 1053 - "agent_ui: test_data_retention_error_maps"
Cohesion: 0.10
Nodes (21): test_data_retention_error_maps_from_provider_error(), test_payment_required_from_zed_uses_upgrade_prompt(), test_payment_required_preserves_non_zed_provider_message(), test_provider_rejection_preserves_provider_message(), test_thread_error_protocol_codes_preserve_context_and_data(), ThreadError, ApiError, AuthenticationFailed (+13 more)

### Community 1054 - "fs: .as_fake()"
Cohesion: 0.11
Nodes (5): FakeHandle, FileHandle, read_dir_entries(), std::fs::File, watch()

### Community 1055 - "fuzzy_nucleo: case_penalty()"
Cohesion: 0.18
Nodes (13): case_penalty(), candidates(), distance_between_paths(), get_filename_match_bonus(), get_tail_proximity_bonus(), match_fixed_path_set(), path_match_helper(), PathMatchCandidate (+5 more)

### Community 1056 - "git_ui: .show_commit_message_error()"
Cohesion: 0.10
Nodes (9): GitPanelAddon, StashKind, All, Staged, Tracked, StashMessageModal, TrashCancel, Cancel (+1 more)

### Community 1057 - "gpui: .action_json_schema()"
Cohesion: 0.15
Nodes (4): ActionBuilder, ActionData, ActionRegistry, MacroActionData

### Community 1058 - "gpui: profiler/actions.rs"
Cohesion: 0.16
Nodes (6): ACTION_STATISTICS, ActionStatistics, ActionTiming, save_action_timing(), take_action_stats(), update_running_action()

### Community 1060 - "migrator: m_2025_03_29/settings.rs"
Cohesion: 0.13
Nodes (10): replace_setting_name(), replace_setting_value(), SETTINGS_PATTERNS, migrate_drag_and_drop_selection(), SETTINGS_PATTERNS, rename_agent_font_size(), SETTINGS_PATTERNS, replace_setting_value() (+2 more)

### Community 1061 - "migrator: m_2025_05_08/settings.rs"
Cohesion: 0.16
Nodes (12): comment_duplicated_agent(), SETTINGS_PATTERNS, is_project_panel_open_file_on_paste(), rename_open_file_on_paste_setting(), replace_open_file_on_paste_setting_value(), SETTINGS_PATTERNS, is_enable_preview_from_code_navigation(), rename_enable_preview_from_code_navigation_setting() (+4 more)

### Community 1062 - "project: can_resolve_link()"
Cohesion: 0.17
Nodes (5): can_resolve_link(), DocumentLinkId, LspStore, Resolved, Resolving

### Community 1064 - "watch: Changed"
Cohesion: 0.16
Nodes (10): Changed, channel(), Receiver, Receiver<T>, Sender, Sender<T>, State, test_basic_watch() (+2 more)

### Community 1065 - "src: Automatic scenario creation"
Cohesion: 0.09
Nodes (22): Automatic scenario creation, Breakpoints, Build tasks, Button, Configuration, Customizing Debug Adapters, Debugger, Dock (+14 more)

### Community 1066 - "workflows: .github/workflows/run_tests.ym"
Cohesion: 0.09
Nodes (21): if, runs-on, timeout-minutes, if, needs, runs-on, env, CARGO_INCREMENTAL (+13 more)

### Community 1067 - "workflows: slack_notify_label_created.yml"
Cohesion: 0.09
Nodes (21): CREATED_BY, DEFAULT_BRANCH, LABEL_COLOR, LABEL_DESCRIPTION, LABEL_NAME, REPO_URL, SLACK_WEBHOOK_URL, jobs (+13 more)

### Community 1068 - "workflows: update_duplicate_magnets.yml"
Cohesion: 0.10
Nodes (21): GITHUB_TOKEN, jobs, update-duplicate-magnets, name, on, schedule, workflow_dispatch, permissions (+13 more)

### Community 1069 - "danger: ref_util"
Cohesion: 0.13
Nodes (11): entries, FIXTURES_PATHS, gpuiCrates, includesIssueUrl, MIGRATION_SCHEMA_FILES, modifiedFixtures, modifiedSchemaFiles, { prHygiene } (+3 more)

### Community 1070 - "shell_command_parser: AndOrList"
Cohesion: 0.13
Nodes (13): extract_commands_from_and_or_list(), extract_commands_from_command(), extract_commands_from_compound_command(), extract_commands_from_compound_list(), extract_commands_from_do_group(), extract_commands_from_function_body(), extract_commands_from_io_redirect(), extract_commands_from_pipeline() (+5 more)

### Community 1071 - "agent_servers: api_key_for_gemini_cli()"
Cohesion: 0.19
Nodes (3): api_key_for_gemini_cli(), CustomAgentServer, is_registry_agent()

### Community 1072 - "edit_prediction_context: edit_prediction_context_tests."
Cohesion: 0.23
Nodes (16): assert_definitions(), assert_related_files(), assert_related_files_impl(), assert_related_files_with_orders(), format_excerpts(), init_test(), setup_fake_lsp(), test_assemble_excerpts() (+8 more)

### Community 1073 - "snippet: CompletionEdit"
Cohesion: 0.27
Nodes (18): CompletionEdit, parse_choices(), parse_int(), parse_snippet(), parse_tabstop(), Snippet, TabStop, tabstop_choices() (+10 more)

### Community 1074 - "editor: NextScrollCursorCenterTopBotto"
Cohesion: 0.19
Nodes (6): NextScrollCursorCenterTopBottom, Bottom, Center, Top, SCROLL_CENTER_TOP_BOTTOM_DEBOUNCE_TIMEOUT, Editor

### Community 1075 - "test-extension: CodeLabelSpan"
Cohesion: 0.14
Nodes (6): CodeLabelSpan, lsp::CompletionKind, lsp::CompletionKind, CompletionKind, strip_newlines_from_detail(), TestExtension

### Community 1076 - "fuzzy_nucleo: match_path_sets()"
Cohesion: 0.13
Nodes (4): match_path_sets(), PathMatch, PathMatchCandidateSet, TestCandidateSet

### Community 1077 - "git_ui_core: git_ui_core.rs"
Cohesion: 0.22
Nodes (9): BranchPickerBuilder, BranchPickerBuilderGlobal, build_branch_picker(), FileHistoryOpener, FileHistoryOpenerGlobal, GitPickerPopover, open_file_history(), set_branch_picker_builder() (+1 more)

### Community 1078 - "gpui: toggle_headless.rs"
Cohesion: 0.23
Nodes (12): describe_todo(), display_environment(), handle_command(), main(), open_window(), print_todos(), run_command(), SAMPLE_TODOS (+4 more)

### Community 1079 - "gpui_web: test_canvas_fallback.mjs"
Cohesion: 0.13
Nodes (10): browser, log, output, browser, log, output, browser, log (+2 more)

### Community 1080 - "livekit_client: RemoteTrack"
Cohesion: 0.15
Nodes (5): RemoteTrack, Audio, Video, LocalTrackPublication, RemoteTrackPublication

### Community 1081 - "lsp_locations: active_picker()"
Cohesion: 0.18
Nodes (18): active_picker(), assert_cmd_click(), cmd_click_request(), references(), rust_cx(), test_cmd_click_cached_locations(), test_cmd_click_cold_locations(), test_cmd_click_empty_hover_requeries() (+10 more)

### Community 1082 - "mermaid_render: element_fixup.rs"
Cohesion: 0.20
Nodes (12): ElementFixup, ElementFixup<I>, font_family_declaration_value(), font_style(), is_bad_rect(), is_hardcoded_text_fill(), process(), push_font_style() (+4 more)

### Community 1083 - "project: .show_notification()"
Cohesion: 0.10
Nodes (19): LspStoreEvent, DiagnosticsUpdated, DiskBasedDiagnosticsFinished, DiskBasedDiagnosticsStarted, LanguageDetected, LanguageServerLog, LanguageServerUpdate, Notification (+11 more)

### Community 1084 - "project: create_remote_shell()"
Cohesion: 0.23
Nodes (5): create_remote_shell(), init_remote_client(), Project, remote_exec_in_shell_uses_non_interactive_command(), remote_terminal_launch_uses_interactive_command()

### Community 1086 - "ui: control_characters.rs"
Cohesion: 0.18
Nodes (20): CONTROL_PICTURES_START, DELETE_SYMBOL, LINE_FEED_SYMBOL, printable_substitute(), replace_control_characters(), replace_control_characters_remapping_offsets(), test_character_count_is_preserved(), test_clamps_offsets_past_the_end() (+12 more)

### Community 1087 - ".doc-examples: reference.md"
Cohesion: 0.10
Nodes (20): `copy_path` {#copy-path}, `create_directory` {#create-directory}, `delete_path` {#delete-path}, `diagnostics` {#diagnostics}, `edit_file` {#edit-file}, Edit Tools {#edit-tools}, `fetch` {#fetch}, `find_path` {#find-path} (+12 more)

### Community 1088 - "src: Built-in Security Rules"
Cohesion: 0.10
Nodes (21): Built-in Security Rules, Configuration, Examples, File Editing: Protect Sensitive Files, File Edits Inside the Project {#file-edits-inside-the-project}, Global Auto-Approve, How It Works, MCP Tools (+13 more)

### Community 1089 - "src: `copy_path`"
Cohesion: 0.10
Nodes (21): `copy_path`, `create_directory`, `delete_path`, `diagnostics`, `edit_file`, Edit Tools, `fetch`, `find_path` (+13 more)

### Community 1090 - "src: Avoiding continual rebuilds"
Cohesion: 0.10
Nodes (21): Avoiding continual rebuilds, Baseline Images, Building Zed for macOS, Building Zed from Source, Cargo errors claiming that a dependency is using unstable features, Dependencies, During the incident, Error compiling metal shaders (+13 more)

### Community 1091 - "src: Auto-reveal"
Cohesion: 0.10
Nodes (21): Auto-reveal, Creating Files and Directories, Cut, Copy, and Paste, Diagnostics, Directory Folding, Drag and Drop, Duplicate, File Operations (+13 more)

### Community 1092 - "src: Cell mode"
Cohesion: 0.10
Nodes (21): Cell mode, Changing which kernel is used per language {#changing-kernels}, Conda Environment, Debugging Kernelspecs, Getting started, Global environment, How Zed Recommends Kernels, Installation (+13 more)

### Community 1093 - "workflows: maintainer_edits_nudge.yml"
Cohesion: 0.10
Nodes (20): concurrency, cancel-in-progress, group, jobs, nudge-contributor, name, if, permissions (+12 more)

### Community 1094 - "workflows: env"
Cohesion: 0.11
Nodes (21): env, if, needs, runs-on, timeout-minutes, env, if, needs (+13 more)

### Community 1095 - "settings: git_panel"
Cohesion: 0.10
Nodes (20): git_panel, button, collapse_untracked_diff, commit_editor, commit_title_max_length, date_style, default_width, diff_stats (+12 more)

### Community 1096 - "agent: DeletePathTool"
Cohesion: 0.18
Nodes (10): DeletePathTool, .NAME, init_test(), test_delete_path_global_skill_directory(), test_delete_path_global_skill_file(), test_delete_path_rejects_global_skills_root(), test_delete_path_symlink_escape_confirm_requires_single_approval(), test_delete_path_symlink_escape_denied() (+2 more)

### Community 1097 - "cloud_api_types: internal_api.rs"
Cohesion: 0.14
Nodes (19): ChannelMember, ChannelMemberKind, Invitee, Member, ChannelMemberRole, Admin, Banned, Guest (+11 more)

### Community 1098 - "editor: bracket_colors_markup()"
Cohesion: 0.16
Nodes (17): bracket_colors_markup(), Editor, editor_bracket_colors_markup(), separate_with_comment_lines(), test_basic_bracket_colorization(), test_bracket_colorization_after_language_swap(), test_bracket_colorization_chunks(), test_bracket_colorization_retained_during_reparse() (+9 more)

### Community 1099 - "editor: CollaborationHub"
Cohesion: 0.13
Nodes (3): CollaborationHub, Entity<Project>, Range<MultiBufferRow>

### Community 1100 - "extensions_ui: Feature"
Cohesion: 0.10
Nodes (20): Feature, AgentClaude, AgentCodex, AgentGemini, ExtensionBasedpyright, ExtensionRuff, ExtensionTailwind, ExtensionTy (+12 more)

### Community 1102 - "git_ui: git_runtime_diagnostics.rs"
Cohesion: 0.20
Nodes (15): assert_redacts(), collect_linux_proc_info(), collect_macos_git_child_diagnostics(), collect_process_tree(), descendants_of(), gather(), leaves_real_argv_alone(), redact_env_var_entry() (+7 more)

### Community 1103 - "git_ui: remote_output.rs"
Cohesion: 0.18
Nodes (18): extract_pull_request_link(), extract_url(), format_output(), PULL_REQUEST_HINTS, RemoteAction, Fetch, Pull, Push (+10 more)

### Community 1104 - "gpui_macros: src/derive_inspector_reflectio"
Cohesion: 0.18
Nodes (7): derive_inspector_reflection(), extract_cfg_attributes(), extract_doc_comment(), generate_reflected_trait(), is_called_from_gpui_crate(), MacroExpander, parse_expanded_items()

### Community 1105 - "language_models: add_message_content_part()"
Cohesion: 0.13
Nodes (6): add_message_content_part(), append_reasoning_content(), build_llama_cpp_request(), request_preserves_assistant_thinking_when_supported(), request_preserves_thinking_for_assistant_tool_calls_when_supported(), request_skips_assistant_thinking_when_unsupported()

### Community 1106 - "mistral: Model"
Cohesion: 0.10
Nodes (10): Model, CodestralLatest, Custom, Ministral14bLatest, Ministral3bLatest, Ministral8bLatest, MistralLargeLatest, MistralMediumLatest (+2 more)

### Community 1107 - "oauth_callback_server: oauth_callback_server.rs"
Cohesion: 0.18
Nodes (15): bind_callback_server(), BIND_MAX_ATTEMPTS, BIND_RETRY_DELAY, CANCEL_PATH, CANCEL_REQUEST_TIMEOUT, handle_oauth_callback_request(), html_escape(), oauth_callback_page() (+7 more)

### Community 1108 - "recent_projects: DisconnectedOverlay"
Cohesion: 0.17
Nodes (4): DisconnectedOverlay, Host, CollabGuestProject, RemoteServerProject

### Community 1110 - "theme: App"
Cohesion: 0.15
Nodes (5): App, GlobalSystemAppearance, GlobalTheme, init(), SystemAppearance

### Community 1111 - "workspace: .detach_and_prompt_err()"
Cohesion: 0.18
Nodes (4): markdown_style(), NotifyTaskExt, Task<anyhow::Result<R>>, Task<std::result::Result<R, E>>

### Community 1112 - "zlog: flush()"
Cohesion: 0.14
Nodes (7): flush(), extract_crate_name_from_module_path(), Logger, scoped_logger(), Timer, Zlog, ZLOG

### Community 1113 - "src: Agent-Specific Auth and Config"
Cohesion: 0.10
Nodes (20): Agent-Specific Auth and Config {#agent-auth-config}, Claude Agent {#claude-agent}, Codex {#codex-cli}, Common Agents {#common-agents}, Configuration Boundaries {#configuration-boundaries}, Copilot {#copilot}, Cursor {#cursor}, Custom Agents {#custom-agents} (+12 more)

### Community 1114 - "src: Additional AI Options"
Cohesion: 0.10
Nodes (20): Additional AI Options, Advanced Config and Productivity Tweaks, Collaboration in Zed vs. VS Code, Common Shared Keybindings (Zed <> VS Code), Configuring GitHub Copilot, Differences in Keybindings, Differences in User Interfaces, Different Keybindings (Zed <> VS Code) (+12 more)

### Community 1115 - "skills: skills/brand-writer/SKILL.md"
Cohesion: 0.10
Nodes (19): Avoid, Bad, Core Messages, Core Voice, Examples, Fixed, Good, Invocation (+11 more)

### Community 1116 - "workflows: steps"
Cohesion: 0.26
Nodes (20): steps, steps, steps, steps, steps, steps, steps, steps (+12 more)

### Community 1117 - "workflows: DRY_RUN"
Cohesion: 0.10
Nodes (20): DRY_RUN, GITHUB_TOKEN, PROJECT_NUMBER, env, id, name, run, uses (+12 more)

### Community 1118 - "xtask: Permissions"
Cohesion: 0.16
Nodes (12): Permissions, GenerateAppToken, GenerateAppToken<'a, Target, Unset>, GenerateAppToken<'a, Unset, Permissions>, RepositoryTarget, (Step<Use>, StepOutput), TokenPermissions, Contents (+4 more)

### Community 1119 - "settings: date_style"
Cohesion: 0.11
Nodes (19): date_style, show_avatar, git, blame, date_format, disable_git, enable_diff, enable_status (+11 more)

### Community 1120 - "gpui: backspace"
Cohesion: 0.16
Nodes (7): arrows_move_the_cursor(), bind_keys(), external_writes_clamp_the_cursor(), Harness, setup(), sibling_inputs_are_isolated(), typing_updates_the_shared_string()

### Community 1121 - "project: ChunksExact"
Cohesion: 0.15
Nodes (11): SemanticTokensEdit, applies_delta_edit(), applies_out_of_bounds_delta_edit_without_panic(), debug_parse_tokens(), extract_semantic_tokens(), modifier_names(), parses_sample_tokens(), raw_to_buffer_semantic_tokens() (+3 more)

### Community 1122 - "command_palette: command_palette/src/persistenc"
Cohesion: 0.18
Nodes (10): CommandPaletteDB, .MIGRATIONS, .NAME, SerializedCommandInvocation, SerializedCommandUsage, test_deletes_all_history_for_one_command(), test_gets_usage_history(), test_handles_max_invocation_entries() (+2 more)

### Community 1123 - "context_server: .client()"
Cohesion: 0.12
Nodes (7): InitializedContextServerProtocol, ServerCapability, Experimental, Logging, Prompts, Resources, Tools

### Community 1127 - "json_schema_store: additionalProperties"
Cohesion: 0.11
Nodes (19): additionalProperties, description, type, properties, type, description, type, description (+11 more)

### Community 1128 - "json_schema_store: description"
Cohesion: 0.11
Nodes (19): description, properties, type, person, description, format, type, description (+11 more)

### Community 1130 - "mermaid_render: mermaid_render.rs"
Cohesion: 0.25
Nodes (16): AccentColor, cyclic_subgraphs_do_not_crash(), er_multibyte_attribute_does_not_crash(), mermaid_diagram_with_mixed_weight_combining_marks_does_not_panic(), render_to_svg(), svg_renderer(), accent_colors_auto_applied_to_nodes(), check_svg_issues() (+8 more)

### Community 1131 - "path: windows_prefix.rs"
Cohesion: 0.17
Nodes (9): is_verbatim_sep(), parse_drive(), parse_drive_exact(), parse_next_component(), parse_prefix(), PrefixParser, PrefixParser<'a, LEN>, PrefixParserSlice (+1 more)

### Community 1132 - "picker: head.rs"
Cohesion: 0.21
Nodes (7): EmptyHead, Head, Editor, Empty, ErasedEditorEvent, Blurred, BufferEdited

### Community 1133 - "project: LazyGlobSet"
Cohesion: 0.14
Nodes (4): LazyGlobSet, RenameActionPredicate, RenamePathsWatchedForServer, test_fetch_inlay_hints_after_buffer_edit()

### Community 1134 - "sandbox_linux: DESIGN.md"
Cohesion: 0.11
Nodes (18): `Ask` mode, Async supervisor, Datagram unix sockets, Design, Exact errors for non-blocking connects, Future improvements, Implementation, Implementation ideas (ai generated) (+10 more)

### Community 1135 - "x_ai: Model"
Cohesion: 0.11
Nodes (8): Model, Custom, Grok420NonReasoning, Grok420Reasoning, Grok43, Grok45, Grok46, Grok47

### Community 1136 - ".conventions: .conventions/brand-writer/SKIL"
Cohesion: 0.11
Nodes (18): Avoid, Bad, Core Messages, Core Voice, Examples, Fixed, Good, Invocation (+10 more)

### Community 1137 - "src: Terminal"
Cohesion: 0.11
Nodes (19): Terminal, Terminal: Alternate Scroll, Terminal: Blinking, Terminal: Copy On Select, Terminal: Cursor Shape, Terminal: Dock, Terminal: Env, Terminal: Font Fallbacks (+11 more)

### Community 1138 - "workflows: extensions/workflows/run_tests"
Cohesion: 0.11
Nodes (18): permissions, uses, concurrency, cancel-in-progress, group, defaults, run, jobs (+10 more)

### Community 1139 - "workflows: slack_notify_community_automat"
Cohesion: 0.11
Nodes (18): SLACK_WEBHOOK_URL, WF_NAME, WF_URL, jobs, notify-slack, name, if, runs-on (+10 more)

### Community 1140 - "privacy-policy.md: privacy-policy.md"
Cohesion: 0.11
Nodes (18): Changes to this Privacy Policy, Children’s Privacy, Contact Information, How to Exercise Your Privacy Rights, How We Disclose the Personal Data We Collect, How We Use the Personal Data We Collect, International Visitors, Introduction (+10 more)

### Community 1141 - "compliance: AutomatedChangeFailure"
Cohesion: 0.12
Nodes (13): AutomatedChangeFailure, AuthorMismatch, InvalidSignature, MissingCommitData, NoMentionInTitle, NotSigned, UnexpectedCoAuthors, UnexpectedFiles (+5 more)

### Community 1142 - "crates_io_client: MIN_REQUEST_INTERVAL"
Cohesion: 0.20
Nodes (12): MIN_REQUEST_INTERVAL, crate_json(), github_config_json(), RecordedRequest, recording_fake_http_client(), test_error_responses_distinguish_permission_and_upstream_failures(), test_github_config_requests_match_crates_io_api(), test_list_crates_requests_next_page_only_after_full_page() (+4 more)

### Community 1143 - "crates_io_client: listed()"
Cohesion: 0.21
Nodes (11): listed(), Crate, CrateDetails, CrateListQuery, GitHubAccountId, GitHubConfig, GitHubConfigId, NewGitHubConfig (+3 more)

### Community 1144 - "zed: ActionStatistics"
Cohesion: 0.24
Nodes (6): PerfIssue, Action, Background, Foreground, Reporter, ReportMade

### Community 1145 - "gpui_windows: c_uint"
Cohesion: 0.16
Nodes (3): MouseWheelSettings, WindowsSystemSettings, AT_RECURSIVE

### Community 1146 - "repl: codehandler"
Cohesion: 0.16
Nodes (9): clean_markdown_tables(), html_to_markdown(), normalize_table_row(), test_html_table_to_markdown(), test_html_with_headings(), test_pandas_dataframe_html(), test_style_tags_are_filtered(), test_table_format_normalized() (+1 more)

### Community 1147 - "agent_servers: init_test()"
Cohesion: 0.24
Nodes (13): init_test(), new_test_thread(), run_until_first_tool_call(), send_and_wait_for_turn(), stub_thread(), test_basic(), test_cancel(), test_e2e_helper_preserves_stop_before_receipt() (+5 more)

### Community 1148 - "agent: Canvas"
Cohesion: 0.22
Nodes (5): Canvas, Format, A8, Rgb24, Rgba32

### Community 1149 - "agent: Canvas"
Cohesion: 0.22
Nodes (5): Canvas, Format, A8, Rgb24, Rgba32

### Community 1150 - "copilot_chat: CopilotChatStatus"
Cohesion: 0.14
Nodes (16): CopilotChatStatus, Authorized, Error, SignedOut, SigningIn, Starting, AccessTokenResponse, DEVICE_CODE_GRANT_TYPE (+8 more)

### Community 1152 - "gpui_util: F"
Cohesion: 0.20
Nodes (5): F, LogErrorFuture, LogErrorWithBacktraceFuture, TryFutureExtBacktrace, UnwrapFuture

### Community 1153 - "language_models: language_models.rs"
Cohesion: 0.28
Nodes (13): ai_disabled(), CompatibleProviderKind, Anthropic, OpenAi, CompatibleProviders, init(), init_test(), provider_icons() (+5 more)

### Community 1154 - "openai_subscribed: .new()"
Cohesion: 0.17
Nodes (5): make_state_with_credentials_provider(), State, test_initial_load_restores_persisted_credentials(), test_sign_out_completes_fully(), test_slow_model_catalog_does_not_block_initial_load()

### Community 1155 - "outline_panel: BufferOutlines"
Cohesion: 0.21
Nodes (4): BufferOutlines, IndexedOutline, OutlineChildren, Invalidated

### Community 1156 - "platform_title_bar: App"
Cohesion: 0.16
Nodes (6): WindowsCaptionButton, Close, Maximize, Minimize, Restore, WindowsWindowControls

### Community 1157 - "prettier: prettier_server.js"
Cohesion: 0.17
Nodes (12): { Buffer }, fs, handleBuffer(), handleMessage(), makeError(), { once }, path, Prettier (+4 more)

### Community 1158 - "project: src/color_extractor.rs"
Cohesion: 0.20
Nodes (14): from_hex(), from_hsl(), from_rgb(), HEX, parse(), parse_component(), ParseMode, Relaxed (+6 more)

### Community 1159 - "sandbox: sandbox/README.md"
Cohesion: 0.11
Nodes (17): Architecture, Blocking IPC-socket escapes (seccomp), Code design, `HostFilesystemLocation`, Implementation, Linux, MacOS, Overview (+9 more)

### Community 1160 - "search: ActionButtonState"
Cohesion: 0.12
Nodes (8): ActionButtonState, Disabled, Toggled, alignment_element(), filter_search_results_input(), input_base_styles(), render_local_action_button(), render_text_input()

### Community 1161 - "settings_content: settings_content/src/title_bar"
Cohesion: 0.15
Nodes (11): String, TitleBarSettingsContent, TitleBarVisibility, Always, HideInFullScreen, Never, window_button_layout_schema(), WindowButtonLayoutContent (+3 more)

### Community 1162 - "settings_sync: GlobalSettingsSync"
Cohesion: 0.24
Nodes (5): GlobalSettingsSync, Resolution, UseLocal, UseRemote, SettingsSync

### Community 1163 - "settings_ui: font_picker.rs"
Cohesion: 0.29
Nodes (3): font_picker(), FontPicker, FontPickerDelegate

### Community 1164 - "shell_command_parser: TerminalCommandValidation"
Cohesion: 0.11
Nodes (18): TerminalCommandValidation, Safe, Unsafe, Unsupported, test_validate_terminal_command_rejects_arithmetic_expansion(), test_validate_terminal_command_rejects_arithmetic_for_clause_unconditionally(), test_validate_terminal_command_rejects_backticks(), test_validate_terminal_command_rejects_braced_parameter_expansion() (+10 more)

### Community 1165 - "terminal: .default()"
Cohesion: 0.22
Nodes (8): content_index_for_mouse(), convert_cells_to_content(), get_cells(), normalize_terminal_bounds(), TerminalBounds, test_mouse_to_cell_clamp(), test_mouse_to_cell_test(), test_write_raw_output_is_independent_of_chunk_boundaries()

### Community 1166 - "ui_macros: dynamic_spacing.rs"
Cohesion: 0.19
Nodes (7): derive_spacing(), DynamicSpacingInput, DynamicSpacingValue, Single, Tuple, derive_dynamic_spacing(), derive_register_component()

### Community 1167 - "util: util/src/markdown.rs"
Cohesion: 0.16
Nodes (10): count_max_consecutive_chars(), generate_heading_slug(), has_uri_scheme(), MarkdownCodeBlock, source_position_from_fragment(), split_local_url_fragment(), test_count_max_consecutive_chars(), test_generate_heading_slug() (+2 more)

### Community 1168 - ".conventions: .conventions/brand-writer/voic"
Cohesion: 0.11
Nodes (17): 10. Philosophy Statement, 1. Hype to Specifics, 2. Marketing to Technical, 3. Abstract to Concrete, 4. Em Dash Chains to Natural Flow, 5. Enthusiasm to Confidence, 6. "It's Not X, It's Y" Fix, 7. Vague Benefits to Specific Outcomes (+9 more)

### Community 1169 - "src: Configuration"
Cohesion: 0.11
Nodes (18): Configuration, Initializing the remote server, Known Limitations, Maintaining the SSH connection, Opening a folder already in WSL, Opening a local folder in WSL, Overview, Port forwarding (+10 more)

### Community 1170 - "skills: skills/brand-writer/voice-exam"
Cohesion: 0.11
Nodes (17): 10. Philosophy Statement, 1. Hype to Specifics, 2. Marketing to Technical, 3. Abstract to Concrete, 4. Em Dash Chains to Natural Flow, 5. Enthusiasm to Confidence, 6. "It's Not X, It's Y" Fix, 7. Vague Benefits to Specific Outcomes (+9 more)

### Community 1171 - "workflows: bump_collab_staging.yml"
Cohesion: 0.12
Nodes (17): jobs, update-collab-staging-tag, name, on, schedule, permissions, contents, cron (+9 more)

### Community 1172 - "workflows: defaults"
Cohesion: 0.12
Nodes (18): defaults, if, outputs, runs-on, timeout-minutes, defaults, run, jobs (+10 more)

### Community 1174 - "collab: ChannelMemberId"
Cohesion: 0.15
Nodes (7): Entity, Model, proto::ChannelMember, Relation, Channel, User, UserToChannel

### Community 1175 - "xtask: commitsfromversiontoversion"
Cohesion: 0.18
Nodes (9): check_compliance(), check_compliance_impl(), ComplianceArgs, ComplianceMode, Single, Version, IGNORE_LIST, MAX_CONCURRENT_REQUESTS (+1 more)

### Community 1176 - "compose.yml: compose.yml"
Cohesion: 0.12
Nodes (16): command, container_name, environment, image, ports, volumes, MINIO_ROOT_PASSWORD, MINIO_ROOT_USER (+8 more)

### Community 1177 - "feedback: copysystemspecsintoclipboard"
Cohesion: 0.18
Nodes (8): email_body(), email_zed_url(), file_bug_report_url(), format_installed_extensions_for_clipboard(), init(), REQUEST_FEATURE_URL, ZED_REPO_URL, SystemSpecs

### Community 1178 - "agent: prompt.md"
Cohesion: 0.12
Nodes (16): Async usage, AWS Bedrock, Contributing, Default Headers, Documentation, Google Vertex, Handling errors, License (+8 more)

### Community 1179 - "collab: location_project()"
Cohesion: 0.15
Nodes (15): location_project(), location_window(), LocationProject, First, Second, LocationWindow, First, Second (+7 more)

### Community 1181 - "edit_prediction_cli: test_tokenize()"
Cohesion: 0.20
Nodes (14): test_tokenize(), char_class(), compute_word_diff(), diff_tokens(), DiffOp, Delete, Equal, Insert (+6 more)

### Community 1182 - "editor: .modify_transaction_selection_"
Cohesion: 0.17
Nodes (9): SelectionHistory, SelectionHistoryEntry, SelectionHistoryMode, Normal, Redoing, Skipping, Undoing, SelectNextState (+1 more)

### Community 1184 - "extension: extension_events.rs"
Cohesion: 0.20
Nodes (9): Event, ConfigureExtensionRequested, ExtensionInstalled, ExtensionsInstalledChanged, ExtensionUninstalled, ExtensionEvents, GlobalExtensionEvents, init() (+1 more)

### Community 1185 - "feature_flags: feature_flags/src/store.rs"
Cohesion: 0.29
Nodes (11): enabled_for_all_wins_over_override(), enum_override_selects_specific_variant(), init_settings_store(), off_override_beats_server_flag(), on_flags_ready_waits_for_server_flags(), on_override_enables_without_server_or_staff(), presence_flag_defaults_to_off(), server_flag_enables_presence() (+3 more)

### Community 1186 - "text: .save()"
Cohesion: 0.15
Nodes (4): chunks_with_line_ending(), LineEnding, Unix, Windows

### Community 1187 - "gpui: .as_f32()"
Cohesion: 0.14
Nodes (4): bool, Pixels, Rems, SpringTarget

### Community 1188 - "gpui: spring.rs"
Cohesion: 0.18
Nodes (14): CRITICAL_DAMPING_TOLERANCE, DEFAULT_SPRING_EPSILON, duration_from_secs(), envelope_decay_start(), EPSILON, find_settle_time(), multiply(), propagators_compose_and_have_expected_determinant() (+6 more)

### Community 1190 - "json_schema_store: description"
Cohesion: 0.12
Nodes (17): description, items, type, packageExportsFallback, packageImportsFallback, $ref, description, items (+9 more)

### Community 1191 - "language_model_core: tool_schema.rs"
Cohesion: 0.24
Nodes (13): inline_refs(), inline_refs_fallibly(), inline_refs_recursive(), inlines_legacy_definitions_and_keeps_sibling_keywords(), inlines_the_references_context_servers_generate(), leaves_a_schema_untouched_when_a_reference_cannot_be_resolved(), normalization_is_idempotent(), normalization_leaves_non_object_schemas_unchanged_except_for_metadata() (+5 more)

### Community 1194 - "open_router: CacheControl"
Cohesion: 0.16
Nodes (12): CacheControl, CacheControlType, Ephemeral, CacheTtl, FiveMinutes, OneHour, MessageContent, Multipart (+4 more)

### Community 1195 - "openai_subscribed: fake_token_response()"
Cohesion: 0.21
Nodes (11): fake_token_response(), FakeCredentialsProvider, get_fresh_credentials(), make_expired_credentials(), make_state(), test_concurrent_refresh_deduplicates(), test_fatal_refresh_clears_auth_state(), test_fresh_credentials_skip_refresh() (+3 more)

### Community 1196 - "picker: Picker<D>"
Cohesion: 0.22
Nodes (5): Picker<D>, PickerAction, Entry, Header, Separator

### Community 1200 - "zed: mac_only_instance.rs"
Cohesion: 0.18
Nodes (12): address(), check_got_handshake(), CONNECT_TIMEOUT, ensure_only_instance(), instance_handshake(), IsOnlyInstance, No, Yes (+4 more)

### Community 1201 - "src: Approval Prompts {#approval-pr"
Cohesion: 0.12
Nodes (17): Approval Prompts {#approval-prompts}, Choosing What to Approve {#choosing-what-to-approve}, Default Access {#default-access}, Git Metadata {#git-metadata}, How big a risk is this in practice? {#how-big-a-risk}, How much can I trust the sandbox? {#trust}, Installing Bubblewrap {#installing-bubblewrap}, Linux {#linux} (+9 more)

### Community 1202 - "src: Actions"
Cohesion: 0.12
Nodes (17): Actions, Contexts, Disabling a binding, Forward keys to terminal, Key bindings, Keybinding Syntax, Keymap Editor, Linux (+9 more)

### Community 1203 - "src: bash.md"
Cohesion: 0.12
Nodes (13): Bash, Configuration, Install `shellcheck`:, See also:, Configuration, Env, See also:, Fish (+5 more)

### Community 1204 - "ISSUE_TEMPLATE: 10_bug_report.yml"
Cohesion: 0.12
Nodes (16): description, label, multiple, options, placeholder, value, body, attributes (+8 more)

### Community 1205 - "workflows: release_nightly.yml"
Cohesion: 0.12
Nodes (16): needs, runs-on, steps, timeout-minutes, env, CARGO_TERM_COLOR, RUST_BACKTRACE, jobs (+8 more)

### Community 1206 - "workflows: if"
Cohesion: 0.12
Nodes (17): if, runs-on, timeout-minutes, if, runs-on, timeout-minutes, if, runs-on (+9 more)

### Community 1207 - "compliance: CommitAuthor"
Cohesion: 0.16
Nodes (4): CommitAuthor, CommitMetadata, CommitSignature, GithubLogin

### Community 1208 - "skills: gpui-test/SKILL.md"
Cohesion: 0.12
Nodes (15): Attribute arguments, Common diagnosis patterns, Environment variables, General Rust test debugging vars often useful with GPUI tests, GPUI Test Debugging, GPUI test macro / scheduler execution, Lower-level scheduler tests, Non-determinism / wrong thread (+7 more)

### Community 1209 - "skills: zed-cherry-pick/SKILL.md"
Cohesion: 0.12
Nodes (15): 1. Gather context, 2. Reproduce the script's setup locally, 3. Check for missing prerequisite cherry-picks, 4. Resolve the conflicts manually, 5. Validate, 6. Finish the cherry-pick, 7. Push and open the PR, Final report to the user (+7 more)

### Community 1210 - "agent: Bindings"
Cohesion: 0.13
Nodes (8): Bindings, Config, Grammar, PathsJSON, Empty, Multiple, Single, TreeSitterJSON

### Community 1212 - "copilot_chat: ResponseFunctionOutput"
Cohesion: 0.13
Nodes (16): ResponseFunctionOutput, Content, Text, ResponseImageDetail, Auto, High, Low, ResponseInputContent (+8 more)

### Community 1214 - "dap: dap_types::EvaluateArgumentsCo"
Cohesion: 0.17
Nodes (5): dap_types::EvaluateArgumentsContext, dap_types::OutputEventGroup, dap_types::ScopePresentationHint, dap_types::Source, Vec<T>

### Community 1215 - "editor: code_completion_tests.rs"
Cohesion: 0.25
Nodes (13): filter_and_sort_matches(), filter_matches(), test_case_sensitive_match_tie_breaker(), test_for_each_prefix(), test_fuzzy_over_sort_positions(), test_fuzzy_score(), test_semver_label_sort_by_latest_version(), test_sort_exact() (+5 more)

### Community 1217 - "git_ui: commit_history_from_response()"
Cohesion: 0.17
Nodes (8): commit_history_from_response(), CommitHistory, Error, Loaded, Loading, CommitHistoryEntry, PanelRepoFooter, test_commit_history_from_response()

### Community 1218 - "gpui: A11y"
Cohesion: 0.16
Nodes (3): A11y, A11yActionListener, accessibility_enabled_is_independent_of_activation()

### Community 1219 - "gpui_tokio: gpui_tokio.rs"
Cohesion: 0.23
Nodes (4): GlobalTokio, init(), init_from_handle(), Tokio

### Community 1220 - "gpui_util: DebugAsDisplay"
Cohesion: 0.20
Nodes (6): DebugAsDisplay, log_err(), log_error_with_caller(), post_inc(), Result<T, E>, some_or_debug_panic()

### Community 1221 - "gpui_windows: .all_font_names()"
Cohesion: 0.17
Nodes (7): font_face_to_font(), font_style_from_dwrite(), font_style_to_dwrite(), get_font_names_from_collection(), get_name(), TextRenderer, TextRendererWrapper

### Community 1222 - "gpui_windows: vsync.rs"
Cohesion: 0.15
Nodes (6): DEFAULT_VSYNC_INTERVAL, get_dwm_interval(), QPC_TICKS_PER_SECOND, retrieve_duration(), VSYNC_INTERVAL_THRESHOLD, VSyncProvider

### Community 1223 - "project: context_server_store/registry."
Cohesion: 0.25
Nodes (3): ContextServerDescriptor, ContextServerDescriptorRegistry, GlobalContextServerDescriptorRegistry

### Community 1225 - "project: LabelPresence"
Cohesion: 0.25
Nodes (5): LabelPresence, KnownAbsent, Present, RootPathTrie<Label>, TriePath

### Community 1226 - "project: integration/trusted_worktrees."
Cohesion: 0.33
Nodes (14): init_test(), init_trust_global(), test_abs_path_trust_covers_multiple_worktrees(), test_auto_trust_all(), test_directory_worktree_trust_enables_single_file(), test_invisible_worktree_stores_do_not_affect_trust(), test_multi_host_trust_isolation(), test_multiple_single_file_worktrees_trust_one() (+6 more)

### Community 1229 - "settings_ui: icon_theme_picker()"
Cohesion: 0.32
Nodes (3): icon_theme_picker(), IconThemePicker, IconThemePickerDelegate

### Community 1231 - "settings_ui: App"
Cohesion: 0.32
Nodes (3): theme_picker(), ThemePicker, ThemePickerDelegate

### Community 1232 - "sqlez: Domain"
Cohesion: 0.17
Nodes (8): Domain, .MIGRATIONS, .NAME, changed_migration_fails(), Connection, migrations_dont_rerun(), test_create_alter_drop(), test_migrations_are_added_to_table()

### Community 1233 - "util: shell_env.rs"
Cohesion: 0.24
Nodes (10): capture(), capture_output_redirection(), capture_powershell_environment_without_startup_output(), capture_unix(), capture_windows(), exit_status(), parse_env_map_from_noisy_output(), parse_env_output() (+2 more)

### Community 1234 - "worktree: IgnoreKind"
Cohesion: 0.14
Nodes (10): IgnoreKind, Gitignore, RepoExclude, IgnoreStack, IgnoreStackEntry, All, Global, None (+2 more)

### Community 1235 - ".conventions: .conventions/brand-writer/tabo"
Cohesion: 0.12
Nodes (15): AI Structural Patterns, Colon-Introduced Lists in Prose, Corporate Euphemisms, Detection Checklist, Em Dash Chains, Empty Enthusiasm, Filler Phrases, Forbidden Punctuation (+7 more)

### Community 1236 - "README.md: docs/README.md"
Cohesion: 0.12
Nodes (15): Actions, Consent Banner, Creating New Templates, Images and videos, Implementation details, Internal notes:, Keybindings, Keymap Overlays (+7 more)

### Community 1237 - "src: Amp Notifications {#amp-notifi"
Cohesion: 0.12
Nodes (16): Amp Notifications {#amp-notifications}, Claude Code Notifications {#claude-code-notifications}, CLI/TUI Setup Notes {#cli-setup}, Closing Terminal Threads {#closing-terminal-threads}, Codex Terminal Titles {#codex-terminal-titles}, Credentials and Remote Projects {#credentials-and-remote-projects}, Notifications {#terminal-thread-notifications}, OpenCode Notifications {#opencode-notifications} (+8 more)

### Community 1238 - "src: Binding runnable tags to task"
Cohesion: 0.12
Nodes (16): Binding runnable tags to task templates, Custom Git Commands, Custom keybindings for tasks, Ephemeral tasks, Hooks, Keybindings to run tasks bound to runnables, More task rerun control, Oneshot tasks (+8 more)

### Community 1239 - "skills: skills/brand-writer/taboo-phra"
Cohesion: 0.12
Nodes (15): AI Structural Patterns, Colon-Introduced Lists in Prose, Corporate Euphemisms, Detection Checklist, Em Dash Chains, Empty Enthusiasm, Filler Phrases, Forbidden Punctuation (+7 more)

### Community 1240 - "workflows: with"
Cohesion: 0.12
Nodes (16): with, app-id, cache, clean, if-no-files-found, name, owner, path (+8 more)

### Community 1241 - "workflows: run_bundling.yml"
Cohesion: 0.12
Nodes (15): concurrency, cancel-in-progress, group, defaults, run, env, CARGO_TERM_COLOR, RUST_BACKTRACE (+7 more)

### Community 1242 - "workflows: triage_queue_board.yml"
Cohesion: 0.12
Nodes (15): concurrency, cancel-in-progress, group, default, description, type, dry_run, jobs (+7 more)

### Community 1243 - "lints: LocalDefId"
Cohesion: 0.24
Nodes (7): BLOCKING_FN_PATHS, BLOCKING_METHODS, BlockingIoOnForeground, is_blocking_call(), is_gpui_foreground_type(), is_in_foreground_fn(), owner_has_foreground_param()

### Community 1244 - "danger: danger"
Cohesion: 0.16
Nodes (15): danger, danger-plugin-pr-hygiene, @types/node, typescript, devDependencies, danger@13.0.8, danger-plugin-pr-hygiene@0.7.1, @types/node@24.13.4 (+7 more)

### Community 1245 - "PROMPTS.md: PROMPTS.md"
Cohesion: 0.12
Nodes (15): P01 — First build, P02 — GitHub repo, P03 — Rebrand, P04 — Cut ties to zed.dev services, P05 — Remove Zed account UI, P06 — VEGA Dark theme, P07 — AI onboarding, P08 — AI defaults (+7 more)

### Community 1246 - "project: color_extractor"
Cohesion: 0.19
Nodes (9): extract_color(), can_extract_from_detail(), can_extract_from_documentation_end(), can_extract_from_documentation_start(), can_extract_from_label(), cannot_extract_from_documentation_middle(), COLOR_TABLE, only_whole_detail_matches_are_allowed() (+1 more)

### Community 1247 - "multi_buffer: .expand_excerpt()"
Cohesion: 0.14
Nodes (5): ExpandExcerptDirection, Down, Up, UpAndDown, ExpandInfo

### Community 1248 - "agent: from_pixels_constructor/before"
Cohesion: 0.18
Nodes (6): Blit, BlitA8ToRgb24, BlitMemcpy, BlitRgb24ToA8, BlitRgb24ToRgba32, BlitRgba32ToRgb24

### Community 1249 - "agent: translate_doc_comments/before."
Cohesion: 0.18
Nodes (6): Blit, BlitA8ToRgb24, BlitMemcpy, BlitRgb24ToA8, BlitRgb24ToRgba32, BlitRgba32ToRgb24

### Community 1250 - "agent: FindPathTool"
Cohesion: 0.18
Nodes (5): FindPathTool, .NAME, init_test(), search_paths(), test_find_path_tool()

### Community 1251 - "agent_ui: AgentNotification"
Cohesion: 0.20
Nodes (4): AgentNotification, AgentNotificationEvent, Accepted, Dismissed

### Community 1252 - "client: zed_urls.rs"
Cohesion: 0.34
Nodes (12): account_url(), acp_registry_blog(), ai_privacy_and_security(), docs_url(), edit_prediction_docs(), llm_provider_docs(), sandboxing_docs(), server_url() (+4 more)

### Community 1256 - "editor: LinkedEditingRanges"
Cohesion: 0.19
Nodes (3): LinkedEditingRanges, LinkedEdits, refresh_linked_ranges()

### Community 1257 - "feature_flags: FeatureFlagValue"
Cohesion: 0.14
Nodes (4): FeatureFlagValue, PresenceFlag, Off, On

### Community 1259 - "file_icons: FileIcons"
Cohesion: 0.30
Nodes (5): FileIcons, test_folder_indicator_default_is_icon(), test_folder_indicators_per_setting(), test_folder_indicators_reflect_expanded_state(), test_suffix_matches_take_precedence_over_prefixes()

### Community 1260 - "git_ui: git_panel_settings.rs"
Cohesion: 0.20
Nodes (3): GitPanelScrollbarAccessor, GitPanelSettings, ScrollbarSettings

### Community 1261 - "gpui: gpui/src/test.rs"
Cohesion: 0.18
Nodes (5): apply_seed_to_proptest_config(), Observation, observe(), run_test(), seed_strategy()

### Community 1262 - "gpui: .with_creator()"
Cohesion: 0.20
Nodes (6): A11yDebug, ephemeral_id(), node_to_json(), NodeCreator, NodeDebugInfo, NodeProvenance

### Community 1263 - "gpui_windows: apply_font_features()"
Cohesion: 0.25
Nodes (4): apply_font_features(), DirectWriteState, FontInfo, is_color_glyph()

### Community 1264 - "json_schema_store: additionalProperties"
Cohesion: 0.14
Nodes (15): additionalProperties, description, patternProperties, properties, peerDependencyMeta, description, type, additionalProperties (+7 more)

### Community 1265 - "migrator: m_2026_02_04/settings.rs"
Cohesion: 0.21
Nodes (12): AGENT_KEY, ALWAYS_ALLOW_TOOL_ACTIONS, DEFAULT_KEY, DEFAULT_MODE_KEY, migrate_agent_tool_permissions(), migrate_agent_with_profiles(), migrate_default_mode_to_default(), migrate_one() (+4 more)

### Community 1267 - "project: integration/signature_help.rs"
Cohesion: 0.22
Nodes (12): current_parameter(), test_create_signature_help_implements_utf16_spec(), test_create_signature_help_markdown_string_1(), test_create_signature_help_markdown_string_2(), test_create_signature_help_markdown_string_3(), test_create_signature_help_markdown_string_4(), test_create_signature_help_markdown_string_5(), test_create_signature_help_markdown_string_6() (+4 more)

### Community 1268 - "recent_projects: recent_projects/src/ssh_config"
Cohesion: 0.23
Nodes (12): does_not_fuzzy_match_host_aliases(), falls_back_to_host_when_hostname_is_absent(), FILTERED_GIT_PROVIDER_HOSTNAMES, filters_git_provider_domains_from_hostname(), HostBlock, is_git_provider_domain(), parse_host_blocks(), parse_hosts() (+4 more)

### Community 1269 - "refineable: Cascade"
Cohesion: 0.23
Nodes (4): Cascade, Cascade<S>, CascadeSlot, Refineable

### Community 1270 - "workspace: GlobalHistoryManager"
Cohesion: 0.27
Nodes (4): GlobalHistoryManager, HistoryManager, HistoryManagerEntry, init()

### Community 1271 - "src: Code formatting"
Cohesion: 0.13
Nodes (15): Code formatting, Configure ESLint's `nodePath`:, Configure ESLint's `problems`:, Configure ESLint's `rulesCustomizations`:, Configure ESLint's `workingDirectory`:, Configuring JavaScript debug tasks, Debug the current file with Node, Debugging (+7 more)

### Community 1272 - "src: Auto Fold Dirs"
Cohesion: 0.13
Nodes (15): Auto Fold Dirs, Auto Open, Auto Reveal Entries, Bold Folder Labels, Default Width, Dock, Entry Spacing, Git Status (+7 more)

### Community 1273 - "src: Colors not updating after chan"
Cohesion: 0.13
Nodes (15): Colors not updating after changing settings, Customizing Token Colors, Default Rules, Enabling Semantic Tokens, Example: Disabling a Token Type, Example: Highlighting Unresolved References, Example: Highlighting Unsafe Code, Example: Using Theme Styles (+7 more)

### Community 1274 - "theme: plugins.js"
Cohesion: 0.21
Nodes (12): copyPageActions(), darkModeToggle(), persistTheme(), setTheme(), detectOS(), getSectionItems(), initCollapsibleSidebar(), initializePlugins() (+4 more)

### Community 1275 - "renovate.json: renovate.json"
Cohesion: 0.13
Nodes (14): dependencyDashboard, extends, ignorePaths, packageRules, prFooter, schedule, $schema, timezone (+6 more)

### Community 1276 - "workflows: steps"
Cohesion: 0.24
Nodes (15): steps, steps, steps, BUMP_TYPE, COMMITTER_TOKEN, CURRENT_VERSION, OLD_VERSION, PR_NUMBER (+7 more)

### Community 1277 - "workflows: default"
Cohesion: 0.13
Nodes (15): default, description, type, description, required, type, bump-type, force-bump (+7 more)

### Community 1278 - "workflows: extension_workflow_rollout.yml"
Cohesion: 0.13
Nodes (14): if, needs, runs-on, timeout-minutes, defaults, run, env, CARGO_TERM_COLOR (+6 more)

### Community 1279 - "workflows: with"
Cohesion: 0.13
Nodes (15): with, authToken, cache, cachixArgs, check-latest, clean, fetch-tags, github_access_token (+7 more)

### Community 1280 - "workflows: env"
Cohesion: 0.27
Nodes (15): env, env, env, env, APPLE_NOTARIZATION_ISSUER_ID, APPLE_NOTARIZATION_KEY, APPLE_NOTARIZATION_KEY_ID, CARGO_INCREMENTAL (+7 more)

### Community 1281 - "settings: inlay_hints"
Cohesion: 0.14
Nodes (14): inlay_hints, edit_debounce_ms, enabled, scroll_debounce_ms, show_background, show_other_hints, show_parameter_hints, show_type_hints (+6 more)

### Community 1282 - "acp_thread: subagents.rs"
Cohesion: 0.25
Nodes (8): absolute_path_from_v1(), association_fields_follow_patch_semantics(), directories_follow_patch_semantics_and_inherit_independently(), state_from_v1(), update_from_v1(), v1_directories_must_be_absolute(), v1_updates_convert_fields_and_state_snapshots(), v2_update()

### Community 1284 - "collab: join_channel()"
Cohesion: 0.16
Nodes (13): join_channel(), pane_summaries(), PaneSummary, participant_location(), share_workspace(), test_active_multi_workspace_determines_participant_location(), test_following_across_workspaces(), test_following_after_replacement() (+5 more)

### Community 1285 - "collab_ui: ChannelEditingState"
Cohesion: 0.16
Nodes (6): ChannelEditingState, Create, Rename, CollabNotificationToast, DraggedChannelView, JoinChannelTooltip

### Community 1286 - "context_server: .http()"
Cohesion: 0.19
Nodes (4): ContextServerId, ContextServerTransport, Custom, Stdio

### Community 1287 - "copilot_chat: ChatMessage"
Cohesion: 0.19
Nodes (12): ChatMessage, Assistant, System, Tool, User, ChatMessageContent, Multipart, Plain (+4 more)

### Community 1288 - "deepseek: src/deepseek.rs"
Cohesion: 0.25
Nodes (11): Choice, DEEPSEEK_API_URL, FunctionChunk, FunctionDefinition, Response, stream_completion(), StreamChoice, StreamDelta (+3 more)

### Community 1289 - "deployment_ui: init()"
Cohesion: 0.29
Nodes (8): init(), open_config(), register_actions(), run_on_selection(), selected_path(), show_message(), show_request(), show_requests()

### Community 1290 - "gpui: inhibit_idle_sleep()"
Cohesion: 0.19
Nodes (3): inhibit_idle_sleep(), release_on_drop(), ActivityGuard

### Community 1291 - "gpui: animation_phases_interpolate_o"
Cohesion: 0.22
Nodes (3): animation_phases_interpolate_over_arbitrary_ranges(), AnimationPhase, SpringAnimation

### Community 1293 - "inspector_ui: inspector_ui/README.md"
Cohesion: 0.14
Nodes (13): Cleaner lifecycle for inspector state viewers / editors, Code cleanups, Consider removing special side pane rendering, Current features, Edit original code in Rust style editor, Future features, Inspector, JSON style editor undo history doesn't get reset (+5 more)

### Community 1294 - "languages: cpp.rs"
Cohesion: 0.15
Nodes (10): semantic_token_rules(), test_cpp_autoindent_access_specifier(), test_cpp_autoindent_access_specifier_next_line(), test_cpp_autoindent_access_specifier_with_method_bodies(), test_cpp_autoindent_basic(), test_cpp_autoindent_consecutive_access_specifiers(), test_cpp_autoindent_if_else(), test_cpp_autoindent_indented_access_specifiers() (+2 more)

### Community 1295 - "livekit_client: remote_video_track_view.rs"
Cohesion: 0.30
Nodes (3): RemoteVideoTrackView, RemoteVideoTrackViewEvent, Close

### Community 1296 - "lsp_command_selector: init_selector_test()"
Cohesion: 0.25
Nodes (13): init_selector_test(), init_selector_test_with_capabilities(), open_selector(), selector_is_open(), test_action_is_not_registered_for_unrelated_buffers(), test_action_is_not_registered_without_commands(), test_command_error_is_shown_in_the_picker_footer(), test_commands_are_scoped_to_the_servers_of_the_buffer_worktree() (+5 more)

### Community 1297 - "prettier: Prettier"
Cohesion: 0.19
Nodes (6): Prettier, .CONFIG_FILE_NAMES, Real, Test, RealPrettier, TestPrettier

### Community 1298 - "repl: encode_test_image()"
Cohesion: 0.22
Nodes (4): encode_test_image(), ImageView, test_image_view_scaled_size_respects_limits(), test_image_view_scaled_size_unbounded()

### Community 1299 - "sqlez_macros: sqlez_macros.rs"
Cohesion: 0.30
Nodes (7): close_delimiter(), create_error(), flatten_stream(), make_sql(), open_delimiter(), sql(), SQLITE

### Community 1300 - "ui: .render()"
Cohesion: 0.16
Nodes (7): KnockoutIconName, DotBg, DotFg, TriangleBg, TriangleFg, XBg, XFg

### Community 1301 - "workspace: Event"
Cohesion: 0.14
Nodes (14): Event, Activate, ActiveItemChanged, ContactRequestedJoin, ItemAdded, ItemRemoved, ModalOpened, OpenBundledFile (+6 more)

### Community 1302 - "AGENTS.md: docs/AGENTS.md"
Cohesion: 0.14
Nodes (13): Content Structure, Core Principles, Feature Documentation, Migration Guides, Page Organization, Pull request hygiene, Section Patterns, Settings Documentation (+5 more)

### Community 1303 - "src: AI by Company"
Cohesion: 0.14
Nodes (14): AI by Company, Anthropic / Claude / Claude Code {#anthropic-claude}, Cursor {#cursor}, DeepSeek {#deepseek}, Gateways and Cloud Platforms {#gateways}, GitHub / Copilot {#github-copilot}, Google / Gemini / Gemini CLI {#google-gemini}, Local Models {#local-models} (+6 more)

### Community 1304 - "src: Amazon Bedrock {#amazon-bedroc"
Cohesion: 0.14
Nodes (14): Amazon Bedrock {#amazon-bedrock}, Bedrock Authentication {#bedrock-authentication}, Bedrock Cross-Region Inference {#bedrock-cross-region-inference}, Bedrock Custom Endpoints {#bedrock-custom-endpoints}, Bedrock Guardrails {#bedrock-guardrails}, Bedrock Mantle Models {#bedrock-mantle-models}, Custom Bedrock Mantle Models {#bedrock-mantle-custom-models}, Custom Bedrock Models {#bedrock-custom-models} (+6 more)

### Community 1305 - "src: Build fails: Path too long"
Cohesion: 0.14
Nodes (14): Build fails: Path too long, Building from source, Building Zed for Windows, Cargo errors claiming that a dependency is using unstable features, Dependencies, Graphics issues, Installing from msys2, Invalid RC path selected (+6 more)

### Community 1306 - "src: Alternate Targets"
Cohesion: 0.14
Nodes (14): Alternate Targets, Automatically locate a debug target based on build command, Binary, Build binary then debug, Debugging, Inlay Hints, Large projects and performance, LSP tasks (+6 more)

### Community 1307 - "ISSUE_TEMPLATE: 11_crash_report.yml"
Cohesion: 0.14
Nodes (13): description, label, placeholder, value, body, attributes, type, validations (+5 more)

### Community 1308 - "workflows: deploy_collab.yml"
Cohesion: 0.14
Nodes (13): env, DOCKER_BUILDKIT, jobs, publish, name, on, push, permissions (+5 more)

### Community 1309 - "terms.md: 16.1."
Cohesion: 0.14
Nodes (14): 16.1., 16.10. Additional Terms, 16.11. Consent to Electronic Communications, 16.12. Contact Information, 16.13. Notice to California Residents, 16.2. Notices, 16.3. DPA, 16.4. Modification of Terms (+6 more)

### Community 1310 - "cargo-timing-info.js: ref_fs"
Cohesion: 0.21
Nodes (11): analyzeTimings(), args, extractUnitData(), formatTime(), formatUnit(), fs, getZedDataDir(), os (+3 more)

### Community 1311 - "lints: consumer/src/lib.rs"
Cohesion: 0.27
Nodes (12): from_long(), from_short(), from_variable(), into_long(), into_short(), into_variable(), new_short(), new_static_short() (+4 more)

### Community 1312 - "settings: title_bar"
Cohesion: 0.15
Nodes (13): title_bar, button_layout, open_menus_on_hover, show, show_branch_name, show_branch_status_icon, show_menus, show_onboarding_banner (+5 more)

### Community 1314 - "client: client/src/proxy.rs"
Cohesion: 0.21
Nodes (4): AsyncReadWrite, connect_proxy_stream(), connect_tls_to_proxy(), excluded_from_proxy()

### Community 1315 - "editor: ToggleBlockComments"
Cohesion: 0.28
Nodes (12): ToggleBlockComments, setup_rust_context(), test_toggle_block_comments(), test_toggle_block_comments_cursor_inside(), test_toggle_block_comments_cursor_inside_unicode_comment(), test_toggle_block_comments_empty_selection_roundtrip(), test_toggle_block_comments_multiline(), test_toggle_block_comments_multiple_cursors() (+4 more)

### Community 1317 - "settings_ui: FeatureFlagDescriptor"
Cohesion: 0.24
Nodes (5): FeatureFlagDescriptor, FeatureFlagVariant, render_feature_flags_page(), render_flag_row(), render_flag_variants()

### Community 1318 - "fs: git_clone_progress.rs"
Cohesion: 0.31
Nodes (6): extracts_the_last_failure_line(), failure_message(), forwards_unrecognized_git_output(), GitCloneProgress, parses_fragmented_progress_and_suppresses_duplicates(), read()

### Community 1319 - "gpui: global.rs"
Cohesion: 0.22
Nodes (3): ReadGlobal, T, UpdateGlobal

### Community 1320 - "gpui_windows: A11yActionHandler"
Cohesion: 0.17
Nodes (3): A11yActionHandler, A11yActivationHandler, A11yState

### Community 1321 - "json_schema_store: devEngineDependency"
Cohesion: 0.15
Nodes (13): devEngineDependency, description, properties, required, type, description, enum, type (+5 more)

### Community 1322 - "languages: init()"
Cohesion: 0.22
Nodes (5): init(), LanguageInfo, load_config(), register_language(), test_env_files_are_not_shell_scripts()

### Community 1323 - "migrator: m_2026_09_16/settings.rs"
Cohesion: 0.27
Nodes (10): KEY_MAPPINGS, migrate(), migrate_one(), nest_agent_threads_sidebar_settings(), nest_agent_threads_sidebar_settings_handles_null_destinations_only_when_needed(), nest_agent_threads_sidebar_settings_leaves_unrelated_and_malformed_values_untouched(), nest_agent_threads_sidebar_settings_migrates_all_supported_scopes(), nest_agent_threads_sidebar_settings_moves_keys_and_preserves_unrelated_fields() (+2 more)

### Community 1325 - "project: concurrent_remote_askpass_requ"
Cohesion: 0.35
Nodes (10): concurrent_remote_askpass_request_is_rejected(), ending_remote_operation_cancels_active_askpass(), late_remote_askpass_response_does_not_restore_operation(), remote_askpass_is_rejected_after_operation_ends(), RemoteAskPassDelegates, RemoteAskPassOperation, request_remote_password(), successful_remote_askpass_allows_another_prompt() (+2 more)

### Community 1326 - "util_macros: util_macros.rs"
Cohesion: 0.21
Nodes (4): line_endings(), perf(), PerfArgs, uri()

### Community 1327 - "util: schemars.rs"
Cohesion: 0.22
Nodes (5): add_new_subschema(), AllowTrailingCommas, DefaultDenyUnknownFields, DEFS_PATH, replace_subschema()

### Community 1328 - "zlog: env_config.rs"
Cohesion: 0.27
Nodes (10): directive_level(), EnvFilter, err_when_invalid_level(), err_when_multiple_max_levels(), global_level(), global_level_and_bare_module(), global_level_and_directive_level(), parse() (+2 more)

### Community 1329 - "lints: defkind"
Cohesion: 0.33
Nodes (5): is_owned_string_built_from_literal(), is_refcounted_string_destination(), is_std_string(), is_string_literal(), OwnedStringIntoShared

### Community 1330 - "src: AI usage {#ai-usage}"
Cohesion: 0.15
Nodes (13): AI usage {#ai-usage}, Billing, Billing cycles {#billing-cycles}, Billing dashboard {#dashboard}, Billing information {#settings}, Individual billing {#individual}, Invoice history {#invoice-history}, Invoice history {#org-invoice-history} (+5 more)

### Community 1331 - "src: Call Hierarchy"
Cohesion: 0.15
Nodes (13): Call Hierarchy, Command Palette, File Finder, Finding & Navigating, Go to Definition, Go to Symbol, Outline Panel, Project Bookmarks {#project-bookmarks} (+5 more)

### Community 1332 - "src: Attaching to an existing insta"
Cohesion: 0.15
Nodes (13): Attaching to an existing instance of Delve, Build and debug separately, Code Lens, Debug Go Packages, Debug Go Tests, Debugging, Go, Go Mod (+5 more)

### Community 1333 - "src: Building from Source"
Cohesion: 0.15
Nodes (13): Building from Source, Can't install CLI {#cant-install-cli}, CLI command not found, GPU or rendering issues, High memory or CPU usage, Homebrew, Installing the CLI, Installing Zed (+5 more)

### Community 1334 - "workflows: bump_zed_version.yml"
Cohesion: 0.15
Nodes (12): defaults, run, target, name, on, workflow_dispatch, permissions, shell (+4 more)

### Community 1335 - "workflows: default"
Cohesion: 0.18
Nodes (13): default, description, type, default, description, type, channel, checkout_ref (+5 more)

### Community 1336 - "workflows: description"
Cohesion: 0.15
Nodes (13): description, required, description, required, description, required, description, required (+5 more)

### Community 1337 - "workflows: with"
Cohesion: 0.15
Nodes (13): with, accountId, apiToken, args, cache, clean, command, fail (+5 more)

### Community 1338 - "workflows: extension_bump.yml"
Cohesion: 0.15
Nodes (12): concurrency, cancel-in-progress, group, defaults, env, CARGO_INCREMENTAL, CARGO_TERM_COLOR, RUST_BACKTRACE (+4 more)

### Community 1339 - "workflows: ACTIONLINT_BIN"
Cohesion: 0.15
Nodes (13): ACTIONLINT_BIN, ARTIFACT_URL, COMPLIANCE_OUTCOME, COMPLIANCE_TAG, GITHUB_APP_ID, GITHUB_APP_KEY, GITHUB_TOKEN, R2_ACCESS_KEY_ID (+5 more)

### Community 1340 - "crates_io_client: CratesIoClient"
Cohesion: 0.27
Nodes (3): CratesIoClient, Result, CrateUpdate

### Community 1341 - "DEPLOYMENT.md: DEPLOYMENT.md"
Cohesion: 0.15
Nodes (12): Architecture, Decisions (owner, 2026-11-18), Deployment & Remote Host — plan, Milestones, Naming, Next session, Risks, Running the server tests (+4 more)

### Community 1342 - "settings: diagnostics"
Cohesion: 0.17
Nodes (12): diagnostics, button, include_warnings, inline, lsp_pull_diagnostics, enabled, max_severity, min_column (+4 more)

### Community 1343 - "action_log: .keep_all_edits()"
Cohesion: 0.39
Nodes (4): ActionLogMetrics, ActionLogTelemetry, telemetry_report_accepted_edits(), telemetry_report_rejected_edits()

### Community 1344 - "workflows: evals.rs"
Cohesion: 0.18
Nodes (7): run_gpui_eval(), if, outputs, runs-on, timeout-minutes, compliance_check, outcome

### Community 1345 - "agent: .deref()"
Cohesion: 0.18
Nodes (3): InvalidationRegion, InvalidationStack<T>, SnippetState

### Community 1347 - "dap: dap_types::Module"
Cohesion: 0.17
Nodes (4): dap_types::Module, dap_types::OutputEvent, dap_types::Scope, dap_types::Thread

### Community 1348 - "editor: Direction"
Cohesion: 0.21
Nodes (5): Direction, Next, Prev, ChangeList, ChangeLocation

### Community 1349 - "editor: GutterButtonIntent"
Cohesion: 0.24
Nodes (4): GutterButtonIntent, SetBookmark, SetBreakpoint, GutterButtonTooltip

### Community 1353 - "json_schema_store: schemas/package.json"
Cohesion: 0.17
Nodes (11): description@DEPRECATED: This field is honored, bu..., oneOf, description@Array of package names that will be b..., oneOf, patternProperties, bundledDependencies, bundleDependencies, $schema (+3 more)

### Community 1354 - "json_schema_store: anyOf"
Cohesion: 0.15
Nodes (12): anyOf, description, markdownDescription, type, x-intellij-html-description, default, description, markdownDescription (+4 more)

### Community 1355 - "markdown: path_range.rs"
Cohesion: 0.26
Nodes (7): LineCol, PathWithRange, test_linecol_parsing(), test_pathrange_from_str(), test_pathrange_leading_text_trimming(), test_pathrange_parsing(), test_pathrange_with_line_and_column()

### Community 1356 - "migrator: m_2026_04_10/settings.rs"
Cohesion: 0.24
Nodes (9): AGENT_KEY, migrate_agent_value(), NEW_TOOL_NAME, OLD_TOOL_NAME, PROFILES_KEY, rename_key(), rename_web_search_to_search_web(), TOOL_PERMISSIONS_KEY (+1 more)

### Community 1357 - "openai_subscribed: Context"
Cohesion: 0.23
Nodes (7): SignInState, Authorizing, Idle, PersistingCredentials, test_cancel_sign_in_drops_pending_task(), test_sign_in_task_remains_alive_while_persisting_credentials(), wait_for_model_catalog()

### Community 1358 - "tabular_data_preview: LineNumber"
Cohesion: 0.21
Nodes (5): LineNumber, RowIdentDisplayMode, Horizontal, Vertical, TableView

### Community 1359 - "telemetry_events: AssistantKind"
Cohesion: 0.17
Nodes (9): AssistantKind, Inline, InlineTerminal, Panel, AssistantPhase, Accepted, Invoked, Rejected (+1 more)

### Community 1360 - "telemetry: Event"
Cohesion: 0.23
Nodes (7): Event, Flexible, init(), event!, serialize_property!, send_event(), TELEMETRY_QUEUE

### Community 1361 - ".conventions: .conventions/brand-writer/rubr"
Cohesion: 0.17
Nodes (11): 1. Technical Grounding (1-5), 2. Natural Syntax (1-5), 3. Quiet Confidence (1-5), 4. Developer Respect (1-5), 5. Information Priority (1-5), 6. Specificity (1-5), 7. Voice Consistency (1-5), 8. Earned Claims (1-5) (+3 more)

### Community 1362 - ".doc-examples: configuration.md"
Cohesion: 0.17
Nodes (11): Configuring Zed, Default Settings {#default-settings}, Example Configuration {#example-configuration}, How Settings Merge {#how-settings-merge}, Per-Release Channel Overrides {#release-channel-overrides}, Project Settings {#project-settings}, See Also {#see-also}, Settings Deep Links {#deep-links} (+3 more)

### Community 1363 - "src: Agent Settings"
Cohesion: 0.17
Nodes (12): Agent Settings, Automatic Compaction {#automatic-compaction}, Compaction Model {#compaction-model}, External Agents {#external-agents}, Feature-Specific Models {#feature-specific-models}, Feature-Specific Settings {#feature-specific-settings}, LLM Providers {#llm-providers}, MCP Servers {#mcp-servers} (+4 more)

### Community 1364 - "src: Agent Panel Usage"
Cohesion: 0.17
Nodes (12): Agent Panel Usage, Agent Path Support {#agent-path-support}, As Custom Servers, As Extensions, Configuration Check, Error Handling, External Agents, Installing MCP Servers (+4 more)

### Community 1365 - "src: Channel Notes"
Cohesion: 0.17
Nodes (12): Channel Notes, Channels {#channels}, Following a Terminal, Following Collaborators, How Following Works, Inviting People, Leaving a Call, Livestreaming & Guests (+4 more)

### Community 1366 - "src: Configuring Zed"
Cohesion: 0.17
Nodes (12): Configuring Zed, Default Settings, Example Configuration, How Settings Merge, Per-file Settings, Per-Release Channel Overrides, Project Settings, Settings Deep Links (+4 more)

### Community 1367 - "src: Arguments"
Cohesion: 0.17
Nodes (12): Arguments, Binary, Build and Debug Binary, C++, CMake Compile Commands, Compile Commands, Debugging, Formatting (+4 more)

### Community 1368 - "src: Debugging"
Cohesion: 0.17
Nodes (12): Debugging, Elixir, Formatting without a language server, Language Servers, See also, Using Dexter, Using ElixirLS, Using ElixirLS (+4 more)

### Community 1369 - "skills: skills/brand-writer/rubric.md"
Cohesion: 0.17
Nodes (11): 1. Technical Grounding (1-5), 2. Natural Syntax (1-5), 3. Quiet Confidence (1-5), 4. Developer Respect (1-5), 5. Information Priority (1-5), 6. Specificity (1-5), 7. Voice Consistency (1-5), 8. Earned Claims (1-5) (+3 more)

### Community 1370 - "workflows: jobs"
Cohesion: 0.17
Nodes (12): jobs, publish_job, update_sha_in_extensions, update_sha_in_zed, if, runs-on, if, needs (+4 more)

### Community 1371 - "workflows: env"
Cohesion: 0.23
Nodes (12): env, env, needs, runs-on, steps, timeout-minutes, APPLE_NOTARIZATION_ISSUER_ID, APPLE_NOTARIZATION_KEY (+4 more)

### Community 1372 - "workflows: needs"
Cohesion: 0.18
Nodes (12): needs, runs-on, steps, timeout-minutes, needs, runs-on, steps, timeout-minutes (+4 more)

### Community 1373 - "workflows: POSTGRES_HOST_AUTH_METHOD"
Cohesion: 0.17
Nodes (12): POSTGRES_HOST_AUTH_METHOD, run_tests_linux, env, image, options, ports, if, needs (+4 more)

### Community 1374 - "workflows: POSTGRES_HOST_AUTH_METHOD"
Cohesion: 0.17
Nodes (12): POSTGRES_HOST_AUTH_METHOD, run_tests_linux, env, image, options, ports, if, needs (+4 more)

### Community 1375 - "workflows: with"
Cohesion: 0.17
Nodes (12): with, cache, check-latest, clean, config, egress-policy, fetch-depth, node-version (+4 more)

### Community 1377 - "livekit.yaml: livekit.yaml"
Cohesion: 0.18
Nodes (10): development, keys, devkey, logging, level, port, rtc, tcp_port (+2 more)

### Community 1378 - "agent_skills: create-skill/SKILL.md"
Cohesion: 0.18
Nodes (10): Creating a Zed Agent Skill, Naming Rules, Optional Frontmatter Fields, Required Frontmatter Fields, SKILL.md Format, Step-by-Step: Creating a Skill, Supporting Files, What is a Skill? (+2 more)

### Community 1379 - "agent: Range<DisplayRow>"
Cohesion: 0.22
Nodes (3): Range<DisplayRow>, Range<MultiBufferRow>, RowRangeExt

### Community 1380 - "agent: Accessing raw response data (e"
Cohesion: 0.18
Nodes (11): Accessing raw response data (e.g. headers), Advanced, Configuring the HTTP client, How to tell whether `None` means `null` or missing, Logging, Making custom/undocumented requests, Managing HTTP resources, Undocumented endpoints (+3 more)

### Community 1385 - "deepseek: Model"
Cohesion: 0.18
Nodes (4): Model, Custom, V4_1Flash, V4Pro

### Community 1386 - "extension_suggest: extension_suggest.rs"
Cohesion: 0.27
Nodes (8): additional_suggestion_for_language(), LanguageAdditionSuggestion, suggest_extension(), suggested_extensions(), SuggestedExtension, SUGGESTIONS_BY_EXTENSION_ID, SUGGESTIONS_BY_LANGUAGE, test_suggested_extension()

### Community 1387 - "fuzzy_nucleo: fuzzy_nucleo/src/matcher.rs"
Cohesion: 0.33
Nodes (7): get_matcher(), get_matchers(), LENGTH_PENALTY, MATCHERS, pool_cap(), return_matcher(), return_matchers()

### Community 1388 - "git_ui_core: worktree_names.rs"
Cohesion: 0.31
Nodes (6): ADJECTIVES, generate_worktree_name(), NOUNS, test_generate_worktree_name_avoids_existing(), test_generate_worktree_name_format(), test_generate_worktree_name_returns_none_when_stuck()

### Community 1389 - "gpui: examples/README.md"
Cohesion: 0.18
Nodes (10): Canvas font fallback, Dynamic font loading, GPUI Examples, Images, drawing, and animation, Interaction, Layout and styling, Running in a browser, Specialized examples (+2 more)

### Community 1390 - "gpui: SpringElementState"
Cohesion: 0.42
Nodes (5): SpringElementState, sampled_easing(), settle_time_is_conservative_for_every_damping_regime(), SpringConfig, SpringState

### Community 1391 - "gpui: f32"
Cohesion: 0.22
Nodes (5): f32, Hsla, hsla_interpolation_takes_the_shortest_hue_path(), Interpolate, Rgba

### Community 1393 - "language_model_core: CompletionIntent"
Cohesion: 0.18
Nodes (11): CompletionIntent, CreateFile, EditFile, GenerateGitCommitMessage, InlineAssist, Subagent, TerminalInlineAssist, ThreadContextSummarization (+3 more)

### Community 1395 - "mermaid_render: postprocess/util.rs"
Cohesion: 0.38
Nodes (9): contrast_ratio_between(), linear_rgb_to_oklab(), linear_to_srgb(), oklab_to_linear_rgb(), relative_luminance(), srgb_to_linear(), sufficient_contrast_for_any_opaque_background(), text_color_for_background() (+1 more)

### Community 1396 - "openai_subscribed: codex_extra_headers()"
Cohesion: 0.25
Nodes (8): codex_extra_headers(), CodexCredentials, list_models(), now_ms(), refresh_token(), RefreshError, Fatal, Transient

### Community 1397 - "openai_subscribed: compact()"
Cohesion: 0.27
Nodes (5): compact(), compaction_response_stream(), stream_completion(), test_explicit_compaction_streams_with_codex_compaction_trigger(), test_server_side_compaction_streams_from_codex_responses()

### Community 1399 - "project: ContextServerSettings"
Cohesion: 0.20
Nodes (4): ContextServerSettings, Extension, Http, Stdio

### Community 1400 - "repl: CellControlType"
Cohesion: 0.18
Nodes (8): CellControlType, CellOptions, ClearCell, CollapseCell, ExpandCell, RerunCell, RunCell, StopCell

### Community 1402 - "AGENTS.md: Change Classification"
Cohesion: 0.18
Nodes (11): Change Classification, Documentation Automation Agent Guidelines, Does Not Require Documentation Update, Formatting, In-Scope Documentation, Out-of-Scope (Do Not Modify), Requires Documentation Update, Scope (+3 more)

### Community 1403 - ".doc-examples: simple-feature.md"
Cohesion: 0.18
Nodes (10): Command Palette {#command-palette}, File Finder {#file-finder}, Finding & Navigating, Go to Definition {#go-to-definition}, Go to Symbol {#go-to-symbol}, Outline Panel {#outline-panel}, Project Search {#project-search}, Quick Reference {#quick-reference} (+2 more)

### Community 1404 - "src: Plans {#plans}"
Cohesion: 0.18
Nodes (11): Plans {#plans}, Plans & Pricing, Spend Limits {#usage-spend-limits}, Student Plan {#student}, Trials {#trials}, Usage {#usage}, Zed Business {#business}, Zed Business {#business-spend-limits} (+3 more)

### Community 1405 - "src: Importing External Agent Threa"
Cohesion: 0.18
Nodes (11): Importing External Agent Threads {#importing-threads}, Multi-Root Folder Projects {#multi-root-folder-projects}, Multiple Projects {#multiple-projects}, Parallel Agents, Running Multiple Threads {#running-multiple-threads}, See Also {#see-also}, Switching Threads {#switching-threads}, Thread History {#threads-history} (+3 more)

### Community 1406 - "src: 1. Open a Project"
Cohesion: 0.18
Nodes (11): 1. Open a Project, 2. Learn the Essential Commands, 3. Configure Your Editor, 4. Set Up Your Language, 5. Try AI Features, Coming from Another Editor?, Getting Started, Join the Community (+3 more)

### Community 1407 - "src: globs.md"
Cohesion: 0.18
Nodes (10): Case insensitive matching, Examples, Glob Flavor, Globs, Implicit Wildcards, Introduction, Matching directories, Matching file extensions (+2 more)

### Community 1408 - "src: Choosing a language server"
Cohesion: 0.18
Nodes (11): Choosing a language server, Debugging, Install PHP, Intelephense, Laravel/Blade, PHP, PHP Tools, Phpactor (+3 more)

### Community 1409 - "src: r.md"
Cohesion: 0.18
Nodes (10): Air, Ark Installation, Configuring Air, Formatting, Installation, Linting, R, REditorSupport/languageserver Configuration (+2 more)

### Community 1410 - "src: performance.md"
Cohesion: 0.18
Nodes (10): Alternative: Building it yourself, Alternative: Building it yourself, In depth CPU profiling (Tracing), One time Setup/Building the profiler:, Rough quick CPU profiling (Flamechart), Setup/Building the importer:, Task/Async profiling, To Save a Trace: (+2 more)

### Community 1411 - "src: Axes"
Cohesion: 0.18
Nodes (11): Axes, Cursor Indicators, Diagnostics, Editor Scrollbar, Git Diff Indicators, Horizontal, Search Results Indicators, Selected Symbols Indicators (+3 more)

### Community 1413 - "workflows: autofix_pr.yml"
Cohesion: 0.18
Nodes (10): concurrency, cancel-in-progress, group, defaults, run, name, permissions, contents (+2 more)

### Community 1414 - "workflows: pr_number"
Cohesion: 0.18
Nodes (11): pr_number, run_clippy, on, workflow_dispatch, description, required, type, default (+3 more)

### Community 1415 - "workflows: POSTGRES_HOST_AUTH_METHOD"
Cohesion: 0.18
Nodes (11): POSTGRES_HOST_AUTH_METHOD, tests, env, image, options, ports, postgres, name (+3 more)

### Community 1416 - "workflows: default"
Cohesion: 0.18
Nodes (11): default, description, type, default, description, type, change-description, filter-repos (+3 more)

### Community 1417 - "workflows: POSTGRES_HOST_AUTH_METHOD"
Cohesion: 0.18
Nodes (11): POSTGRES_HOST_AUTH_METHOD, run_tests_linux, env, image, options, ports, if, runs-on (+3 more)

### Community 1418 - "workflows: env"
Cohesion: 0.33
Nodes (11): env, env, ACCOUNT_NAME, AZURE_CLIENT_ID, AZURE_CLIENT_SECRET, AZURE_TENANT_ID, CERT_PROFILE_NAME, ENDPOINT (+3 more)

### Community 1419 - "subprocessors.md: subprocessors.md"
Cohesion: 0.18
Nodes (10): AI Services (Zed-Hosted Models), AI Subprocessors, Analytics & Data Processing, Billing & Payments, Collaboration Services, Email & Communication, How Zed Uses Subprocessors, Infrastructure & Hosting (+2 more)

### Community 1420 - "terms.md: terms.md"
Cohesion: 0.18
Nodes (10): 10. Ownership; Feedback, 12. Indemnity, 14. Publicity, 1. Overview, 5. Customer Obligations, 6. Suspension of Service, 7.1. Collaboration Services, 7.2. Third-Party Integrations (+2 more)

### Community 1422 - "compliance: escape_markdown_link_text()"
Cohesion: 0.24
Nodes (4): escape_markdown_link_text(), escape_markdown_table_text(), ReportEntry<R>, ReportEntry<ReviewSuccess>

### Community 1423 - "collab: ChannelBufferCollaboratorId"
Cohesion: 0.31
Nodes (3): Model, Relation, Channel

### Community 1424 - "agent_servers: .list_sessions()"
Cohesion: 0.31
Nodes (5): AgentSessionListRequest, AgentSessionListResponse, session_info_work_dirs_deduplicate_cwd_and_additional_directories(), session_info_work_dirs_preserve_cwd_then_additional_directories(), work_dirs_from_session_info()

### Community 1428 - "channel: channel_store_tests.rs"
Cohesion: 0.60
Nodes (6): assert_channels(), init_test(), test_dangling_channel_paths(), test_update_channels(), test_update_channels_order_independent(), update_channels()

### Community 1429 - "collab: project_collaborator.rs"
Cohesion: 0.31
Nodes (3): Model, Relation, Project

### Community 1430 - "dap: dap_types::ModuleId"
Cohesion: 0.22
Nodes (3): dap_types::ModuleId, dap_types::OutputEventCategory, ProtoConversion

### Community 1431 - "deepseek: Request"
Cohesion: 0.20
Nodes (10): Request, ResponseFormat, JsonObject, Text, ToolChoice, Auto, None, Required (+2 more)

### Community 1432 - "editor: Editor"
Cohesion: 0.22
Nodes (6): Editor, editor_property_test(), TestAction, Backspace, Move, Type

### Community 1433 - "extension_api: ContextServerSettings"
Cohesion: 0.22
Nodes (4): ContextServerSettings, get_settings(), LanguageSettings, LspSettings

### Community 1435 - "gpui: focus_cleanup.rs"
Cohesion: 0.33
Nodes (7): CompletedBatch, CreatedEntity, ENTITY_COUNT, final_drops(), final_drops_with_windows(), measure(), no_drops()

### Community 1436 - "gpui_platform: bare_grid()"
Cohesion: 0.24
Nodes (5): bare_grid(), closed_inspector_inputs(), ClosedInspectorInput, measure_closed_inspector(), panel()

### Community 1437 - "gpui: .fmt()"
Cohesion: 0.20
Nodes (7): Effect, Defer, Emit, EntityCreated, Notify, NotifyGlobalObservers, RefreshWindows

### Community 1438 - "gpui: Poll"
Cohesion: 0.20
Nodes (3): Poll, Observation<T>, WithTimeout<T>

### Community 1439 - "json_schema_store: description"
Cohesion: 0.20
Nodes (10): description, type, type, (node|npm|pnpm|yarn), extends, volta, description, patternProperties (+2 more)

### Community 1443 - "livekit_client: CaptureInput"
Cohesion: 0.33
Nodes (3): CaptureInput, start_capture(), write_out()

### Community 1444 - "lmstudio: ImageUrl"
Cohesion: 0.36
Nodes (7): ImageUrl, MessageContent, Multipart, Plain, MessagePart, Image, Text

### Community 1445 - "migrator: m_2025_01_29/settings.rs"
Cohesion: 0.38
Nodes (6): LANGUAGE_SETTINGS_REPLACE, replace_edit_prediction_provider_setting(), replace_setting_in_languages(), replace_setting_name(), SETTINGS_PATTERNS, SETTINGS_STRING_REPLACE

### Community 1446 - "migrator: m_2026_02_25/settings.rs"
Cohesion: 0.36
Nodes (7): AGENT_SERVERS_KEY, BUILTIN_MAPPINGS, BuiltinMapping, migrate_builtin_agent_servers_to_registry(), migrate_builtin_entry(), migrate_one(), REGISTRY_COMPATIBLE_FIELDS

### Community 1447 - "migrator: m_2026_04_17/settings.rs"
Cohesion: 0.31
Nodes (7): migrate_one(), migrate_title_bar_value(), NEW_KEY, OLD_KEY, promote_show_branch_icon_true_to_show_branch_status_icon(), SETTINGS_KEY, TITLE_BAR_KEY

### Community 1448 - "open_ai: Role"
Cohesion: 0.22
Nodes (8): Role, Assistant, Developer, System, Tool, User, String, ResponseMessageItem

### Community 1449 - "project: CompletionDocumentation"
Cohesion: 0.33
Nodes (6): CompletionDocumentation, MultiLineMarkdown, MultiLinePlainText, SingleLine, SingleLineAndMultiLinePlainText, Undocumented

### Community 1451 - "remote: json_log.rs"
Cohesion: 0.31
Nodes (4): deserialize_level(), LogRecord, LogRecord<'a>, serialize_level()

### Community 1452 - "repl: ExecutionStatus"
Cohesion: 0.20
Nodes (10): ExecutionStatus, ConnectingToKernel, Executing, Finished, KernelErrored, Queued, Restarting, Shutdown (+2 more)

### Community 1453 - "settings_content: GitHostingProviderKind"
Cohesion: 0.20
Nodes (10): GitHostingProviderKind, Bitbucket, Forgejo, Gerrit, Gitea, Github, Gitiles, Gitlab (+2 more)

### Community 1456 - "util: assertions.rs"
Cohesion: 0.24
Nodes (6): assert_set_eq!, set_eq!, SetEqError, LeftMissing, RightMissing, SetEqError<T>

### Community 1457 - "watch: watch/src/error.rs"
Cohesion: 0.24
Nodes (3): NoReceiverError, Sender<T>, State<T>

### Community 1458 - "zed_actions: ExtensionCategoryFilter"
Cohesion: 0.20
Nodes (10): ExtensionCategoryFilter, ContextServers, DebugAdapters, Grammars, IconThemes, Languages, LanguageServers, Snippets (+2 more)

### Community 1459 - "zlog: EnabledStatus"
Cohesion: 0.27
Nodes (6): EnabledStatus, Disabled, Enabled, NotConfigured, is_possibly_enabled_level(), is_scope_enabled()

### Community 1460 - "AGENTS.md: Blockquote Formatting"
Cohesion: 0.20
Nodes (10): Blockquote Formatting, Code Block Annotations, Cross-Linking, Custom Preprocessor, Documentation System, Formatting Requirements, Image References, Key Files (+2 more)

### Community 1461 - "src: Applicable Terms {#applicable-"
Cohesion: 0.20
Nodes (10): Applicable Terms {#applicable-terms}, Business Controls {#business-controls}, Data Collected from Edit Prediction Training {#data-collected-edit-prediction-training}, Data Collected from Feedback {#data-collected-ai-feedback}, Edit Prediction Training Data {#edit-predictions}, Feedback and Training Data, File Exclusions {#file-exclusions}, Response Ratings and Feedback {#ai-feedback-with-ratings} (+2 more)

### Community 1462 - "src: Automatic prompt"
Cohesion: 0.20
Nodes (10): Automatic prompt, Dev Containers, Editing the dev container configuration, Extensions, Known Limitations, Manual open, Requirements, See also (+2 more)

### Community 1463 - "src: lua.md"
Cohesion: 0.20
Nodes (9): Formatting, Inlay Hints, Lua, LuaCATS Definitions, LuaLS Formatting, luarc.json, LÖVE (Love2D) {#love2d}, PlaydateSDK (+1 more)

### Community 1464 - "src: Attach debugger to a server ru"
Cohesion: 0.20
Nodes (10): Attach debugger to a server running in web browser (`npx serve`), Code Lens, Configuring JavaScript debug tasks, Debugging, Inlay Hints, Language servers, Large projects, See also (+2 more)

### Community 1465 - "src: Activate on close"
Cohesion: 0.20
Nodes (10): Activate on close, Close Position, Drag And Drop Selection, Editor Tabs, File Icons, Git Status, Session, Show close button (+2 more)

### Community 1466 - "src: Auto Fetch"
Cohesion: 0.20
Nodes (10): Auto Fetch, Branch Picker, Diff Base, Git, Git Dates, Git Gutter, Git Integration and Display {#git-integration-and-display}, Gutter Debounce (+2 more)

### Community 1467 - "src: Codestral Predictions {#edit-p"
Cohesion: 0.20
Nodes (10): Codestral Predictions {#edit-predictions-codestral}, Copilot Predictions {#edit-predictions-copilot}, Disabled Globs, Display and Data Collection {#edit-prediction-display-and-data-collection}, Edit Prediction Button, Edit Prediction Provider, Edit Predictions, Ollama Predictions {#edit-predictions-ollama} (+2 more)

### Community 1468 - "src: Admin {#role-admin}"
Cohesion: 0.20
Nodes (10): Admin {#role-admin}, Billing Manager {#role-billing-manager}, Changing a Member's Role {#changing-roles}, Inviting Members {#inviting-members}, Managing User Roles {#managing-users}, Member {#role-member}, Owner {#role-owner}, Removing a Member {#removing-members} (+2 more)

### Community 1469 - "workflows: CC"
Cohesion: 0.20
Nodes (10): CC, CXX, jobs, run_autofix, has_changes, pull-requests, env, outputs (+2 more)

### Community 1470 - "workflows: with"
Cohesion: 0.20
Nodes (10): with, app-id, clean, owner, permission-contents, permission-pull-requests, permission-workflows, private-key (+2 more)

### Community 1471 - "workflows: env"
Cohesion: 0.20
Nodes (10): env, if, name, runs-on, timeout-minutes, CC, CXX, DOCS_AMPLITUDE_API_KEY (+2 more)

### Community 1472 - "workflows: steps"
Cohesion: 0.20
Nodes (10): steps, CHANNEL, DOCS_CHANNEL, MDBOOK_BOOK__SITE_URL, env, id, if, name (+2 more)

### Community 1473 - "workflows: defaults"
Cohesion: 0.20
Nodes (10): defaults, defaults, if, needs, runs-on, timeout-minutes, run, check_rust (+2 more)

### Community 1474 - "workflows: if"
Cohesion: 0.20
Nodes (10): if, needs, runs-on, timeout-minutes, jobs, check_extension, tests_pass, if (+2 more)

### Community 1475 - "workflows: rollout_workflows_to_extension"
Cohesion: 0.20
Nodes (10): rollout_workflows_to_extension, repo, if, needs, runs-on, strategy, timeout-minutes, fail-fast (+2 more)

### Community 1476 - "workflows: publish_extension_cli.yml"
Cohesion: 0.20
Nodes (9): defaults, run, env, CARGO_INCREMENTAL, CARGO_TERM_COLOR, name, permissions, contents (+1 more)

### Community 1477 - "workflows: orchestrate"
Cohesion: 0.20
Nodes (10): orchestrate, if, outputs, runs-on, changed_extensions, changed_packages, run_action_checks, run_docs (+2 more)

### Community 1478 - "lints: async_block_without_await.rs"
Cohesion: 0.31
Nodes (5): AsyncWork, main(), returns_42(), sync_fn(), Worker

### Community 1479 - "SETTINGS_SYNC.md: SETTINGS_SYNC.md"
Cohesion: 0.20
Nodes (9): Code layout, Decisions, GitHub calls, Milestones, Needs from the owner, Settings Sync — design and plan, Status, What syncs (+1 more)

### Community 1480 - "gpui_util: BuildHasher"
Cohesion: 0.22
Nodes (3): debug_panic!, TypeIdHashBuilder, TypeIdHasher

### Community 1481 - "CLAUDE.md: CLAUDE.md"
Cohesion: 0.22
Nodes (8): Commands, Git layout, graphify, Licensing — must respect, Prerequisites (macOS), Rules for this fork, VEGA, Where branding lives

### Community 1483 - "agent_servers: .drop()"
Cohesion: 0.22
Nodes (3): repeated_spawns_preserve_command_and_stdio(), spawn_stdio(), StdioProcess

### Community 1484 - "agent_ui: Formatter"
Cohesion: 0.25
Nodes (3): SubagentCardId, SessionMessage, ToolCall

### Community 1486 - "collab: project_repository_statuses.rs"
Cohesion: 0.28
Nodes (7): Model, Relation, StatusKind, Ignored, Tracked, Unmerged, Untracked

### Community 1487 - "collab: tables/room.rs"
Cohesion: 0.28
Nodes (6): Model, Relation, Channel, Follower, Project, RoomParticipant

### Community 1488 - "collab: worktree_settings_file.rs"
Cohesion: 0.28
Nodes (7): LocalSettingsKind, Debug, Editorconfig, Settings, Tasks, Model, Relation

### Community 1489 - "deepseek: FunctionContent"
Cohesion: 0.22
Nodes (9): FunctionContent, RequestMessage, Assistant, System, Tool, User, ToolCall, ToolCallContent (+1 more)

### Community 1490 - "deepseek: ImageUrl"
Cohesion: 0.36
Nodes (7): ImageUrl, MessageContent, Multipart, Plain, MessagePart, Image, Text

### Community 1491 - "edit_prediction: init_test()"
Cohesion: 0.33
Nodes (8): init_test(), prediction_edits_for_single_file_diff(), test_apply_diff_unique_via_previous_context(), test_prediction_edits_for_single_file_diff_can_target_project_file(), test_prediction_edits_for_single_file_diff_drops_marker_only_edit(), test_prediction_edits_for_single_file_diff_places_cursor_after_inline_completion(), test_prediction_edits_for_single_file_diff_strips_cursor_marker_that_overlaps_source(), test_prediction_edits_for_single_file_diff_strips_inline_cursor_marker()

### Community 1492 - "editor: invisibles.rs"
Cohesion: 0.25
Nodes (7): C0_SYMBOLS, DEL, FIXED_WIDTH_SPACE, IDEOGRAPHIC_SPACE, is_format_or_other_invisible_character(), replacement(), should_preserve_invisible_character()

### Community 1494 - "gpui: contexts.md"
Cohesion: 0.22
Nodes (8): `App`, `AsyncApp` and `AsyncWindowContext`, `Context<T>`, Contexts, `Entity<T>`, Non-Context Core Types, `TestAppContext`, `Window`

### Community 1495 - "gpui: gpui/README.md"
Cohesion: 0.22
Nodes (8): Additional Topics, Dependencies, Getting Started, `gpui_platform`, macOS, Other Resources, The Big Picture, Welcome to GPUI!

### Community 1496 - "gpui: SpringAnimation<T>"
Cohesion: 0.22
Nodes (7): SpringAnimation<T>, SpringPlayback, Cancelled, Completed, Paused, Running, Stopped

### Community 1497 - "gpui_util: LogErrorFuture<F>"
Cohesion: 0.39
Nodes (3): LogErrorFuture<F>, LogErrorWithBacktraceFuture<F>, UnwrapFuture<F>

### Community 1498 - "gpui_windows: check_unicode_separator_cursor"
Cohesion: 0.25
Nodes (6): check_unicode_separator_cursor(), get_system_subpixel_rendering(), get_system_ui_font_name(), tabs_preserve_native_advances_and_cursor(), unicode_line_separators_preserve_shaped_line_cursor(), unicode_paragraph_separators_preserve_shaped_line_cursor()

### Community 1500 - "llama_cpp: ImageUrl"
Cohesion: 0.39
Nodes (7): ImageUrl, MessageContent, Multipart, Plain, MessagePart, Image, Text

### Community 1501 - "project: DiffKind"
Cohesion: 0.22
Nodes (6): DiffKind, SinceOid, Staged, Uncommitted, Unstaged, GitDiffOperations

### Community 1502 - "project: Candidates"
Cohesion: 0.25
Nodes (5): Candidates, Directories, Entries, Files, PathMatchCandidateSet

### Community 1503 - "project: telemetry_snapshot.rs"
Cohesion: 0.42
Nodes (3): GitState, TelemetrySnapshot, TelemetryWorktreeSnapshot

### Community 1505 - "src: Adding Context"
Cohesion: 0.22
Nodes (9): Adding Context, Getting Started, Inline Assistant, Inline Assistant vs. Edit Prediction, Multiple Cursors, Multiple Models, Parallel Generations, Prefilling Prompts (+1 more)

### Community 1506 - "src: AI Quick Start"
Cohesion: 0.22
Nodes (9): AI Quick Start, Change AI Settings {#ai-settings}, Choose Which Models the Zed Agent Uses {#model-access}, Configure the Zed Agent {#configure-zed-agent}, Learn More {#learn-more}, Turn AI Off {#turn-ai-off}, Use a Specific AI Feature {#features}, Use Another Coding Agent in Zed {#agent-cli} (+1 more)

### Community 1507 - "src: Agent Path Boundaries {#agent-"
Cohesion: 0.22
Nodes (9): Agent Path Boundaries {#agent-path-boundaries}, llama.cpp Context Length {#llama-cpp-context}, llama.cpp {#llama-cpp}, LM Studio {#lm-studio}, Local Edit Prediction {#edit-prediction}, Local OpenAI-Compatible Servers {#openai-compatible}, Ollama Context Length {#ollama-context}, Ollama {#ollama} (+1 more)

### Community 1508 - "src: ChatGPT Plus / Pro {#chatgpt}"
Cohesion: 0.22
Nodes (9): ChatGPT Plus / Pro {#chatgpt}, Claude Pro / Max {#claude}, Cursor {#cursor}, DeepSeek {#deepseek}, GitHub Copilot Enterprise {#copilot-enterprise}, GitHub Copilot {#github-copilot}, OpenCode Zen / Go {#opencode}, Subscriptions Used Through Agent Harnesses {#agent-harnesses} (+1 more)

### Community 1509 - "src: Appearance"
Cohesion: 0.22
Nodes (9): Appearance, Customize Zed in 5 Minutes, Font Ligatures, Fonts, Icon Themes, Line Height, Themes, UI Elements (+1 more)

### Community 1510 - "src: Changing a member's role"
Cohesion: 0.22
Nodes (9): Changing a member's role, Creating an organization, Inviting members, Managing members, Multiple Organizations, Organization Dashboard, Organizations, Personal Organizations (+1 more)

### Community 1511 - "src: Built-in terminal"
Cohesion: 0.22
Nodes (9): Built-in terminal, Environment Variables, Language servers, Launched from the CLI, Launched via window manager, Dock, or launcher, Look-up of language servers, Tasks, Where and how are environment variables used? (+1 more)

### Community 1512 - "src: Capabilities"
Cohesion: 0.22
Nodes (9): Capabilities, `download_file`, Examples, Examples, Examples, Extension Capabilities, `npm:install`, `process:exec` (+1 more)

### Community 1513 - "src: Client-Side Metrics"
Cohesion: 0.22
Nodes (9): Client-Side Metrics, Concerns and Questions, Configuring Telemetry Settings, Dataflow, Diagnostics, Server-Side Metrics, Telemetry in Zed, Types of Telemetry (+1 more)

### Community 1514 - "src: Agent Error Messages"
Cohesion: 0.22
Nodes (9): Agent Error Messages, Language Server Issues, macOS, "Max tokens reached", Performance Issues (Profiling), Retrieve Zed and System Information, Startup and Workspace Issues, Troubleshooting (+1 more)

### Community 1515 - "src: Adding Folders to a Project"
Cohesion: 0.22
Nodes (9): Adding Folders to a Project, From Open Recent, From the CLI, How Projects Open, Opening in a New Window, See Also, Via Settings, Windows & Projects (+1 more)

### Community 1517 - "DISCUSSION_TEMPLATE: feature-requests.yml"
Cohesion: 0.22
Nodes (8): label, value, body, attributes, id, type, validations, required

### Community 1518 - "workflows: permissions"
Cohesion: 0.22
Nodes (9): permissions, permissions, promote_to_stable, contents, pull-requests, if, needs, permissions (+1 more)

### Community 1519 - "workflows: BRANCH"
Cohesion: 0.22
Nodes (9): BRANCH, CHANNEL, COMMIT, GIT_AUTHOR_EMAIL, GIT_AUTHOR_NAME, GIT_COMMITTER_EMAIL, GIT_COMMITTER_NAME, GITHUB_TOKEN (+1 more)

### Community 1520 - "workflows: steps"
Cohesion: 0.39
Nodes (9): steps, CLUSTER_NAME, steps, env, name, run, uses, steps (+1 more)

### Community 1521 - "workflows: steps"
Cohesion: 0.42
Nodes (9): steps, steps, steps, id, if, name, run, uses (+1 more)

### Community 1522 - "workflows: GH_TOKEN"
Cohesion: 0.22
Nodes (9): GH_TOKEN, NEXTEST_NO_TESTS, PACKAGE_NAME, PR_USER_LOGIN, RESULT_CHECK_EXTENSION, RESULT_CHECK_RUST, RESULT_ORCHESTRATE, VERSION_CHANGED (+1 more)

### Community 1523 - "workflows: COMMIT_SHA"
Cohesion: 0.22
Nodes (9): COMMIT_SHA, FILTER_REPOS, GH_TOKEN, MATRIX_REPO, PR_NUMBER, PREV_COMMIT, REMOVED_CI, REMOVED_SHARED (+1 more)

### Community 1524 - "workflows: if"
Cohesion: 0.22
Nodes (9): if, outputs, runs-on, timeout-minutes, fetch_extension_repos, prev_commit, removed_ci, removed_shared (+1 more)

### Community 1526 - "danger: danger/tsconfig.json"
Cohesion: 0.22
Nodes (8): compilerOptions, allowJs, module, noEmit, skipLibCheck, strict, target, include

### Community 1527 - "lints: lints/README.md"
Cohesion: 0.22
Nodes (8): Current lints, Demo, How to run, lints, Prerequisites, Run a single lint, Run all lints against a single crate, Run all lints against the whole repo

### Community 1528 - "skills: lint-creator/SKILL.md"
Cohesion: 0.25
Nodes (7): Adding a dylint to `tooling/lints`, After the lint works, Before writing anything, Diagnostics rules, Layout, UI fixture sections, UI tests (required)

### Community 1529 - "settings: gutter"
Cohesion: 0.25
Nodes (8): gutter, bookmarks, breakpoints, folds, git_gutter_width, line_numbers, min_line_number_digits, runnables

### Community 1530 - "settings: markdown_preview"
Cohesion: 0.25
Nodes (8): markdown_preview, code_font_family, font_family, font_size, heading_font_weight, limit_content_width, max_width, open_markdown_files_in_preview

### Community 1531 - "settings: preview_tabs"
Cohesion: 0.25
Nodes (8): preview_tabs, enable_keep_preview_on_code_navigation, enable_preview_file_from_code_navigation, enable_preview_from_file_finder, enable_preview_from_multibuffer, enable_preview_from_project_panel, enable_preview_multibuffer_from_code_navigation, enabled

### Community 1532 - "settings: search"
Cohesion: 0.25
Nodes (8): search, button, case_sensitive, center_on_match, include_ignored, regex, search_on_type, whole_word

### Community 1533 - "gpui_apple: config"
Cohesion: 0.43
Nodes (6): compile_metal_shaders(), emit_stitched_shaders(), find_gpui_crate_dir(), generate_shader_bindings(), main(), run()

### Community 1535 - "cli: OpenBehavior"
Cohesion: 0.25
Nodes (8): OpenBehavior, Add, AlwaysNew, Classic, Default, ExistingWindow, PreferNewWindow, Reuse

### Community 1536 - "collab: tables/buffer.rs"
Cohesion: 0.32
Nodes (5): Model, Relation, Channel, Operations, Snapshots

### Community 1537 - "collab: tables/notification.rs"
Cohesion: 0.39
Nodes (3): Model, Relation, Recipient

### Community 1539 - "dap: inline_value.rs"
Cohesion: 0.32
Nodes (7): InlineValueLocation, VariableLookupKind, Expression, Variable, VariableScope, Global, Local

### Community 1540 - "db: db_path()"
Cohesion: 0.29
Nodes (5): db_path(), DbScope, GlobalDbScope, open_db(), ReleaseChannel

### Community 1541 - "deepseek: Role"
Cohesion: 0.29
Nodes (6): Role, Assistant, System, Tool, User, String

### Community 1544 - "extension_host: dap::AttachRequest"
Cohesion: 0.36
Nodes (4): dap::AttachRequest, latest::dap::AttachRequest, AttachRequest, task::AttachRequest

### Community 1545 - "extension_host: dap::LaunchRequest"
Cohesion: 0.36
Nodes (4): dap::LaunchRequest, latest::dap::LaunchRequest, LaunchRequest, task::LaunchRequest

### Community 1547 - "gpui_ios: tests/text_input.rs"
Cohesion: 0.36
Nodes (3): handler_queries(), IosWindowState, WindowReference

### Community 1548 - "gpui: RandomizedChildMutation"
Cohesion: 0.25
Nodes (8): RandomizedChildMutation, Border, Bounds, Clipping, Color, Opacity, Text, Visibility

### Community 1550 - "migrator: m_2025_01_30/keymap.rs"
Cohesion: 0.39
Nodes (4): action_argument_snake_case(), ACTION_ARGUMENT_SNAKE_CASE_REPLACE, KEYMAP_PATTERNS, to_snake_case()

### Community 1551 - "migrator: m_2025_03_03/keymap.rs"
Cohesion: 0.39
Nodes (4): KEYMAP_PATTERNS, replace_string_action(), STRING_REPLACE, STRING_TO_ARRAY_REPLACE

### Community 1552 - "mistral: Role"
Cohesion: 0.29
Nodes (6): Role, Assistant, System, Tool, User, String

### Community 1553 - "open_router: Role"
Cohesion: 0.29
Nodes (6): Role, Assistant, System, Tool, User, String

### Community 1554 - "openai_subscribed: CatalogModel"
Cohesion: 0.25
Nodes (8): CatalogModel, ModelServiceTier, ModelsResponse, ModelVisibility, Hide, List, None, ReasoningEffortPreset

### Community 1557 - "settings_content: serde_helper.rs"
Cohesion: 0.29
Nodes (4): serialize_f32_with_two_decimal_places(), serialize_optional_f32_with_two_decimal_places(), TestNonOptional, TestOptional

### Community 1558 - "settings_profile_selector: active_settings_profile_picker"
Cohesion: 0.39
Nodes (6): active_settings_profile_picker(), init_test(), test_settings_profile_selector_is_in_user_configuration_order(), test_settings_profile_selector_state(), test_settings_profile_with_default_base(), test_settings_profile_with_user_base()

### Community 1559 - "settings_ui: a11y_text_position()"
Cohesion: 0.25
Nodes (5): a11y_text_position(), build_a11y_text_runs(), building_text_runs_never_panics(), char_index_for_byte(), is_word_char()

### Community 1560 - "snippet_provider: ListOrDirect"
Cohesion: 0.29
Nodes (5): ListOrDirect, List, Single, Vec<String>, VsCodeSnippet

### Community 1562 - "util: marked_text_offsets_by()"
Cohesion: 0.32
Nodes (6): marked_text_offsets_by(), marked_text_ranges_by(), TextRangeMarker, Empty, Range, ReverseRange

### Community 1564 - "zed: PreviewTarget"
Cohesion: 0.32
Nodes (5): PreviewTarget, Markdown, Svg, TabularData, QuickActionBar

### Community 1565 - "zlog: Record"
Cohesion: 0.25
Nodes (4): Record, SourceFmt, scope_ref_new(), ScopeRef

### Community 1566 - "src: Advanced Configuration/JDTLS i"
Cohesion: 0.25
Nodes (8): Advanced Configuration/JDTLS initialization Options, Debugging, Extension Install, Install OpenJDK, Java, Launch Scripts (aka Tasks) in Windows, Quick start and configuration, See also

### Community 1567 - "src: yaml.md"
Cohesion: 0.25
Nodes (7): Configuration, Custom Tags, Formatting, Prettier Formatting, Schemas, YAML, yaml-language-server Formatting

### Community 1568 - "src: Button"
Cohesion: 0.25
Nodes (8): Button, Case Sensitive, Center On Match, Include Ignore, Regex, Search, Search On Type, Whole Word

### Community 1569 - "src: Build Your Theme"
Cohesion: 0.25
Nodes (8): Build Your Theme, Configuring a Theme, Installing New Themes, Local Themes {#local-themes}, Selecting a Theme, Theme Overrides, Themes, Toggle Theme Mode from the Keyboard

### Community 1570 - "workflows: if"
Cohesion: 0.25
Nodes (8): if, needs, runs-on, jobs, bump_main, resolve_versions, if, runs-on

### Community 1571 - "workflows: steps"
Cohesion: 0.57
Nodes (8): steps, steps, steps, steps, id, name, run, uses

### Community 1572 - "workflows: cherry_pick.yml"
Cohesion: 0.25
Nodes (7): defaults, run, name, permissions, contents, run-name, shell

### Community 1573 - "workflows: jobs"
Cohesion: 0.25
Nodes (8): jobs, run_cherry_pick, runs-on, steps, id, name, run, uses

### Community 1574 - "workflows: CC"
Cohesion: 0.29
Nodes (8): CC, CXX, style, env, if, name, runs-on, env

### Community 1575 - "workflows: deploy_docs.yml"
Cohesion: 0.25
Nodes (7): defaults, run, jobs, name, permissions, contents, shell

### Community 1576 - "workflows: extension_tests.yml"
Cohesion: 0.25
Nodes (7): concurrency, cancel-in-progress, group, defaults, name, permissions, contents

### Community 1577 - "workflows: steps"
Cohesion: 0.43
Nodes (8): steps, steps, steps, id, name, run, uses, working-directory

### Community 1578 - "third-party-terms.md: third-party-terms.md"
Cohesion: 0.25
Nodes (7): Anthropic, GitHub, Google, OpenAI, OpenRouter, Vercel, xAI

### Community 1579 - "settings: file_finder"
Cohesion: 0.29
Nodes (7): file_finder, file_icons, include_channels, include_ignored, modal_max_width, prefill_query_from_selection, skip_focus_for_active_in_search

### Community 1580 - "settings: minimap"
Cohesion: 0.29
Nodes (7): minimap, current_line_highlight, display_in, max_width_columns, show, thumb, thumb_border

### Community 1581 - "settings: tabs"
Cohesion: 0.29
Nodes (7): tabs, activate_on_close, close_position, file_icons, git_status, show_close_button, show_diagnostics

### Community 1583 - "collab: ContactId"
Cohesion: 0.38
Nodes (4): Model, Relation, UserARoomParticipant, UserBRoomParticipant

### Community 1584 - "acp_thread: SandboxPermission"
Cohesion: 0.29
Nodes (5): SandboxPermission, AllowAlways, AllowOnce, AllowThread, Deny

### Community 1585 - "agent: Context"
Cohesion: 0.29
Nodes (7): Context, Core Concepts, Images, Prompts, Resources, Server, Tools

### Community 1587 - "collab: dotenv.rs"
Cohesion: 0.38
Nodes (4): main(), add_vars(), get_dotenv_vars(), load_dotenv()

### Community 1588 - "collab: observed_buffer_edits.rs"
Cohesion: 0.38
Nodes (4): Model, Relation, Buffer, User

### Community 1589 - "collab: tables/user.rs"
Cohesion: 0.33
Nodes (5): Model, Relation, ChannelMemberships, HostedProjects, RoomParticipant

### Community 1591 - "edit_prediction_cli: repair.md"
Cohesion: 0.29
Nodes (6): Instructions, Output Format, Quality Feedback, Repair Request, Your Improved Prediction, Your Previous Prediction (word-diff format)

### Community 1592 - "editor: LspDataKind"
Cohesion: 0.29
Nodes (7): LspDataKind, CodeLens, DocumentColors, DocumentLinks, DocumentSymbols, FoldingRanges, PullDiagnostics

### Community 1593 - "editor: MinimapVisibility"
Cohesion: 0.29
Nodes (3): MinimapVisibility, Disabled, Enabled

### Community 1595 - "extension_api: extension_api/README.md"
Cohesion: 0.29
Nodes (6): Cargo metadata, Compatible Zed versions, Extension Manifest, Implementing an Extension, Testing your extension, The Zed Rust Extension API

### Community 1596 - "json_schema_store: description"
Cohesion: 0.29
Nodes (7): description, items, markdownDescription, type, uniqueItems, x-intellij-html-description, customConditions

### Community 1597 - "json_schema_store: extendsDefinition"
Cohesion: 0.29
Nodes (7): extendsDefinition, description, markdownDescription, oneOf, x-intellij-html-description, properties, extends

### Community 1598 - "json_schema_store: description"
Cohesion: 0.29
Nodes (7): description, items, markdownDescription, type, uniqueItems, x-intellij-html-description, moduleSuffixes

### Community 1599 - "json_schema_store: anyOf"
Cohesion: 0.29
Nodes (7): anyOf, default, description, markdownDescription, type, x-intellij-html-description, newLine

### Community 1600 - "json_schema_store: target"
Cohesion: 0.29
Nodes (7): target, anyOf, default, description, markdownDescription, type, x-intellij-html-description

### Community 1601 - "language_extension: extract_int()"
Cohesion: 0.29
Nodes (3): extract_int(), lsp_completion_item_kind_to_extension(), lsp_insert_text_format_to_extension()

### Community 1603 - "migrator: m_2025_03_06/keymap.rs"
Cohesion: 0.43
Nodes (3): KEYMAP_PATTERNS, replace_array_with_single_string(), TRANSFORM_ARRAY

### Community 1604 - "migrator: m_2025_04_15/keymap.rs"
Cohesion: 0.43
Nodes (3): KEYMAP_PATTERNS, replace_string_action(), STRING_REPLACE

### Community 1605 - "migrator: m_2025_12_08/keymap.rs"
Cohesion: 0.43
Nodes (3): KEYMAP_PATTERNS, replace_string_action(), STRING_REPLACE

### Community 1606 - "openai_subscribed: make_fresh_credentials()"
Cohesion: 0.48
Nodes (5): make_fresh_credentials(), test_model_catalog_failure_preserves_fallback_models(), test_model_catalog_request_times_out(), test_model_catalog_uses_account_visible_models(), test_obsolete_model_catalog_cannot_replace_newer_models()

### Community 1610 - "terminal: CursorShape"
Cohesion: 0.33
Nodes (5): CursorShape, Bar, Block, Hollow, Underline

### Community 1611 - "theme_importer: Appearance"
Cohesion: 0.38
Nodes (5): Appearance, AppearanceContent, ThemeAppearanceJson, Dark, Light

### Community 1612 - "src: Accessing admin controls"
Cohesion: 0.29
Nodes (7): Accessing admin controls, Admin Controls, Agent Thread Feedback, Collaboration, Data sharing, Edit Predictions, Hosted AI models

### Community 1613 - "src: diagnostics.md"
Cohesion: 0.29
Nodes (6): Diagnostics, Editor tabs, Inline diagnostics (Error lens), Other UI places, Project Panel, Regular diagnostics

### Community 1614 - "src: Build and Debug Binary"
Cohesion: 0.29
Nodes (7): Build and Debug Binary, C, Clangd: Force detect as C, CMake Compile Commands, Compile Commands, Debugging, Formatting

### Community 1615 - "src: Inline Schema Specification"
Cohesion: 0.29
Nodes (7): Inline Schema Specification, JSON, JSON Language Server, JSON Lines Preview, JSONC, JSONC Prettier Formatting, Schema Specification via Settings

### Community 1616 - "src: kotlin.md"
Cohesion: 0.29
Nodes (6): Configuration, JAVA_HOME, JVM Target, Kotlin, Kotlin Language Server, Kotlin LSP

### Community 1617 - "src: proto.md"
Cohesion: 0.29
Nodes (6): Configuration, Formatting, Install protobuf-language-server, Install ProtoLS, Proto, Setup

### Community 1618 - "src: Configuration"
Cohesion: 0.29
Nodes (7): Configuration, Emacs, Modelines, Notes, Supported Emacs Variables, Supported Vim Options, Vim

### Community 1619 - "src: Current Line Highlight"
Cohesion: 0.29
Nodes (7): Current Line Highlight, Display In {#minimap-display-in}, Maximum Width Columns {#minimap-max-width-columns}, Minimap, Show Mode, Thumb Border, Thumb Display

### Community 1620 - "src: Enable keep preview on code na"
Cohesion: 0.29
Nodes (7): Enable keep preview on code navigation, Enable preview file from code navigation, Enable preview from file finder, Enable preview from multibuffer, Enable preview from project panel, Enable preview multibuffer from code navigation, Preview tabs

### Community 1621 - "src: File Finder"
Cohesion: 0.29
Nodes (7): File Finder, File Icons, Include Channels {#file-finder-include-channels}, Include Ignored {#file-finder-include-ignored}, Modal Max Width, Prefill Query from Selection {#file-finder-prefill-query-from-selection}, Skip Focus For Active In Search

### Community 1622 - "workflows: steps"
Cohesion: 0.43
Nodes (7): steps, steps, id, if, name, run, uses

### Community 1623 - "workflows: GIT_AUTHOR_EMAIL"
Cohesion: 0.29
Nodes (7): GIT_AUTHOR_EMAIL, GIT_AUTHOR_NAME, GIT_COMMITTER_EMAIL, GIT_COMMITTER_NAME, GITHUB_TOKEN, PR_NUMBER, env

### Community 1624 - "workflows: description"
Cohesion: 0.29
Nodes (7): description, required, type, channel, on, workflow_dispatch, inputs

### Community 1625 - "workflows: with"
Cohesion: 0.29
Nodes (7): with, cache, clean, fetch-depth, path, token, tool

### Community 1626 - "workflows: description"
Cohesion: 0.29
Nodes (7): description, required, description, required, app-id, app-secret, secrets

### Community 1627 - "workflows: if"
Cohesion: 0.29
Nodes (7): if, needs, outputs, runs-on, timeout-minutes, create_version_label, tag

### Community 1628 - "workflows: env"
Cohesion: 0.29
Nodes (7): env, CARGO_BUILD_TARGET, CARGO_INCREMENTAL, CARGO_TERM_COLOR, RUST_BACKTRACE, RUSTUP_TOOLCHAIN, ZED_EXTENSION_CLI_SHA

### Community 1629 - "workflows: working-directory"
Cohesion: 0.29
Nodes (7): working-directory, on, workflow_call, inputs, default, description, type

### Community 1630 - "workflows: orchestrate"
Cohesion: 0.29
Nodes (7): orchestrate, defaults, if, outputs, runs-on, check_extension, check_rust

### Community 1631 - "workflows: with"
Cohesion: 0.29
Nodes (7): with, cache, clean, fetch-depth, key, path, tool

### Community 1632 - "workflows: message"
Cohesion: 0.29
Nodes (7): message, description, required, type, on, workflow_dispatch, inputs

### Community 1633 - "workflows: steps"
Cohesion: 0.57
Nodes (7): steps, id, name, run, uses, steps, steps

### Community 1634 - "workflows: RUST_LIB_BACKTRACE"
Cohesion: 0.29
Nodes (7): RUST_LIB_BACKTRACE, run_tests_mac, env, if, needs, runs-on, timeout-minutes

### Community 1635 - "terms.md: 3.1. Price"
Cohesion: 0.29
Nodes (7): 3.1. Price, 3.2. Taxes, 3.3. Authorization, 3.4. Subscription Service, 3.5. Consumption Fees, 3.6. Delinquent Accounts, 3. General Payment Terms

### Community 1638 - "uninstall.sh: uninstall.sh"
Cohesion: 0.52
Nodes (6): check_remaining_installations(), linux(), macos(), main(), prompt_remove_preferences(), uninstall.sh script

### Community 1639 - "crates_io_client: CratesIoError"
Cohesion: 0.33
Nodes (5): CratesIoError, NotFound, PermissionDenied, Request, Upstream

### Community 1640 - "crates_io_client: parse_owner_additions()"
Cohesion: 0.33
Nodes (6): parse_owner_additions(), OwnerAddition, Other, TeamAdded, UserAlreadyInvited, UserInvited

### Community 1641 - "ROADMAP.md: ROADMAP.md"
Cohesion: 0.29
Nodes (6): Ongoing, Phase 0 — Setup, Phase 1 — Independent build, Phase 2 — AI-first, Phase 3 — Ship, VEGA roadmap

### Community 1642 - "settings: toolbar"
Cohesion: 0.33
Nodes (6): toolbar, agent_review, breadcrumbs, code_actions, quick_actions, selections_menu

### Community 1646 - "buffer_diff: HunkStatus"
Cohesion: 0.33
Nodes (6): HunkStatus, HunkStatus, DiffHunkStatusKind, Added, Deleted, Modified

### Community 1647 - "collab: collab/README.md"
Cohesion: 0.33
Nodes (5): Database setup, Deployment, Local Development, Testing collaborative features locally, Zed Server

### Community 1648 - "collab: contributor.rs"
Cohesion: 0.40
Nodes (3): Model, Relation, User

### Community 1649 - "collab: JoinChannelInternalResponse"
Cohesion: 0.40
Nodes (3): JoinChannelInternalResponse, Response<proto::JoinChannel>, Response<proto::JoinRoom>

### Community 1650 - "context_server: BearerError"
Cohesion: 0.33
Nodes (5): BearerError, InsufficientScope, InvalidRequest, InvalidToken, Other

### Community 1651 - "context_server: Completion"
Cohesion: 0.40
Nodes (5): Completion, CompletionTotal, Exact, HasMore, Unknown

### Community 1654 - "editor: ActiveDiagnostic"
Cohesion: 0.33
Nodes (5): ActiveDiagnostic, All, Group, None, ActiveDiagnosticGroup

### Community 1656 - "editor: .report_editor_event()"
Cohesion: 0.40
Nodes (4): ReportEditorEvent, Closed, EditorOpened, Saved

### Community 1658 - "extension_host: lsp::InsertTextFormat"
Cohesion: 0.47
Nodes (3): lsp::InsertTextFormat, lsp::InsertTextFormat, InsertTextFormat

### Community 1659 - "extension_host: lsp::Symbol"
Cohesion: 0.47
Nodes (3): lsp::Symbol, lsp::Symbol, Symbol

### Community 1660 - "extension_host: lsp::SymbolKind"
Cohesion: 0.47
Nodes (3): lsp::SymbolKind, lsp::SymbolKind, SymbolKind

### Community 1661 - "extension_host: DebugAdapterBinary"
Cohesion: 0.47
Nodes (3): DebugAdapterBinary, latest::DebugAdapterBinary, extension::DebugAdapterBinary

### Community 1662 - "feature_flags: DemoFlag"
Cohesion: 0.27
Nodes (4): DemoFlag, .NAME, IntensityFlag, .NAME

### Community 1663 - "fuzzy_nucleo: Case"
Cohesion: 0.40
Nodes (3): Case, Ignore, Smart

### Community 1664 - "gpui: RingOperation"
Cohesion: 0.33
Nodes (6): RingOperation, Collect, NewCollector, Pin, Publish, Unpin

### Community 1665 - "gpui: seeds.rs"
Cohesion: 0.53
Nodes (3): calculate_seeds(), collect(), seeds()

### Community 1666 - "gpui_web: gallery.js"
Cohesion: 0.33
Nodes (4): back, example, gallery, status

### Community 1669 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, composite

### Community 1670 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, declaration

### Community 1671 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, emitBOM

### Community 1672 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, emitDeclarationOnly

### Community 1673 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, erasableSyntaxOnly

### Community 1674 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, exactOptionalPropertyTypes

### Community 1675 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, inlineSources

### Community 1676 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, isolatedModules

### Community 1677 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, jsxFactory

### Community 1678 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, jsxFragmentFactory

### Community 1679 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, jsxImportSource

### Community 1680 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, listFiles

### Community 1681 - "json_schema_store: anyOf"
Cohesion: 0.33
Nodes (6): anyOf, description, markdownDescription, type, x-intellij-html-description, module

### Community 1682 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, noEmit

### Community 1683 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, noEmitOnError

### Community 1684 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, noLib

### Community 1685 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, noResolve

### Community 1686 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, noStrictGenericChecks

### Community 1687 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, noUnusedLocals

### Community 1688 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, preserveConstEnums

### Community 1689 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, preserveSymlinks

### Community 1690 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, preserveValueImports

### Community 1691 - "json_schema_store: default"
Cohesion: 0.33
Nodes (6): default, description, markdownDescription, type, x-intellij-html-description, pretty

### Community 1692 - "json_schema_store: reactNamespace"
Cohesion: 0.33
Nodes (6): reactNamespace, default, description, markdownDescription, type, x-intellij-html-description

### Community 1693 - "json_schema_store: removeComments"
Cohesion: 0.33
Nodes (6): removeComments, default, description, markdownDescription, type, x-intellij-html-description

### Community 1694 - "json_schema_store: rewriteRelativeImportExtension"
Cohesion: 0.33
Nodes (6): rewriteRelativeImportExtensions, default, description, markdownDescription, type, x-intellij-html-description

### Community 1695 - "json_schema_store: skipDefaultLibCheck"
Cohesion: 0.33
Nodes (6): skipDefaultLibCheck, default, description, markdownDescription, type, x-intellij-html-description

### Community 1696 - "json_schema_store: skipLibCheck"
Cohesion: 0.33
Nodes (6): skipLibCheck, default, description, markdownDescription, type, x-intellij-html-description

### Community 1697 - "json_schema_store: sourceMap"
Cohesion: 0.33
Nodes (6): sourceMap, default, description, markdownDescription, type, x-intellij-html-description

### Community 1698 - "json_schema_store: suppressExcessPropertyErrors"
Cohesion: 0.33
Nodes (6): suppressExcessPropertyErrors, default, description, markdownDescription, type, x-intellij-html-description

### Community 1699 - "json_schema_store: suppressImplicitAnyIndexErrors"
Cohesion: 0.33
Nodes (6): suppressImplicitAnyIndexErrors, default, description, markdownDescription, type, x-intellij-html-description

### Community 1700 - "json_schema_store: tsBuildInfoFile"
Cohesion: 0.33
Nodes (6): tsBuildInfoFile, default, description, markdownDescription, type, x-intellij-html-description

### Community 1701 - "json_schema_store: useUnknownInCatchVariables"
Cohesion: 0.33
Nodes (6): useUnknownInCatchVariables, default, description, markdownDescription, type, x-intellij-html-description

### Community 1703 - "migrator: m_2025_01_30/settings.rs"
Cohesion: 0.60
Nodes (3): replace_tab_close_button_setting_key(), replace_tab_close_button_setting_value(), SETTINGS_PATTERNS

### Community 1704 - "migrator: m_2025_12_15/settings.rs"
Cohesion: 0.67
Nodes (3): is_restore_on_startup_setting(), rename_restore_on_startup_values(), SETTINGS_PATTERNS

### Community 1705 - "proto: buf.yaml"
Cohesion: 0.33
Nodes (5): breaking, use, lint, except, version

### Community 1706 - "proto: Item"
Cohesion: 0.33
Nodes (3): split_repository_update(), split_worktree_update(), test_split_repository_update_keeps_branch_list_on_final_chunk()

### Community 1707 - "settings_content: ExtensionCapabilityContent"
Cohesion: 0.33
Nodes (5): ExtensionCapabilityContent, DownloadFile, NpmInstallPackage, ProcessExec, ExtensionSettingsContent

### Community 1708 - "settings_content: flatten_collision_probe.rs"
Cohesion: 0.40
Nodes (3): check(), KNOWN_SHADOWED_KEYS, no_key_collisions_between_named_fields_and_flattened_sections()

### Community 1711 - "terminal_view: print256color.sh"
Cohesion: 0.67
Nodes (5): contrast_colour(), print_blocks(), print_colour(), print_run(), print256color.sh script

### Community 1712 - "ui: AnimationDuration"
Cohesion: 0.40
Nodes (4): AnimationDuration, Fast, Instant, Slow

### Community 1715 - "x_ai_subscribed: refresh_token()"
Cohesion: 0.47
Nodes (4): refresh_token(), RefreshError, Fatal, Transient

### Community 1718 - "AGENTS.md: Code and Settings"
Cohesion: 0.33
Nodes (6): Code and Settings, Formatting Conventions, Keybindings, Tables, Terminal Commands, Tips and Notes

### Community 1719 - "src: Agent Path Boundaries {#agent-"
Cohesion: 0.33
Nodes (6): Agent Path Boundaries {#agent-path-boundaries}, Agent Profiles, Built-in Profiles {#built-in-profiles}, Configure Profiles {#configure-profiles}, Profiles and Settings {#settings}, Profiles vs. Tool Permissions {#profiles-vs-tool-permissions}

### Community 1720 - "src: Instruction File Support {#sup"
Cohesion: 0.33
Nodes (6): Instruction File Support {#support}, Instructions, Instructions vs. Skills {#instructions-vs-skills}, Migrating from Rules {#migrating-from-rules}, Personal Instructions {#personal-instructions}, Project Instructions {#project-instructions}

### Community 1721 - "src: AI Data Retained by Zed {#ai-d"
Cohesion: 0.33
Nodes (6): AI Data Retained by Zed {#ai-data-retained-by-zed}, AI Privacy, AI Request Paths {#ai-request-paths}, Controls and Related Privacy Docs {#controls-and-related-privacy-docs}, Provider Safety Retention for Designated Models {#provider-safety-retention}, Zed-Hosted Model Commitments {#data-retention-and-training}

### Community 1722 - "src: Authenticate with Zed"
Cohesion: 0.33
Nodes (6): Authenticate with Zed, Email Addresses {#email}, Hiding Sign In button from the interface, Signing In, Signing Out, What Features Require Signing In?

### Community 1723 - "src: ansible.md"
Cohesion: 0.33
Nodes (5): Ansible, File detection, Inventory, LSP Configuration, Setup

### Community 1724 - "src: Diagnostics"
Cohesion: 0.33
Nodes (6): Diagnostics, Editing in a multibuffer, Find References, Multibuffers, Navigating to the Source File, Project search

### Community 1725 - "src: Find All References"
Cohesion: 0.33
Nodes (6): Find All References, Navigation {#navigation}, Outline Panel, Project Diagnostics, Project Search Results, Usage with multibuffers

### Community 1726 - "src: Agent Panel {#agent-panel}"
Cohesion: 0.33
Nodes (6): Agent Panel {#agent-panel}, Default Width {#agent-panel-default-width}, Flexible Sizing {#agent-panel-flexible-sizing}, Threads Sidebar Auto Open {#agent-threads-sidebar-auto-open}, Threads Sidebar Default Width {#agent-threads-sidebar-default-width}, Threads Sidebar Position {#agent-threads-sidebar-position}

### Community 1727 - "src: Completions"
Cohesion: 0.33
Nodes (6): Completions, LSP, LSP Fetch Timeout (ms), LSP Insert Mode, Min Words Query Length, Words

### Community 1728 - "actionlint.yml: actionlint.yml"
Cohesion: 0.33
Nodes (5): ignore, paths, .github/workflows/{ci,release_nightly}.yml, self-hosted-runner, labels

### Community 1729 - "ISSUE_TEMPLATE: config.yml"
Cohesion: 0.33
Nodes (5): blank_issues_enabled, contact_links, about, name, url

### Community 1730 - "workflows: next_version"
Cohesion: 0.33
Nodes (6): next_version, pr_branch, preview_branch, preview_tag, stable_branch, outputs

### Community 1731 - "workflows: defaults"
Cohesion: 0.33
Nodes (6): defaults, if, needs, runs-on, timeout-minutes, bump_extension_version

### Community 1732 - "workflows: RUST_LIB_BACKTRACE"
Cohesion: 0.33
Nodes (6): RUST_LIB_BACKTRACE, run_tests_mac, env, if, runs-on, timeout-minutes

### Community 1733 - "workflows: continue-on-error"
Cohesion: 0.33
Nodes (6): continue-on-error, if, needs, runs-on, timeout-minutes, build_nix_linux_x86_64

### Community 1734 - "workflows: if"
Cohesion: 0.33
Nodes (6): if, outputs, runs-on, timeout-minutes, check_nightly_tag, skip

### Community 1735 - "workflows: on"
Cohesion: 0.33
Nodes (6): on, merge_group, pull_request, push, branches, branches

### Community 1736 - "terms.md: 13.1. Definition"
Cohesion: 0.33
Nodes (6): 13.1. Definition, 13.2. Obligations, 13.3. Exclusions, 13.4. Remedies, 13.5. Required Disclosures, 13. Confidentiality

### Community 1739 - "install.sh: install.sh"
Cohesion: 0.53
Nodes (5): curl(), macos(), main(), install.sh script, linux()

### Community 1742 - "settings: tab_bar"
Cohesion: 0.40
Nodes (5): tab_bar, show, show_nav_history_buttons, show_pinned_tabs_in_separate_row, show_tab_bar_buttons

### Community 1743 - "themes: ayu.json"
Cohesion: 0.40
Nodes (4): author, name, $schema, themes

### Community 1744 - "themes: gruvbox.json"
Cohesion: 0.40
Nodes (4): author, name, $schema, themes

### Community 1745 - "themes: one.json"
Cohesion: 0.40
Nodes (4): author, name, $schema, themes

### Community 1746 - "project: CacheInlayHintsTask"
Cohesion: 0.40
Nodes (3): ChunkFetch, Cached, Running

### Community 1748 - "agent: AskUserToolOutput"
Cohesion: 0.50
Nodes (4): AskUserToolOutput, Answered, Error, LanguageModelToolResultContent

### Community 1749 - "agent: CreateThreadToolOutput"
Cohesion: 0.50
Nodes (4): CreateThreadToolOutput, Error, Success, LanguageModelToolResultContent

### Community 1750 - "agent: Advanced Usage"
Cohesion: 0.40
Nodes (5): Advanced Usage, Low-Level Server, MCP Primitives, Server Capabilities, Writing MCP Clients

### Community 1751 - "agent: Claude Desktop Integration"
Cohesion: 0.40
Nodes (5): Claude Desktop Integration, Development Mode, Direct Execution, Mounting to an Existing ASGI Server, Running Your Server

### Community 1752 - "agent: FindPathToolOutput"
Cohesion: 0.50
Nodes (4): FindPathToolOutput, Error, Success, LanguageModelToolResultContent

### Community 1753 - "collab: buffer_operation.rs"
Cohesion: 0.50
Nodes (3): Model, Relation, Buffer

### Community 1754 - "collab: buffer_snapshot.rs"
Cohesion: 0.50
Nodes (3): Model, Relation, Buffer

### Community 1755 - "collab: tables/extension.rs"
Cohesion: 0.50
Nodes (3): Model, Relation, LatestVersion

### Community 1756 - "collab: language_server.rs"
Cohesion: 0.50
Nodes (3): Model, Relation, Project

### Community 1757 - "collab: project_repository.rs"
Cohesion: 0.50
Nodes (3): Model, Relation, Project

### Community 1758 - "collab: tables/worktree.rs"
Cohesion: 0.50
Nodes (3): Model, Relation, Project

### Community 1759 - "dap: DapStatus"
Cohesion: 0.40
Nodes (5): DapStatus, CheckingForUpdate, Downloading, Failed, None

### Community 1760 - "edit_prediction_cli: qa.md"
Cohesion: 0.40
Nodes (4): Current File, Edit History (chronological), Evaluate, Predicted Next Edit

### Community 1763 - "editor: ActionFetchReady"
Cohesion: 0.40
Nodes (5): ActionFetchReady, CodeActionsForSelection, Fetching, None, Ready

### Community 1764 - "editor: ContextMenuOptions"
Cohesion: 0.40
Nodes (4): ContextMenuOptions, ContextMenuPlacement, Above, Below

### Community 1771 - "google_ai: Task"
Cohesion: 0.40
Nodes (5): Task, BatchEmbedContents, EmbedContent, GenerateContent, StreamGenerateContent

### Community 1776 - "icons: icons/README.md"
Cohesion: 0.40
Nodes (4): Contributing, Guidelines, Sourcing, Zed Icons

### Community 1777 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, allowImportingTsExtensions

### Community 1778 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, charset

### Community 1779 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, declarationDir

### Community 1780 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, diagnostics

### Community 1781 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, disableReferencedProjectLoad

### Community 1782 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, emitDecoratorMetadata

### Community 1783 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, experimentalDecorators

### Community 1784 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, mapRoot

### Community 1785 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, noImplicitAny

### Community 1786 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, noImplicitThis

### Community 1787 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, noPropertyAccessFromIndexSignature

### Community 1788 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, outDir

### Community 1789 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, outFile

### Community 1790 - "json_schema_store: description"
Cohesion: 0.40
Nodes (5): description, markdownDescription, type, x-intellij-html-description, preserveWatchOutput

### Community 1791 - "json_schema_store: rootDir"
Cohesion: 0.40
Nodes (5): rootDir, description, markdownDescription, type, x-intellij-html-description

### Community 1792 - "json_schema_store: sourceRoot"
Cohesion: 0.40
Nodes (5): sourceRoot, description, markdownDescription, type, x-intellij-html-description

### Community 1793 - "json_schema_store: stripInternal"
Cohesion: 0.40
Nodes (5): stripInternal, description, markdownDescription, type, x-intellij-html-description

### Community 1794 - "language_models_cloud: AnthropicModelMode"
Cohesion: 0.50
Nodes (4): AnthropicModelMode, ModelMode, Default, Thinking

### Community 1796 - "project_panel: cmp_worktree_entries()"
Cohesion: 0.80
Nodes (3): cmp_worktree_entries(), par_sort_worktree_entries(), sort_worktree_entries()

### Community 1798 - "proto: proto/src/macros.rs"
Cohesion: 0.40
Nodes (4): entity_messages!, lsp_messages!, messages!, request_messages!

### Community 1799 - "search: InputPanel"
Cohesion: 0.40
Nodes (5): InputPanel, Exclude, Include, Query, Replacement

### Community 1801 - "terminal_view: terminal_view/README.md"
Cohesion: 0.40
Nodes (4): Backend Boundary, Design Notes, Input, Terminal View

### Community 1803 - "ui: .spacing()"
Cohesion: 0.40
Nodes (4): ListItemSpacing, Dense, ExtraDense, Sparse

### Community 1804 - "AGENTS.md: Bad: Defensive or dismissive"
Cohesion: 0.40
Nodes (5): Bad: Defensive or dismissive, Bad: Wordy and promotional, Examples, Good: Direct and actionable, Good: Honest about limitations

### Community 1805 - "AGENTS.md: Bad: Defensive or dismissive"
Cohesion: 0.40
Nodes (5): Bad: Defensive or dismissive, Bad: Wordy and promotional, Examples of Good Copy, Good: Direct and actionable, Good: Honest about limitations

### Community 1806 - "src: Context Windows {#context-wind"
Cohesion: 0.40
Nodes (5): Context Windows {#context-windows}, Recent Model Retirements {#recent-model-retirements}, Tool Calls {#tool-calls}, Usage {#usage}, Zed-Hosted Models

### Community 1807 - "src: Anthropic-Compatible Providers"
Cohesion: 0.40
Nodes (5): Anthropic-Compatible Providers {#anthropic-api-compatible}, Choose a Model Access Path {#choose-a-model-access-path}, Edit Prediction {#edit-prediction}, LLM Providers, OpenAI-Compatible Providers {#openai-api-compatible}

### Community 1808 - "src: Agent Paths {#agent-paths}"
Cohesion: 0.40
Nodes (5): Agent Paths {#agent-paths}, AI, AI Features {#ai-features}, Configure AI {#configure-ai}, Model Access {#model-access}

### Community 1809 - "src: Additional admin controls"
Cohesion: 0.40
Nodes (5): Additional admin controls, How individual plans differ, Privacy for Business, What data still leaves the organization, What's enforced by default

### Community 1810 - "src: Configuring Icon Themes"
Cohesion: 0.40
Nodes (5): Configuring Icon Themes, Icon Theme Development, Icon Themes, Installing more Icon Themes, Selecting an Icon Theme

### Community 1811 - "src: ocaml.md"
Cohesion: 0.40
Nodes (4): Launching Zed, OCaml, Setup Instructions, Using Opam

### Community 1812 - "src: Dark"
Cohesion: 0.40
Nodes (5): Dark, Icon Theme, Icon Theme Object, Light, Mode

### Community 1813 - "src: Dark"
Cohesion: 0.40
Nodes (5): Dark, Light, Mode, Theme, Theme Object

### Community 1814 - "src: Editor Tab Bar"
Cohesion: 0.40
Nodes (5): Editor Tab Bar, Navigation History Buttons, Pinned Tabs in a Separate Row {#tab-bar-pinned-tabs}, Show, Tab Bar Buttons

### Community 1815 - "src: Fast Scroll Sensitivity"
Cohesion: 0.40
Nodes (5): Fast Scroll Sensitivity, Horizontal Scroll Margin, Mouse Wheel Zoom, Scroll Sensitivity, Vertical Scroll Margin

### Community 1816 - "src: Terminal: Button"
Cohesion: 0.40
Nodes (5): Terminal: Button, Terminal: Path Hyperlink Regexes, Terminal: Path Hyperlink Timeout (ms), Terminal: Toolbar, Terminal: Working Directory

### Community 1817 - "src: update.md"
Cohesion: 0.40
Nodes (4): Auto-updates, How to check your current version, How to control update behavior, Update Zed

### Community 1819 - "workflows: needs"
Cohesion: 0.40
Nodes (5): needs, runs-on, steps, timeout-minutes, bundle_linux_x86_64

### Community 1820 - "workflows: R2_ACCESS_KEY_ID"
Cohesion: 0.40
Nodes (5): R2_ACCESS_KEY_ID, R2_ACCOUNT_ID, R2_SECRET_ACCESS_KEY, SCCACHE_BUCKET, env

### Community 1821 - "workflows: if"
Cohesion: 0.40
Nodes (5): if, needs, runs-on, timeout-minutes, clippy_mac

### Community 1822 - "workflows: if"
Cohesion: 0.40
Nodes (5): if, needs, runs-on, timeout-minutes, clippy_mac_x86_64

### Community 1823 - "workflows: if"
Cohesion: 0.40
Nodes (5): if, needs, runs-on, timeout-minutes, clippy_windows

### Community 1824 - "workflows: run_tests_windows"
Cohesion: 0.40
Nodes (5): run_tests_windows, if, needs, runs-on, timeout-minutes

### Community 1825 - "terms.md: 2.1. Eligibility"
Cohesion: 0.40
Nodes (5): 2.1. Eligibility, 2.2. Access Grant, 2.3. Acceptable Use, 2.4. Restrictions, 2. Service

### Community 1826 - "terms.md: 4.1. Zed's Use of Customer Dat"
Cohesion: 0.40
Nodes (5): 4.1. Zed's Use of Customer Data, 4.2. Customer's Ownership of Output, 4.3. Zed's Collection of Output Rating, 4.4. Telemetry, 4. Data

### Community 1827 - "terms.md: 9.1. Term"
Cohesion: 0.40
Nodes (5): 9.1. Term, 9.2. Termination, 9.3. Effect of Termination, 9.4. Modification of the Service, 9. Term, Termination, and Modification of the Service

### Community 1829 - "extension_api: zed_extension_api"
Cohesion: 0.40
Nodes (5): zed_extension_api, zed_glsl, zed_html, zed_proto, zed_test_extension

### Community 1831 - "lib: blob-store.sh script"
Cohesion: 0.60
Nodes (4): blob-store.sh script, upload_to_blob_store(), upload_to_blob_store_public(), upload_to_blob_store_with_acl()

### Community 1834 - "settings: collaboration_panel"
Cohesion: 0.50
Nodes (4): collaboration_panel, button, default_width, dock

### Community 1835 - "settings: telemetry"
Cohesion: 0.50
Nodes (4): telemetry, anthropic_retention, diagnostics, metrics

### Community 1836 - "settings: theme"
Cohesion: 0.50
Nodes (4): theme, dark, light, mode

### Community 1837 - "action_log: TrackedBufferStatus"
Cohesion: 0.50
Nodes (4): TrackedBufferStatus, Created, Deleted, Modified

### Community 1838 - "agent: TrackedBufferStatus"
Cohesion: 0.50
Nodes (4): TrackedBufferStatus, Created, Deleted, Modified

### Community 1839 - "agent: RasterizationOptions"
Cohesion: 0.50
Nodes (4): RasterizationOptions, Bilevel, GrayscaleAa, SubpixelAa

### Community 1840 - "agent: RasterizationOptions"
Cohesion: 0.50
Nodes (4): RasterizationOptions, Bilevel, GrayscaleAa, SubpixelAa

### Community 1841 - "agent: Long Requests"
Cohesion: 0.50
Nodes (4): Long Requests, Request IDs, Retries, Timeouts

### Community 1867 - "dap: IoKind"
Cohesion: 0.50
Nodes (4): IoKind, StdErr, StdIn, StdOut

### Community 1868 - "deepseek: ReasoningEffort"
Cohesion: 0.50
Nodes (4): ReasoningEffort, High, Low, Max

### Community 1869 - "deepseek: Thinking"
Cohesion: 0.50
Nodes (4): Thinking, ThinkingType, Disabled, Enabled

### Community 1870 - "edit_prediction_cli: codex-acp--add-derive.md"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1871 - "edit_prediction_cli: flask--add-and-rename-test-fun"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1872 - "edit_prediction_cli: flask--add-import-statement.md"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1873 - "edit_prediction_cli: flask--add-test-function.md"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1874 - "edit_prediction_cli: flask--rename-accepted-predict"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1875 - "edit_prediction_cli: hello-world--rename-accepted-g"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1876 - "edit_prediction_cli: terraform--add-comment.md"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1877 - "edit_prediction_cli: tree-sitter--if-let-to-match.m"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1878 - "edit_prediction_cli: tree-sitter--tuple-to-struct-d"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1879 - "edit_prediction_cli: tree-sitter--tuple-to-struct-d"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1880 - "edit_prediction_cli: tree-sitter--tuple-to-struct-f"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1881 - "edit_prediction_cli: tree-sitter--tuple-to-struct-f"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1882 - "edit_prediction_cli: tree-sitter--tuple-to-struct-l"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1883 - "edit_prediction_cli: vscode--add-async-and-await.md"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1884 - "edit_prediction_cli: vscode--add-class-decorator.md"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1885 - "edit_prediction_cli: vscode--add-interface-method.m"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1886 - "edit_prediction_cli: vscode--log-object-property.md"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1887 - "edit_prediction_cli: zed--add-eprintln.md"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1888 - "edit_prediction_cli: zed--change-match-arm.md"
Cohesion: 0.50
Nodes (3): Cursor Position, Edit History, Expected Patch

### Community 1889 - "edit_prediction_types: EditPredictionGranularity"
Cohesion: 0.50
Nodes (4): EditPredictionGranularity, Full, Line, Word

### Community 1890 - "editor: BreakpointPromptEditAction"
Cohesion: 0.50
Nodes (4): BreakpointPromptEditAction, Condition, HitCondition, Log

### Community 1891 - "editor: SelectSyntaxNodeScrollBehavior"
Cohesion: 0.50
Nodes (4): SelectSyntaxNodeScrollBehavior, CursorBottom, CursorTop, FitSelection

### Community 1892 - "extension_api: PENDING_CHANGES.md"
Cohesion: 0.50
Nodes (3): Pending Changes, Slash Commands, vNext

### Community 1895 - "feature_flags: AgentThreadWorktreeLabel"
Cohesion: 0.50
Nodes (4): AgentThreadWorktreeLabel, Both, Branch, Worktree

### Community 1901 - "rpc: rpc/src/macros.rs"
Cohesion: 0.50
Nodes (3): entity_messages!, messages!, request_messages!

### Community 1902 - "search: Event"
Cohesion: 0.50
Nodes (3): Event, Dismissed, UpdateLocation

### Community 1903 - "search: ViewEvent"
Cohesion: 0.50
Nodes (4): ViewEvent, Activate, Dismiss, UpdateTab

### Community 1905 - "sidebar: SerializedSidebar"
Cohesion: 0.50
Nodes (4): SerializedSidebar, SerializedSidebarView, History, ThreadList

### Community 1906 - "ui: HoverTarget"
Cohesion: 0.50
Nodes (4): HoverTarget, MainMenu, None, Submenu

### Community 1908 - "AGENTS.md: Behavioral Guidelines"
Cohesion: 0.50
Nodes (4): Behavioral Guidelines, Conservative by Default, Incremental Updates, Traceability

### Community 1909 - "AGENTS.md: Keymap File Locations"
Cohesion: 0.50
Nodes (4): Keymap File Locations, Recognized Rules Files, Settings File Locations, Zed-Specific Conventions

### Community 1910 - "AGENTS.md: Page Structure Patterns"
Cohesion: 0.50
Nodes (4): Page Structure Patterns, Provider/Feature Documentation Pattern, Settings Documentation Pattern, Standard Page Layout

### Community 1911 - "src: Audio Settings {#audio-setting"
Cohesion: 0.50
Nodes (4): Audio Settings {#audio-settings}, Collaboration {#collaboration}, Collaboration Panel {#collaboration-panel}, Selecting Audio Devices

### Community 1912 - "src: Editing Code"
Cohesion: 0.50
Nodes (4): Editing Code, How These Features Work Together, Related Features, What's in This Section

### Community 1913 - "src: Anthropic Retention {#telemetr"
Cohesion: 0.50
Nodes (4): Anthropic Retention {#telemetry-anthropic-retention}, Diagnostics, Metrics, Telemetry

### Community 1914 - "src: Example configuration"
Cohesion: 0.50
Nodes (4): Example configuration, Known Limitations, Scopes, Snippets

### Community 1915 - "src: Action Reference"
Cohesion: 0.50
Nodes (4): Action Reference, Opening the Tab Switcher, Quick Switching, Tab Switcher

### Community 1916 - "src: Adding toolchains manually"
Cohesion: 0.50
Nodes (4): Adding toolchains manually, Selecting toolchains, Toolchains, Why do we need toolchains?

### Community 1917 - "src: Configuring broad worktree tru"
Cohesion: 0.50
Nodes (4): Configuring broad worktree trust, Trust hierarchy, What is restricted, Zed and trusted worktrees

### Community 1918 - "EXTRACTION.md: EXTRACTION.md"
Cohesion: 0.50
Nodes (3): Extracting an extension to dedicated repo, Pre-requisites, Process

### Community 1919 - "README.md: extensions/README.md"
Cohesion: 0.50
Nodes (3): Dev Extensions, Structure, Zed Extensions

### Community 1920 - "workflows: if"
Cohesion: 0.50
Nodes (4): if, needs, runs-on, commit_changes

### Community 1921 - "workflows: if"
Cohesion: 0.50
Nodes (4): if, needs, runs-on, create_preview_branch

### Community 1922 - "workflows: description"
Cohesion: 0.50
Nodes (4): description, required, type, branch

### Community 1923 - "workflows: pr_number"
Cohesion: 0.50
Nodes (4): pr_number, description, required, type

### Community 1924 - "workflows: name"
Cohesion: 0.50
Nodes (4): name, needs, runs-on, deploy

### Community 1925 - "workflows: needs"
Cohesion: 0.50
Nodes (4): needs, runs-on, timeout-minutes, build_static_bwrap_linux_x86_64

### Community 1926 - "workflows: needs"
Cohesion: 0.50
Nodes (4): needs, runs-on, steps, bundle_mac_aarch64

### Community 1927 - "workflows: if"
Cohesion: 0.50
Nodes (4): if, runs-on, timeout-minutes, check_scripts

### Community 1928 - "workflows: if"
Cohesion: 0.50
Nodes (4): if, runs-on, timeout-minutes, create_draft_release

### Community 1929 - "workflows: if"
Cohesion: 0.50
Nodes (4): if, runs-on, timeout-minutes, bundle_linux_aarch64

### Community 1930 - "workflows: if"
Cohesion: 0.50
Nodes (4): if, runs-on, timeout-minutes, bundle_linux_x86_64

### Community 1931 - "workflows: if"
Cohesion: 0.50
Nodes (4): if, runs-on, timeout-minutes, bundle_mac_x86_64

### Community 1932 - "workflows: if"
Cohesion: 0.50
Nodes (4): if, runs-on, timeout-minutes, bundle_windows_aarch64

### Community 1933 - "terms.md: 11.1."
Cohesion: 0.50
Nodes (4): 11.1., 11.2., 11.3., 11. Limitations of Liability

### Community 1934 - "terms.md: 15.1. Governing Law, Jurisdict"
Cohesion: 0.50
Nodes (4): 15.1. Governing Law, Jurisdiction and Venue, 15.2. Dispute Resolution and Arbitration, 15.3. Waiver of Jury Trial and Class Action Waiver, 15. Governing Law, Dispute Resolution and Arbitration

### Community 1935 - "tests: sandboxing/README.md"
Cohesion: 0.50
Nodes (3): CI, Experimental `sandbox_linux`, Linux sandbox VM tests

### Community 1940 - "BRAND.md: BRAND.md"
Cohesion: 0.50
Nodes (3): Icon, Palette, VEGA brand

### Community 1944 - "settings: active_pane_modifiers"
Cohesion: 0.67
Nodes (3): active_pane_modifiers, border_size, inactive_opacity

### Community 1945 - "settings: audio"
Cohesion: 0.67
Nodes (3): audio, experimental.input_audio_device, experimental.output_audio_device

### Community 1946 - "settings: calls"
Cohesion: 0.67
Nodes (3): calls, mute_on_join, share_on_join

### Community 1947 - "settings: centered_layout"
Cohesion: 0.67
Nodes (3): centered_layout, left_padding, right_padding

### Community 1948 - "settings: drag_and_drop_selection"
Cohesion: 0.67
Nodes (3): drag_and_drop_selection, delay, enabled

### Community 1949 - "settings: focus_follows_mouse"
Cohesion: 0.67
Nodes (3): focus_follows_mouse, debounce_ms, enabled

### Community 1952 - "action_log: UndoBufferStatus"
Cohesion: 0.67
Nodes (3): UndoBufferStatus, Created, Modified

### Community 1954 - "agent: Creating a batch"
Cohesion: 0.67
Nodes (3): Creating a batch, Getting results from a batch, Message Batches

### Community 1955 - "agent: Echo Server"
Cohesion: 0.67
Nodes (3): Echo Server, Examples, SQLite Explorer

### Community 1962 - "dev_container: ComposeUpBehavior"
Cohesion: 0.67
Nodes (3): ComposeUpBehavior, Create, Resume

### Community 1963 - "editor: FoldStatus"
Cohesion: 0.67
Nodes (3): FoldStatus, Foldable, Folded

### Community 2007 - "feature_flags: Intensity"
Cohesion: 0.67
Nodes (3): Intensity, High, Low

### Community 2015 - "outline_panel: CachedEntriesUpdate"
Cohesion: 0.67
Nodes (3): CachedEntriesUpdate, Contents, Entries

### Community 2016 - "outline_panel: ItemsDisplayMode"
Cohesion: 0.67
Nodes (3): ItemsDisplayMode, Outline, Search

### Community 2023 - "ui: SubmenuOpenTrigger"
Cohesion: 0.67
Nodes (3): SubmenuOpenTrigger, Keyboard, Pointer

### Community 2029 - "AGENTS.md: Core Principles"
Cohesion: 0.67
Nodes (3): Core Principles, Voice and Tone, What to Avoid

### Community 2030 - "AGENTS.md: Must"
Cohesion: 0.67
Nodes (3): Must, Must Not, Safety Constraints

### Community 2031 - "AGENTS.md: Output Format"
Cohesion: 0.67
Nodes (3): Output Format, Phase 4 Documentation Plan, Phase 6 Summary Format

### Community 2032 - "src: Agent Path vs. LLM Provider {#"
Cohesion: 0.67
Nodes (3): Agent Path vs. LLM Provider {#agent-path-vs-llm-provider}, Agents, Thread Types {#thread-types}

### Community 2033 - "src: Migrating to Skills {#migratin"
Cohesion: 0.67
Nodes (3): Migrating to Skills {#migrating-to-skills}, `.rules` Files {#rules-files}, Rules {#rules}

### Community 2034 - "src: How It Differs from Other Agen"
Cohesion: 0.67
Nodes (3): How It Differs from Other Agent Paths {#other-agent-paths}, What Zed Agent Uses {#what-zed-agent-uses}, Zed Agent

### Community 2036 - "src: Getting started"
Cohesion: 0.67
Nodes (3): Getting started, What's included, Zed Business

### Community 2037 - "src: Adding a Contact"
Cohesion: 0.67
Nodes (3): Adding a Contact, Contacts and Private Calls {#contacts}, Private Calls

### Community 2038 - "src: Completions"
Cohesion: 0.67
Nodes (3): Completions, Edit Predictions {#edit-predictions}, Language Server Code Completions {#code-completions}

### Community 2041 - "src: Active Pane Modifiers"
Cohesion: 0.67
Nodes (3): Active Pane Modifiers, Border size, Inactive Opacity

### Community 2042 - "src: Agent"
Cohesion: 0.67
Nodes (3): Agent, Prevent Idle Sleep, Subagent Model {#agent-subagent-model}

### Community 2043 - "src: Debounce"
Cohesion: 0.67
Nodes (3): Debounce, Enabled, Focus Follows Mouse

### Community 2044 - "src: Diagnostics"
Cohesion: 0.67
Nodes (3): Diagnostics, Inline Diagnostics, LSP Pull Diagnostics {#lsp-pull-diagnostics}

### Community 2045 - "src: Hour Format"
Cohesion: 0.67
Nodes (3): Hour Format, Journal, Path

### Community 2046 - "src: Quick start"
Cohesion: 0.67
Nodes (3): Quick start, Running & Testing, What's here

### Community 2047 - "workflows: defaults"
Cohesion: 0.67
Nodes (3): defaults, run, shell

### Community 2048 - "workflows: DIGITALOCEAN_SPACES_ACCESS_KEY"
Cohesion: 0.67
Nodes (3): DIGITALOCEAN_SPACES_ACCESS_KEY, DIGITALOCEAN_SPACES_SECRET_KEY, env

## Knowledge Gaps
- **6891 isolated node(s):** `config:recommended`, `:semanticCommitsDisabled`, `:separateMultipleMajorReleases`, `:dependencyDashboardApproval`, `helpers:pinGitHubActionDigests` (+6886 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 27413 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **449 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `TestAppContext` connect `Editor Commands & Context` to `UI Actions & Workspace Glue`, `Project & Agent Server Core`, `LLM Provider Integrations`, `Multi-Buffer & Diff Hunks`, `UI Notices & Components`, `GPUI Layout & Examples`, `json_schema_store: borrowappcontext_as`, `GPUI Style Primitives`, `GPUI Web & Metal Platform`, `Apple Platform FFI`, `Editor Element Geometry`, `Language Servers & Toolchains`, `GPUI Window & App Context`, `editor: DiagnosticRenderer`, `Language Buffer Core`, `Text Shaping & CoreText`, `GPUI Scroll & Input Ops`, `Tasks & JSON Schemas`, `Git Proto Messages`, `Shared Settings Types`, `remote: MessageId`, `agent_ui: .emit_load_error()`, `agent: AssertionFn`, `Markdown Rendering`, `Code Action Requests`, `settings_profile_selector: active_settings_profile_picker`, `Screen Capture & Wgpu`, `GPUI Lists & Minimap`, `Agent Settings & Compaction`, `project: resolve_git_worktree_to_main_r`, `agent: decide_permission_for_paths()`, `Language Completions & Deploy`, `Collab & Debugger Tests`, `ACP Thread Content`, `Git Repository Backend`, `Pane & Navigation History`, `ACP Thread Messages`, `watch: Changed`, `extensions_ui: RELOAD_DEBOUNCE_DURATION`, `Agent Skills & Terminal`, `Agent Tools & Extension API`, `Remote Server Protocol`, `Agent Conversation View`, `agent: path_matches_always_confirm()`, `Agent Panel`, `agent_ui: all_models()`, `edit_prediction_context: edit_prediction_context_tests.`, `Sidebar Tests`, `Git Commit UI`, `Completion Menu`, `ACP Thread Lifecycle`, `project: lsp_log_view_tests.rs`, `App Entry & Actions`, `lsp_locations: active_picker()`, `Project Git Tests`, `project: create_remote_shell()`, `search: buffer_search_query()`, `GPUI App Actions`, `GPUI Styled Methods`, `Multi-Workspace Windows`, `GPUI Frame Profiler`, `Edit Prediction Core`, `openai_subscribed: make_fresh_credentials()`, `project: context_server_store`, `agent: DeletePathTool`, `agent_ui: AvailableCommandInput`, `language: ClearHistory`, `lsp: run_work_cycle()`, `title_bar: .get_menus()`, `git_ui: Close`, `ui: gpui::AnimationElement<T>`, `debugger_ui: CompletionItemType`, `agent_ui: copythreadtoclipboard`, `keymap_editor: CopyAction`, `agent: agentmodelgroupname`, `language_model: fake_provider.rs`, `repl: addcodeblock`, `gpui_linux: asfd`, `markdown_preview: bufferdiff`, `agent: ActionLog`, `remote: .global()`, `project_panel: project_panel_tests.rs`, `outline: assert_single_caret_at_row()`, `gpui: backspace`, `picker: picker.rs`, `terminal_view: terminal_view.rs`, `project: CompletionListItemDefaults`, `terminal_view: activatenextpane`, `ui: axis_as_scrollbaraxis`, `agent_ui: completion_menu_max_width`, `language: Buffer`, `language_models: available_model_to_google_mode`, `agent_ui: .is_retained_thread()`, `project: LazyGlobSet`, `terminal_view: .to_run()`, `tabular_data_preview: ClearSelection`, `collab: AnyConnection`, `terminal: cursor_blinking()`, `client: CrashInfo`, `zed: IncomingCall`, `acp_thread: mention.rs`, `agent_servers: init_test()`, `project: compiled_selector()`, `git: BlameBufferResponse`, `remote_server: check_pid_file()`, `settings_ui: agent_skills_settings_path`, `language_model: image_format_from_external_con`, `git_ui: CheckoutBranch`, `openai_subscribed: .new()`, `search: base_rem_size_in_px`, `project: .worktree_id()`, `snippet_provider: test_snippets()`, `agent_servers: CancelSessionNotification`, `gpui: .create_terminal()`, `Worktree Scanning`, `buffer_diff: Anchor`, `agent_skills: component_matches_ignore_ascii`, `terminal: .default()`, `agent_servers: acpbetafeatureflag`, `context_server: OAuthTokenProvider`, `debugger_ui: resolve_path()`, `edit_prediction_cli: line_start_offset()`, `agent_ui: build_terminal_prompt_editor()`, `go_to_line: UserCaretPosition`, `gpui: src/randomized_element_tree.rs`, `language: server_precedences()`, `outline_panel: active_outline_panel()`, `recent_projects: user_ssh_config_file()`, `ui: arrow_key_with_pointer_on_subm`, `workspace: SaveStrategy`, `file_finder: parse_file_search_query()`, `client: calculate_json_checksum()`, `dev_container: devcontainer_api.rs`, `terminal: Attr`, `inspector_ui: assert_inspector_watchers()`, `gpui: .set_input_handler()`, `agent_ui: anthropiceventreporter`, `repl: CellMetadata`, `project_bookmarks: bookmarkstore`, `acp_thread: .patch_preview()`, `language_selector: active_picker()`, `openai_subscribed: fake_token_response()`, `agent_ui: choicecard`, `recent_projects: remote_connections.rs`, `search: .new()`, `agent_servers: Response`, `action_log: clock`, `settings_sync: account.rs`, `agent_ui: AgentProfileContent`, `settings_ui: all_projects()`, `gpui: examples/testing.rs`, `editor: code_completion_tests.rs`, `git_ui: RestoreFile`, `which_key: text_for_keybinding_keystrokes`, `auto_update: autoupdateevent`, `agent_ui: AgentSessionConfigOptions`, `agent: agent/src/db.rs`, `agent: skill_tool.rs`, `language: src/buffer_tests.rs`, `project: integration/trusted_worktrees.`, `agent_ui: OutputTokensUpdate`, `client: client/src/llm_token.rs`, `dap: IpAddr`, `git_ui_core: build_buffer_diff()`, `theme_selector: init()`, `vim: vim/src/command.rs`, `agent_settings: agent_settings.rs`, `agent_ui: AgentPanelEvent`, `workspace: AppNotifications`, `x_ai_subscribed: do_oauth_flow()`, `extension: compilationconcurrency`, `agent_ui: agent_diff_renderer()`, `multi_buffer: multi_buffer_tests.rs`, `call_hierarchy: buffer_text_style`, `git_ui: commit_message_editor()`, `project: .as_ref()`, `agent: FindPathTool`, `edit_prediction: asrefstr`, `livekit_client: ClaimGrants`, `dev_container: DevContainerUp`, `git_ui: ActivateChangesTab`, `benchmarks: benches/edit_file_tool.rs`, `gpui: gpui/src/test.rs`, `agent_ui: agentmodelicon`, `project: integration/signature_help.rs`, `prompt_store: BuiltInPrompt`, `agent_ui: agentsessionlistrequest`, `agent: test_resolve_path()`, `eval_cli: agentconnection_as`, `collab: join_channel()`, `repl: init()`, `terminal: assert_wakeups_and_notifies()`, `editor: columnar_selection.rs`, `gpui: frame_cost()`, `tasks_ui: Context`, `acp_thread: .clear_completed_plan_entries(`, `languages: cpp.rs`, `git_ui: AnyEntity`, `lsp_command_selector: init_selector_test()`, `outline_panel: add_outline_panel()`, `project: SerializedBookmark`, `zed: CliRequest`, `repl: plain.rs`, `theme_selector: .focus_handle()`, `command_palette: CommandInterceptResult`, `copilot_chat: src/copilot_chat.rs`, `git_ui: CommitDetails`, `workspace: multi_workspace_tests.rs`, `gpui: AnimationExample`, `editor: ToggleBlockComments`, `repl: local_kernel_specifications()`, `acp_tools: AcpTools`, `recent_projects: AddToWorkspace`, `agent: default_context_lines()`, `project: concurrent_remote_askpass_requ`, `search: active_match_index`, `which_key: humanize_action_name()`, `tab_switcher: activatepreviousitem`, `workflows: evals.rs`, `markdown: empty_render_image_does_not_pa`, `rpc: FakeServer`, `git_ui: ButtonStates`, `gpui: AnyWindowHandle`, `gpui: key_dispatch.rs`, `openai_subscribed: Context`, `gpui_macos: caller_action_ids_do_not_colli`, `image_viewer: displayed_render_image()`, `diagnostics: diagnostics_tests.rs`, `agent: tools/edit_file_tool.rs`, `agent_ui: cycling_modes_preserves_choice`, `editor: test_click_on_parameter_inlay_`, `edit_prediction: ts_error_count.rs`, `extension_host: ExtensionIndex`, `agent_ui: build_assist_editor_renderer()`, `openai_subscribed: compact()`, `agent: templates.rs`, `editor: cancel_flycheck`, `git_ui_core: git_ui_core/src/notifications.`, `git_ui_core: DeleteWorktreeTooltip`, `repl: clearoutput`, `git_ui: CompareWithBranch`, `language_models: ClientTokenProvider`, `acp_thread: diff_change_label()`, `agent: create_directory_tool.rs`, `agent: init_test()`, `keymap_editor: keystroke_input.rs`, `terminal_view: alternatescroll`, `editor: .to_item_events()`, `editor: bookmark_path()`, `gpui: AtomicUsize`, `language_models: authenticate_does_not_fetch_mo`, `call: CallCanceled`, `edit_prediction: EditPredictionRejectReason`, `tabular_data_preview: EditorState`, `remote_connection: .ask_password()`, `vim: Direction`, `gpui: process_info.rs`, `language_models: add_message_content_part()`, `open_path_prompt: DirectoryState`, `agent_ui: agent_panel`, `gpui: input()`, `picker_preview: picker_preview.rs`, `project_symbols: init()`, `agent_servers: .drop()`, `editor: test_client_can_wrap_with_emme`, `git_ui: ButtonStates`, `x_ai_subscribed: src/x_ai_subscribed.rs`, `edit_prediction: childentriesoptions`, `edit_prediction: init_test()`, `settings_ui: AnthropicCompatibleAvailableMo`, `audio: apm`, `editor: test_setup()`, `gpui: hot_reload.rs`, `agent: deserialize_maybe_stringified(`, `agent_ui: Diff`, `fs: .record_watcher_diagnostics()`, `git_ui: stash.rs`, `language_models: capability_cells_update_when_a`, `copilot: Copilot`, `git_ui: copy_branch_name()`?**
  _High betweenness centrality (0.104) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `TestAppContext` (e.g. with `frame_cost()` and `wake_rechecks_native_visibility()`) actually correct?**
  _`TestAppContext` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `config:recommended`, `:semanticCommitsDisabled`, `:separateMultipleMajorReleases` to the rest of the system?**
  _6891 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `UI Actions & Workspace Glue` be split into smaller, more focused modules?**
  _Cohesion score 0.002768196498011732 - nodes in this community are weakly interconnected._
- **Why does `Task` connect `Remote Collaboration Auth` to `UI Actions & Workspace Glue`, `Project & Agent Server Core`, `LLM Provider Integrations`, `Editor Commands & Context`, `Multi-Buffer & Diff Hunks`, `Language Servers & Toolchains`, `GPUI Style Primitives`, `Apple Platform FFI`, `Editor Element Geometry`, `Wayland Platform Layer`, `Editor Text Transform Actions`, `GPUI Window & App Context`, `Language Buffer Core`, `GPUI Scroll & Input Ops`, `Tasks & JSON Schemas`, `Git Proto Messages`, `LSP Proto Requests`, `Shared Settings Types`, `Workspace Actions`, `Markdown Rendering`, `Agent Edit Predictions`, `Code Action Requests`, `Worktree Scanning`, `Screen Capture & Wgpu`, `Agent Settings & Compaction`, `Language Completions & Deploy`, `Pane & Navigation History`, `Collab & Debugger Tests`, `ACP Thread Content`, `Git Repository Backend`, `ACP Thread Messages`, `Agent Sidebar UI`, `Agent Skills & Terminal`, `Agent Tools & Extension API`, `Remote Server Protocol`, `Agent Conversation View`, `Project Panel`, `Agent Panel`, `Item Handles & Following`, `Git Commit UI`, `Completion Menu`, `ACP Thread Lifecycle`, `Project Git Tests`, `App Entry & Actions`, `Workspace Persistence`, `Agent Permission Requests`, `GPUI App Actions`, `Multi-Workspace Windows`, `AWS Bedrock Provider`, `Edit Prediction Core`, `project: ContinueResponse`, `agent_ui: AvailableCommandInput`, `language: ClearHistory`, `lsp: run_work_cycle()`, `agent: AuthorizationKind`, `vim: register()`, `git_ui: Close`, `agent: Addon`, `agent_ui: copythreadtoclipboard`, `keymap_editor: CopyAction`, `agent: agentmodelgroupname`, `language_model: fake_provider.rs`, `agent: bracketmatch`, `settings: asset_str`, `gpui_linux: asfd`, `markdown_preview: bufferdiff`, `remote: .global()`, `terminal_view: terminal_view.rs`, `picker: picker.rs`, `acp_thread: .can_set_title()`, `project: CompletionListItemDefaults`, `repl: clearcurrentoutput`, `terminal_view: activatenextpane`, `ui: axis_as_scrollbaraxis`, `agent_servers: agentsessioninfo`, `agent_ui: completion_menu_max_width`, `language: Buffer`, `agent_ui: .is_retained_thread()`, `terminal: cursor_blinking()`, `collab: AnyConnection`, `client: CrashInfo`, `zed: IncomingCall`, `terminal: AlacCell`, `workspace: Context`, `agent: .create_sibling_thread()`, `acp_thread: mention.rs`, `settings_ui: agent_skills_settings_path`, `git: BlameBufferResponse`, `search: base_rem_size_in_px`, `git_ui: CheckoutBranch`, `agent: .extend_key_context()`, `project: .worktree_id()`, `agent_servers: CancelSessionNotification`, `gpui: .create_terminal()`, `buffer_diff: Anchor`, `agent_servers: ConnectionTo`, `agent_servers: acpbetafeatureflag`, `debugger_ui: resolve_path()`, `agent_ui: build_terminal_prompt_editor()`, `fs: .directories()`, `recent_projects: user_ssh_config_file()`, `language_models_cloud: client_supports_status_message`, `editor: .get()`, `terminal: Attr`, `settings_ui: project_skills_relative_path()`, `agent_ui: anthropiceventreporter`, `repl: CellMetadata`, `search: .new()`, `outline_panel: entry_git_aware_label_color()`, `collab_ui: CollapseSelectedChannel`, `agent_servers: Response`, `call: call_impl/participant.rs`, `search: active_search_view()`, `settings_sync: account.rs`, `agent_ui: AgentProfileContent`, `settings_ui: all_projects()`, `project: BreakpointsForFile`, `extension_host: .new()`, `editor: assert_chunks_are_contiguous()`, `project: AllocateWorktreeId`, `remote: parse_shell()`, `reqwest_client: batches`, `auto_update: autoupdateevent`, `agent_ui: AgentSessionConfigOptions`, `agent: agent/src/db.rs`, `project: BufferReloaded`, `client: client/src/llm_token.rs`, `dap: IpAddr`, `debugger_ui: DebuggerProvider`, `vim: vim/src/command.rs`, `agent_ui: AgentPanelEvent`, `file_finder: FileFinder`, `agent_ui: agent_diff_renderer()`, `call_hierarchy: buffer_text_style`, `project: .as_ref()`, `fs: BlobReadGateState`, `extension: DebugAdapterRegistryProxy`, `git_ui: ActivateChangesTab`, `scheduler: scheduler/src/executor.rs`, `agent_ui: agentmodelicon`, `prompt_store: BuiltInPrompt`, `agent_ui: agentsessionlistrequest`, `agent: test_resolve_path()`, `language_core: .adapter_language_name()`, `editor: AcceptInlayHint`, `git_ui: commitsummary`, `editor: TabEdit`, `editor: assert_hovers()`, `repl: init()`, `agent: SandboxPermissions`, `cloud_llm_client: LanguageModelToolResultContent`, `tasks_ui: Context`, `livekit_client: audio_mixer`, `paths: paths/src/paths.rs`, `vim: ClipboardItem`, `command_palette: CommandInterceptResult`, `copilot_chat: src/copilot_chat.rs`, `git_ui: CommitDetails`, `gpui: .on_action()`, `gpui_macos: appleactivity`, `acp_tools: AcpTools`, `recent_projects: AddToWorkspace`, `project: ContextServer`, `search: active_match_index`, `which_key: humanize_action_name()`, `editor: accepteditprediction`, `agent_ui: ArchiveListItem`, `component_preview: ActivePageId`, `workspace: .supported_options()`, `dev_container: DevContainerMessage`, `markdown: empty_render_image_does_not_pa`, `language_model: authenticateerror`, `git_ui: ButtonStates`, `gpui: AnyWindowHandle`, `language_models: CacheTtl`, `editor: assert_blame_rows()`, `gpui_macos: caller_action_ids_do_not_colli`, `image_viewer: displayed_render_image()`, `gpui_macos: .set_initial_windowing()`, `project: create_gpui_image()`, `acp_thread: .create_terminal()`, `scheduler: SessionId`, `agent: tools/edit_file_tool.rs`, `extensions_ui: DevExtensionNotInstalledError`, `agent_ui: build_assist_editor_renderer()`, `gpui: example_editor.rs`, `debugger_ui: ActiveBreakpointStripMode`, `remote_server: AddWorktree`, `git_ui_core: git_ui_core/src/notifications.`, `livekit_client: ConnectionQuality`, `channel: .link()`, `git_ui_core: DeleteWorktreeTooltip`, `git_ui: CompareWithBranch`, `language_models: ClientTokenProvider`, `gpui: .tabbed_windows()`, `title_bar: AccountButton`, `keymap_editor: keystroke_input.rs`, `terminal_view: alternatescroll`, `editor: ActiveScrollbarState`, `project: AdaptiveBatcher<T>`, `call: CallCanceled`, `debugger_ui: send_telemetry()`, `language_tools: EnabledLogStreams`, `onboarding: go_to_welcome_page()`, `http_client: FetchStep`, `agent_ui: agent_configuration.rs`, `remote_connection: .ask_password()`, `extension: test_debug_adapter_schemas()`, `extension_host: Event`, `fs: atomic_replace()`, `gpui: process_info.rs`, `language_models: add_message_content_part()`, `open_path_prompt: DirectoryState`, `agent_ui: agent_panel`, `project: BinarySettings`, `copilot_ui: .status()`, `edit_prediction_ui: ActivePrediction`, `gpui: bench_task_reports_long_task_w`, `language_tools: ServerHealth`, `picker_preview: picker_preview.rs`, `agent: contextserverstatus`, `editor: test_client_can_wrap_with_emme`, `git_ui: ButtonStates`, `x_ai_subscribed: src/x_ai_subscribed.rs`, `edit_prediction: childentriesoptions`, `release_channel: should_install_crash_handler()`, `git_ui: ButtonStates`, `language_models: provider/open_ai_compatible.rs`, `audio: apm`, `agent_ui: collect_markdowns()`, `remote: .fmt()`, `gpui: .set_dock_menu()`, `language_models: ConfigurationView`, `diagnostics: init()`, `agent: deserialize_maybe_stringified(`, `git_ui: stash.rs`, `toolchain_selector: ImageLoadingExample`, `gpui: AtomicU16`, `workspace: StoreData`, `repl: ClientControlConnection`, `collab_ui: CopyLink`, `copilot: Copilot`, `diagnostics: BufferDiagnosticsEditor`, `git_ui: copy_branch_name()`, `language_models: BoxStream`, `workspace: AutoHideFade`, `json_schema_store: borrowappcontext_as`, `debugger_ui: .stack_frame_list()`, `gpui: Counter`, `zed: clone_and_open`, `language_tools: DisplayItem`, `remote: MessageId`, `tabular_data_preview: ClearSelection`, `agent_ui: .emit_load_error()`, `deployment: .connect()`, `agent: decide_permission_for_paths()`, `db: OnboardingUpsell`, `miniprofiler_ui: ProfilingCollector`, `language_models: AnthropicLanguageModelProvider`, `language_models: BedrockAuthMethodContent`, `language_models: provider/x_ai.rs`, `language_tools: language_tools.rs`, `lsp_locations: build_location_matches()`, `agent: path_matches_always_confirm()`, `agent_ui: all_models()`, `context_server: AnyNotification`, `editor: debugger_hover.rs`, `search: buffer_search_query()`, `tabular_data_preview: tabular_data_preview/src/parse`, `git_ui: BufferSubscriptions`, `project: context_server_store`, `workspace: AnyElement`, `debugger_ui: CompletionItemType`, `copilot_chat: ChatLocation`, `lsp_command_selector: active_buffer()`, `repl: addcodeblock`, `action_log: ActionLog`, `agent: ActionLog`, `agent: .new()`, `outline: assert_single_caret_at_row()`, `notifications: AddNotification`, `collab_ui: ChannelMembership`, `language_models: available_model_to_google_mode`, `github_releases: asyncseekext`, `language_models: language_model()`, `language_model: image_format_from_external_con`, `snippet_provider: test_snippets()`, `gpui: .read()`, `remote: App`, `search: SearchResults`, `workspace: SaveStrategy`, `keymap_editor: ClearKeystrokes`, `client: calculate_json_checksum()`, `inspector_ui: assert_inspector_watchers()`, `project_bookmarks: bookmarkstore`, `acp_thread: .patch_preview()`, `action_log: clock`, `tab_switcher: CloseSelectedItem`, `editor: BookmarksTabEvent`, `zed: WatchSnapshot`, `gpui: examples/testing.rs`, `project: Bookmark`, `workspace: OpenRecentProject`, `agent: skill_tool.rs`, `git_ui_core: build_buffer_diff()`, `remote: handle_io()`, `x_ai_subscribed: do_oauth_flow()`, `channel: src/channel.rs`, `terminal: extension_api/src/process.rs`, `zed: EventWrapper`, `context_server: AnyRequest`, `project: BranchDiffEvent`, `tabular_data_preview: ColumnFilterDelegate`, `agent_ui: .active_initial_content()`, `call: call_impl/diagnostics.rs`, `language_models: ConfigurationView`, `svg_preview: App`, `gpui_web: applicationhandle`, `git_ui: common_prefix()`, `git_ui: AnyEntity`, `git_ui: build_clipboard_buffer()`, `activity_indicator: ActivityIcon`, `repl: local_kernel_specifications()`, `agent: default_context_lines()`, `git_ui: .editor_position()`, `agent: collapse_multiline_range()`, `gpui: HeadlessAppContext`, `repl: build_grouped_entries()`, `settings_ui: pages.rs`, `language: BufferState`, `collab_ui: CallDiagnosticsReport`, `edit_prediction: fixed_line_window_around_curso`, `language_models: copilotsettings`, `gpui: AppContext`, `agent_ui: cycling_modes_preserves_choice`, `edit_prediction_context: CachedDefinition`, `workspace: init()`, `debugger_ui: .focus_handle()`, `debugger_ui: ModuleList`, `editor: Editor`, `encoding_selector: available_encodings()`, `project: DefaultPrettier`, `agent: create_directory_tool.rs`, `agent: init_test()`, `language_models: authenticate_does_not_fetch_mo`, `recent_projects: Cancel`, `settings_profile_selector: display_name()`, `tabular_data_preview: EditorState`, `edit_prediction_ui: ContextRetrievalStartedDebugEv`, `git_ui: Any`, `onboarding: BaseKeymapSelector`, `zed: GlobalMigrationNotification`, `client: event_coalescer.rs`, `line_ending_selector: init()`, `livekit_client: examples/test_app.rs`, `project_symbols: init()`, `extension_host: ExtensionVersion`, `gpui: hot_reload.rs`, `language_models: ApiCompatibleProviderConfigura`, `recent_projects: App`, `agent: AddSelectionsState`, `project: CodeLensActionId`, `language_tools: init()`, `project: can_resolve_link()`, `agent_servers: api_key_for_gemini_cli()`, `project: create_remote_shell()`, `repl: RemoteRunningKernel`, `agent: DeletePathTool`, `repl: App`, `workspace: .detach_and_prompt_err()`, `copilot: File`, `agent_servers: init_test()`, `openai_subscribed: .new()`, `settings_sync: GlobalSettingsSync`, `settings_ui: font_picker.rs`, `editor: Editor`, `gpui_tokio: gpui_tokio.rs`, `project: context_server_store/registry.`, `settings_ui: icon_theme_picker()`, `settings_ui: OllamaModelPicker`, `settings_ui: App`, `agent: FindPathTool`, `gpui: inhibit_idle_sleep()`, `livekit_client: remote_video_track_view.rs`, `action_log: .keep_all_edits()`, `openai_subscribed: Context`, `agent: FindReferencesTool`, `agent: GoToDefinitionTool`, `agent: RenameTool`, `theme_extension: App`, `agent_servers: .list_sessions()`, `language_models: .api_url()`, `project: telemetry_snapshot.rs`, `openai_subscribed: make_fresh_credentials()`, `project: Manager`, `editor: ActionFetchReady`?**
  _High betweenness centrality (0.065) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `Task` (e.g. with `.background_spawn()` and `.spawn_dedicated()`) actually correct?**
  _`Task` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Should `Project & Agent Server Core` be split into smaller, more focused modules?**
  _Cohesion score 0.0032249642479605114 - nodes in this community are weakly interconnected._