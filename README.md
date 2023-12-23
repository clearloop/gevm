# GEVM

> [!WARNING]
>
> This is an expermental project just for exploring the potential
> of embedding EVM in gear.

EVM in gear wasm environment.

## Currently failed 😅

```rust
[tracing::span::active add_two] <- init_lazy_pages_for_program_version_1;
[tracing::span add_two] -- init_lazy_pages_for_program_version_1;
[gear_lazy_pages::signal add_two] Interrupted, exception info = ExceptionInfo { fault_addr: 0x2800ffc6c, is_write: Some(true) }
[gear_lazy_pages::mprotect add_two] mprotect interval: 0x2800fc000, size: 0x4000, mask: rw-
[syscalls add_two] size(-374680786, 1047660)
[tracing::span add_two] pre_process_memory_accesses_version_2;
[tracing::span::active add_two] -> pre_process_memory_accesses_version_2;
[gear_lazy_pages::host_func add_two] host func mem accesses: [] [[offset: 0xffc6c, size: 0x4]]
[tracing::span::active add_two] <- pre_process_memory_accesses_version_2;
[tracing::span add_two] -- pre_process_memory_accesses_version_2;
[gear_core_processor::ext add_two] Skipped decrease to global value
[gear_lazy_pages::signal add_two] Interrupted, exception info = ExceptionInfo { fault_addr: 0x280114ae4, is_write: Some(false) }
[gear_lazy_pages::mprotect add_two] mprotect interval: 0x280114000, size: 0x4000, mask: r--
[gear_lazy_pages::signal add_two] Interrupted, exception info = ExceptionInfo { fault_addr: 0x280114ae8, is_write: Some(true) }
[gear_lazy_pages::mprotect add_two] mprotect interval: 0x280114000, size: 0x4000, mask: rw-
[gear_lazy_pages::signal add_two] Interrupted, exception info = ExceptionInfo { fault_addr: 0x28011fffc, is_write: Some(true) }
[gear_lazy_pages::mprotect add_two] mprotect interval: 0x28011c000, size: 0x4000, mask: rw-
[syscalls add_two] size(-374691041, 1047644)
[tracing::span add_two] pre_process_memory_accesses_version_2;
[tracing::span::active add_two] -> pre_process_memory_accesses_version_2;
[gear_lazy_pages::host_func add_two] host func mem accesses: [] [[offset: 0xffc5c, size: 0x4]]
[tracing::span::active add_two] <- pre_process_memory_accesses_version_2;
[tracing::span add_two] -- pre_process_memory_accesses_version_2;
[gear_core_processor::ext add_two] Skipped decrease to global value
[syscalls add_two] read(-374691097, 0, 82, 1133368, 1047660)
[tracing::span add_two] pre_process_memory_accesses_version_2;
[tracing::span::active add_two] -> pre_process_memory_accesses_version_2;
[gear_lazy_pages::host_func add_two] host func mem accesses: [] [[offset: 0x114b38, size: 0x52]]
[tracing::span::active add_two] <- pre_process_memory_accesses_version_2;
[tracing::span add_two] -- pre_process_memory_accesses_version_2;
[gear_core_processor::ext add_two] Skipped decrease to global value
[tracing::span add_two] pre_process_memory_accesses_version_2;
[tracing::span::active add_two] -> pre_process_memory_accesses_version_2;
[gear_lazy_pages::host_func add_two] host func mem accesses: [] [[offset: 0xffc6c, size: 0x4]]
[tracing::span::active add_two] <- pre_process_memory_accesses_version_2;
[tracing::span add_two] -- pre_process_memory_accesses_version_2;
[gear_core_processor::ext add_two] Skipped decrease to global value
[gear_lazy_pages::signal add_two] Interrupted, exception info = ExceptionInfo { fault_addr: 0x28010d570, is_write: Some(false) }
[gear_lazy_pages::mprotect add_two] mprotect interval: 0x28010c000, size: 0x4000, mask: r--
[gear_lazy_pages::signal add_two] Interrupted, exception info = ExceptionInfo { fault_addr: 0x280101dc8, is_write: Some(false) }
[gear_lazy_pages::mprotect add_two] mprotect interval: 0x280100000, size: 0x4000, mask: r--
[gear_lazy_pages::signal add_two] Interrupted, exception info = ExceptionInfo { fault_addr: 0x28010b0d1, is_write: Some(false) }
[gear_lazy_pages::mprotect add_two] mprotect interval: 0x280108000, size: 0x4000, mask: r--
[gear_lazy_pages::signal add_two] Interrupted, exception info = ExceptionInfo { fault_addr: 0x280110f4c, is_write: Some(false) }
[gear_lazy_pages::mprotect add_two] mprotect interval: 0x280110000, size: 0x4000, mask: r--
[runtime::sandbox add_two] invocation error: wasm `unreachable` instruction executed
[gear_core_backend::state add_two] Execution result = Err(Error::Execution)
[tracing::span add_two] mprotect_lazy_pages_version_1;
[tracing::span::active add_two] -> mprotect_lazy_pages_version_1;
[gear_lazy_pages::mprotect add_two] mprotect interval: 0x280000000, size: 0x120000, mask: rw-
[tracing::span::active add_two] <- mprotect_lazy_pages_version_1;
[tracing::span add_two] -- mprotect_lazy_pages_version_1;
[tracing::span add_two] lazy_pages_status_version_1;
[tracing::span::active add_two] -> lazy_pages_status_version_1;
[tracing::span::active add_two] <- lazy_pages_status_version_1;
[tracing::span add_two] -- lazy_pages_status_version_1;
[gear_core_processor::executor add_two] Termination reason: Trap(Unknown)
[tracing::span add_two] write_accessed_pages_version_1;
[tracing::span::active add_two] -> write_accessed_pages_version_1;
[tracing::span::active add_two] <- write_accessed_pages_version_1;
[gear_core_processor::executor add_two] 💥 Trap during execution of 0x0100000000000000000000000000000000000000000000000000000000000000
[gtest::manager add_two] [0xa75838b553e77f3623ecad2b62794a11f86c3d2f3ab06d572ab5584ecdba41e3] new dispatch#0x38c981e8b858ba2c393f1234038f9cf756718f1b3cd1dc4fbccdc97ba6105e97
```

btw the gas cost is too much `374973548` for addtwo.

```rust
RunResult { log: [CoreLog { id: 0x38c981e8b858ba2c393f1234038f9cf756718f1b3cd1dc4fbccdc97ba6105e97, source: 0x0100000000000000000000000000000000000000000000000000000000000000, destination: 0x0000000000000000000000000000000000000000000000000000000000000000, payload: 0x5265
61736f6e2069..6f63637572726564, reply_code: Some(Error(Execution(UnreachableInstruction))) }], main_failed: true, others_failed: false, message_id: 0xa75838b553e77f3623ecad2b62794a11f86c3d2f3ab06d572ab5584ecdba41e3, total_processed: 1, main_gas_burned: Gas(374973548), oth
ers_gas_burned: Gas(0) }
```

## LICENSE

GPL-3.0-only
