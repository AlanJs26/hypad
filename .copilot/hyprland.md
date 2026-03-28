# Crate Documentation

**Version:** 0.4.0-beta.3

**Format Version:** 57

# Module `hyprland`

# Hyprland-rs

[![Crates.io](https://img.shields.io/crates/v/hyprland)](https://crates.io/crates/hyprland)
![Crates.io](https://img.shields.io/crates/d/hyprland)
[![Crates.io](https://img.shields.io/crates/l/hyprland)](https://www.gnu.org/licenses/gpl-3.0.html)
[![docs.rs](https://img.shields.io/docsrs/hyprland)](https://docs.rs/hyprland)
[![Hyprland](https://img.shields.io/badge/Made%20for-Hyprland-blue)](https://github.com/hyprwm/Hyprland)
[![Discord](https://img.shields.io/discord/1055990214411169892?label=discord)](https://discord.gg/zzWqvcKRMy)

An unofficial rust wrapper for Hyprland's IPC

## Help Wanted!
We need help with developing the next version of hyprland-rs `0.4`,
if you know how to do the things in <https://github.com/hyprland-community/hyprland-rs/milestone/4>
contributions in those areas would be greatly appreciated!

## Disclaimer

If something doesn't work, doesn't matter what,
make sure you are on the latest version (or commit) of Hyprland before making an issue!

## Getting started!

Let's get started with Hyprland-rs!

### Adding to your project

Add the code below to the dependencies section of your Cargo.toml file!

```toml
hyprland = "0.4.0-beta.3"
```

### Reading the docs

Hyprland-rs has a ton of types (and some really long ones)! Its important you know how the ones you will be using work!
The docs can be found at [docs.rs/hyprland](https://docs.rs/hyprland)

#### Master version

If Hyprland-rs is broken (or other reason) and is taking too long for a release to come out,
you can use the master branch in Cargo (will not allow the crate to be published to `crates.io`):

```toml
hyprland = { git = "https://github.com/hyprland-community/hyprland-rs", branch = "master" }
```

### What this crate provides

This crate provides 6 modules (+1 for shared things)

- `data` for getting information on the compositor
- `event_listener` which provides the `EventListener` struct for listening for events
- `dispatch` for calling dispatchers
- `keyword` for dealing with config option (aka keywords)
- `config::binds` for changing binds (in future `config` might have config generation)
- `ctl` for calling hyprctl commands

## Example Usage

Check the examples in the [`examples` directory](https://github.com/hyprland-community/hyprland-rs/tree/master/examples)

## Modules

## Module `shared`

This module provides shared things throughout the crate
# The Shared Module

This module provides shared private and public functions, structs, enum, and types

```rust
pub mod shared { /* ... */ }
```

### Types

#### Struct `Address`

The address struct holds a address as a tuple with a single value
and has methods to reveal the address in different data formats

```rust
pub struct Address(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new<T: ToString>(string: T) -> Self { /* ... */ }
  ```
  This creates a new address from a value that implements [ToString]

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Address { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Address) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Address) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Address) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Type Alias `WorkspaceId`

This type provides the id used to identify workspaces
> its a type because it might change at some point

```rust
pub type WorkspaceId = i32;
```

#### Type Alias `MonitorId`

This type provides the id used to identify monitors
> its a type because it might change at some point

```rust
pub type MonitorId = i128;
```

#### Enum `WorkspaceType`

**Attributes:**

- `Other("#[serde(untagged)]")`

This enum holds workspace data

```rust
pub enum WorkspaceType {
    Regular(String),
    Special(Option<String>),
}
```

##### Variants

###### `Regular`

A named workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` | The name |

###### `Special`

The special workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<String>` | The name, if exists |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WorkspaceType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: &WorkspaceType) -> Self { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &WorkspaceType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WorkspaceType) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WorkspaceType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(int: u8) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(int: u16) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(int: u32) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(int: u64) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(int: usize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(int: i8) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(int: i16) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(int: i32) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(int: i64) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(int: isize) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `CommandFlag`

This enum defines the possible command flags that can be used.

```rust
pub enum CommandFlag {
    JSON,
    Empty,
}
```

##### Variants

###### `JSON`

The JSON flag.

###### `Empty`

An empty flag.

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CommandFlag { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> CommandFlag { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CommandFlag) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `CommandContent`

This struct defines the content of a command, which consists of a flag and a data string.

```rust
pub struct CommandContent {
    pub flag: CommandFlag,
    pub data: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `flag` | `CommandFlag` | The flag for the command. |
| `data` | `String` | The data string for the command. |

##### Implementations

###### Methods

- ```rust
  pub fn as_bytes(self: &Self) -> Vec<u8> { /* ... */ }
  ```
  Converts the command content to a byte vector.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CommandContent { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> CommandContent { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```
    Formats the command content as a string for display.

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CommandContent) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Traits

#### Trait `HyprData`

This trait provides a standardized way to get data

```rust
pub trait HyprData {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get`: This method gets the data
- `get_async`: This method gets the data (async)
- `instance_get`: This method gets the data
- `instance_get_async`: This method gets the data (async)

##### Implementations

This trait is implemented for the following types:

- `Monitors`
- `Workspaces`
- `Clients`
- `Layers`
- `Devices`
- `Version`
- `CursorPosition`
- `Binds`
- `Animations`
- `WorkspaceRules`
- `FullscreenState`

#### Trait `HyprDataVec`

This trait provides a standardized way to get data in a from of a vector

```rust
pub trait HyprDataVec<T>: HyprData {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `to_vec`: This method returns a vector of data

##### Implementations

This trait is implemented for the following types:

- `Monitors`
- `Workspaces`
- `Clients`
- `Binds`
- `WorkspaceRules`

#### Trait `HyprDataActive`

Trait for helper functions to get the active of the implementor

```rust
pub trait HyprDataActive {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get_active`: This method gets the active data
- `get_active_async`: This method gets the active data (async)
- `instance_get_active`: This method gets the active data
- `instance_get_active_async`: This method gets the active data (async)

##### Implementations

This trait is implemented for the following types:

- `Monitor`
- `Workspace`

#### Trait `HyprDataActiveOptional`

Trait for helper functions to get the active of the implementor, but for optional ones

```rust
pub trait HyprDataActiveOptional {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get_active`: This method gets the active data
- `get_active_async`: This method gets the active data (async)
- `instance_get_active`: This method gets the active data
- `instance_get_active_async`: This method gets the active data (async)

##### Implementations

This trait is implemented for the following types:

- `Client`

### Re-exports

#### Re-export `command`

```rust
pub use command;
```

## Module `data`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"data\"), span: src/lib.rs:26:7: 26:23 (#0) }])]")`

This module provides functions for getting information on the compositor
# Data module

This module provides functions for getting information on the compositor

## Usage

here is a example of every function in use! (blocking)
```rust
use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::Result;

fn main() -> Result<()> {
    let instance = &hyprland::instance::Instance::from_current_env()?;

    let monitors = Monitors::get(instance)?.to_vec();
    println!("{monitors:#?}");

    let workspaces = Workspaces::get(instance)?.to_vec();
    println!("{workspaces:#?}");

    let clients = Clients::get(instance)?.to_vec();
    println!("{clients:#?}");

    let active_window = Client::get_active(instance)?;
    println!("{active_window:#?}");

    let layers = Layers::get(instance)?;
    println!("{layers:#?}");

    let devices = Devices::get(instance)?;
    println!("{devices:#?}");

    let version = Version::get(instance)?;
    println!("{version:#?}");

    let cursor_pos = CursorPosition::get(instance)?;
    println!("{cursor_pos:#?}");
    Ok(())
}
```

```rust
pub mod data { /* ... */ }
```

### Re-exports

#### Re-export `crate::data::helpers::*`

```rust
pub use crate::data::helpers::*;
```

#### Re-export `crate::data::regular::*`

```rust
pub use crate::data::regular::*;
```

## Module `event_listener`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"listener\"), span: src/lib.rs:30:7: 30:27 (#0) }])]")`

This module provides the EventListener struct for listening and acting upon for events
# Event Listener Module
for documentation go to:
* [EventStream] for the event listener implementation based on the [futures_lite::Stream] api
* [EventListener] for the normal [Fn] based event listener
* [AsyncEventListener] for the [Fn] based event listener which uses closures that return [std::future::Future]s

```rust
pub mod event_listener { /* ... */ }
```

### Re-exports

#### Re-export `EventListener`

```rust
pub use crate::event_listener::immutable::EventListener;
```

#### Re-export `AsyncEventListener`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/event_listener/mod.rs:18:11: 18:33 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/event_listener/mod.rs:18:35: 18:52 (#0) }], src/event_listener/mod.rs:18:10: 18:53 (#0))])]")`

```rust
pub use crate::event_listener::async_im::AsyncEventListener;
```

#### Re-export `EventStream`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/event_listener/mod.rs:23:11: 23:33 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/event_listener/mod.rs:23:35: 23:52 (#0) }], src/event_listener/mod.rs:23:10: 23:53 (#0))])]")`

```rust
pub use crate::event_listener::stream::EventStream;
```

#### Re-export `crate::event_listener::shared::*`

```rust
pub use crate::event_listener::shared::*;
```

## Module `dispatch`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"dispatch\"), span: src/lib.rs:34:7: 34:27 (#0) }])]")`

This module is for calling dispatchers and changing keywords
# Dispatch module

This module is used for calling dispatchers and changing keywords

## Usage

```rust
use hyprland::Result;
use hyprland::dispatch::{Dispatch, DispatchType};
fn main() -> Result<()> {
    Dispatch::call(DispatchType::Exec("kitty"))?;

   Ok(())
}
````

```rust
pub mod dispatch { /* ... */ }
```

### Types

#### Enum `WindowIdentifier`

This enum is for identifying a window

```rust
pub enum WindowIdentifier<''a> {
    Address(Address),
    ClassRegularExpression(&''a str),
    Title(&''a str),
    Tag(&''a str),
    ProcessId(u32),
    ActiveWindow,
    Floating,
    Tiled,
}
```

##### Variants

###### `Address`

The address of a window

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Address` |  |

###### `ClassRegularExpression`

A Regular Expression to match the window class (handled by Hyprland)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

###### `Title`

The window title

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

###### `Tag`

A window tag regex

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

###### `ProcessId`

The window's process Id

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

###### `ActiveWindow`

The active window

###### `Floating`

The first floating window

###### `Tiled`

The first tiled window

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WindowIdentifier<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `FullscreenType`

This enum holds the fullscreen types

```rust
pub enum FullscreenType {
    Real,
    Maximize,
    NoParam,
}
```

##### Variants

###### `Real`

Fills the whole screen

###### `Maximize`

Maximizes the window

###### `NoParam`

Passes no param

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FullscreenType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `FullscreenState`

**Attributes:**

- `Other("#[allow(missing_docs)]")`

This enum holds the params to the [DispatchType::ToggleFullscreenState] dispatcher

```rust
pub enum FullscreenState {
    Current = -1,
    None = 0,
    Maximize = 1,
    Fullscreen = 2,
    MaximizeFullscreen = 3,
}
```

##### Variants

###### `Current`

Discriminant: `-1`

Discriminant value: `-1`

###### `None`

Discriminant: `0`

Discriminant value: `0`

###### `Maximize`

Discriminant: `1`

Discriminant value: `1`

###### `Fullscreen`

Discriminant: `2`

Discriminant value: `2`

###### `MaximizeFullscreen`

Discriminant: `3`

Discriminant value: `3`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FullscreenState { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `Direction`

**Attributes:**

- `Other("#[allow(missing_docs)]")`

This enum holds directions, typically used for moving

```rust
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
```

##### Variants

###### `Up`

###### `Down`

###### `Right`

###### `Left`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Direction { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `Position`

This enum is used for resizing and moving windows precisely

```rust
pub enum Position {
    Delta(i16, i16),
    Exact(i16, i16),
    DeltaFraction(i16, i16),
    ExactFraction(i16, i16),
}
```

##### Variants

###### `Delta`

A delta in pixels

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i16` |  |
| 1 | `i16` |  |

###### `Exact`

The exact size in pixels

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i16` |  |
| 1 | `i16` |  |

###### `DeltaFraction`

A delta in window fraction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i16` |  |
| 1 | `i16` |  |

###### `ExactFraction`

The exact size in screen fraction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i16` |  |
| 1 | `i16` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Position { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `CycleDirection`

**Attributes:**

- `Other("#[allow(missing_docs)]")`

This enum holds a direction for cycling

```rust
pub enum CycleDirection {
    Next,
    Previous,
}
```

##### Variants

###### `Next`

###### `Previous`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CycleDirection { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `WindowSwitchDirection`

**Attributes:**

- `Other("#[allow(missing_docs)]")`

This enum holds a direction for switch windows in a group

```rust
pub enum WindowSwitchDirection {
    Back,
    Forward,
    Index(i32),
}
```

##### Variants

###### `Back`

###### `Forward`

###### `Index`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WindowSwitchDirection { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `MonitorIdentifier`

This enum is used for identifying monitors

```rust
pub enum MonitorIdentifier<''a> {
    Direction(Direction),
    Id(MonitorId),
    Name(&''a str),
    Current,
    Relative(i32),
}
```

##### Variants

###### `Direction`

The monitor that is to the specified direction of the active one

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Direction` |  |

###### `Id`

The monitor id

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MonitorId` |  |

###### `Name`

The monitor name

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

###### `Current`

The current monitor

###### `Relative`

The workspace relative to the current workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MonitorIdentifier<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `Corner`

**Attributes:**

- `Other("#[allow(missing_docs)]")`

This enum holds corners

```rust
pub enum Corner {
    BottomLeft = 0,
    BottomRight = 1,
    TopRight = 2,
    TopLeft = 3,
}
```

##### Variants

###### `BottomLeft`

Discriminant: `0`

Discriminant value: `0`

###### `BottomRight`

Discriminant: `1`

Discriminant value: `1`

###### `TopRight`

Discriminant: `2`

Discriminant value: `2`

###### `TopLeft`

Discriminant: `3`

Discriminant value: `3`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Corner { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `WorkspaceOptions`

This enum holds options that are applied to the current workspace

```rust
pub enum WorkspaceOptions {
    AllPseudo,
    AllFloat,
}
```

##### Variants

###### `AllPseudo`

Makes all windows pseudo tiled

###### `AllFloat`

Makes all windows float

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WorkspaceOptions { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `FirstEmpty`

This struct holds options for the first empty workspace

```rust
pub struct FirstEmpty {
    pub on_monitor: bool,
    pub next: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `on_monitor` | `bool` | If the first empty workspace should be on the monitor |
| `next` | `bool` | If the first empty workspace should be next |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FirstEmpty { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FirstEmpty) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `WorkspaceIdentifierWithSpecial`

This enum is for identifying workspaces that also includes the special workspace

```rust
pub enum WorkspaceIdentifierWithSpecial<''a> {
    Id(WorkspaceId),
    Relative(i32),
    RelativeMonitor(i32),
    RelativeMonitorIncludingEmpty(i32),
    RelativeOpen(i32),
    Previous,
    PreviousPerMonitor,
    Empty(FirstEmpty),
    Name(&''a str),
    Special(Option<&''a str>),
}
```

##### Variants

###### `Id`

The workspace Id

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WorkspaceId` |  |

###### `Relative`

The workspace relative to the current workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

###### `RelativeMonitor`

The workspace on the monitor relative to the current workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

###### `RelativeMonitorIncludingEmpty`

The workspace on the monitor relative to the current workspace, including empty workspaces

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

###### `RelativeOpen`

The open workspace relative to the current workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

###### `Previous`

The previous Workspace

###### `PreviousPerMonitor`

The previous Workspace

###### `Empty`

The first available empty workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `FirstEmpty` |  |

###### `Name`

The name of the workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

###### `Special`

The special workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<&''a str>` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WorkspaceIdentifierWithSpecial<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WorkspaceIdentifierWithSpecial<''a>) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `WorkspaceIdentifier`

This enum is for identifying workspaces

```rust
pub enum WorkspaceIdentifier<''a> {
    Id(WorkspaceId),
    Relative(i32),
    RelativeMonitor(i32),
    RelativeMonitorIncludingEmpty(i32),
    RelativeOpen(i32),
    Previous,
    Empty,
    Name(&''a str),
}
```

##### Variants

###### `Id`

The workspace Id

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WorkspaceId` |  |

###### `Relative`

The workspace relative to the current workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

###### `RelativeMonitor`

The workspace on the monitor relative to the current workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

###### `RelativeMonitorIncludingEmpty`

The workspace on the monitor relative to the current workspace, including empty workspaces

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

###### `RelativeOpen`

The open workspace relative to the current workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

###### `Previous`

The previous Workspace

###### `Empty`

The first available empty workspace

###### `Name`

The name of the workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WorkspaceIdentifier<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WorkspaceIdentifier<''a>) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `WindowMove`

This enum is the params to [DispatchType::MoveWindow] dispatcher

```rust
pub enum WindowMove<''a> {
    Monitor(MonitorIdentifier<''a>),
    Direction(Direction),
}
```

##### Variants

###### `Monitor`

Moves the window to a specified monitor

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MonitorIdentifier<''a>` |  |

###### `Direction`

Moves the window in a specified direction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Direction` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WindowMove<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `TagAction`

**Attributes:**

- `Other("#[allow(missing_docs)]")`

This enum holds the actions that can be applied to a tag

```rust
pub enum TagAction {
    Add,
    Remove,
    Toggle,
}
```

##### Variants

###### `Add`

###### `Remove`

###### `Toggle`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TagAction { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `SignalType`

This enum holds the signals

```rust
pub enum SignalType {
    SIGHUP = 1,
    SIGINT = 2,
    SIGQUIT = 3,
    SIGILL = 4,
    SIGTRAP = 5,
    SIGABRT = 6,
    SIGBUS = 7,
    SIGFPE = 8,
    SIGKILL = 9,
    SIGUSR1 = 10,
    SIGSEGV = 11,
    SIGUSR2 = 12,
    SIGPIPE = 13,
    SIGALRM = 14,
    SIGTERM = 15,
    SIGSTKFLT = 16,
    SIGCHLD = 17,
    SIGCONT = 18,
    SIGSTOP = 19,
    SIGTSTP = 20,
    SIGTTIN = 21,
    SIGTTOU = 22,
    SIGURG = 23,
    SIGXCPU = 24,
    SIGXFSZ = 25,
    SIGVTALRM = 26,
    SIGPROF = 27,
    SIGWINCH = 28,
    SIGIO = 29,
    SIGPWR = 30,
    SIGSYS = 31,
}
```

##### Variants

###### `SIGHUP`

Hangup detected on controlling terminal

Discriminant: `1`

Discriminant value: `1`

###### `SIGINT`

Interrupt from keyboard

Discriminant: `2`

Discriminant value: `2`

###### `SIGQUIT`

Quit from keyboard

Discriminant: `3`

Discriminant value: `3`

###### `SIGILL`

Illegal Instruction

Discriminant: `4`

Discriminant value: `4`

###### `SIGTRAP`

Trace/breakpoint trap

Discriminant: `5`

Discriminant value: `5`

###### `SIGABRT`

Abort signal from abort

Discriminant: `6`

Discriminant value: `6`

###### `SIGBUS`

Bus error (bad memory access)

Discriminant: `7`

Discriminant value: `7`

###### `SIGFPE`

Erroneous arithmetic operation

Discriminant: `8`

Discriminant value: `8`

###### `SIGKILL`

Kill signal

Discriminant: `9`

Discriminant value: `9`

###### `SIGUSR1`

User-defined signal 1

Discriminant: `10`

Discriminant value: `10`

###### `SIGSEGV`

Invalid memory reference

Discriminant: `11`

Discriminant value: `11`

###### `SIGUSR2`

User-defined signal 2

Discriminant: `12`

Discriminant value: `12`

###### `SIGPIPE`

Broken pipe

Discriminant: `13`

Discriminant value: `13`

###### `SIGALRM`

Timer signal from alarm

Discriminant: `14`

Discriminant value: `14`

###### `SIGTERM`

Termination signal

Discriminant: `15`

Discriminant value: `15`

###### `SIGSTKFLT`

Stack fault on coprocessor

Discriminant: `16`

Discriminant value: `16`

###### `SIGCHLD`

Child stopped, terminated, or continued

Discriminant: `17`

Discriminant value: `17`

###### `SIGCONT`

Continue if stopped

Discriminant: `18`

Discriminant value: `18`

###### `SIGSTOP`

Stop process

Discriminant: `19`

Discriminant value: `19`

###### `SIGTSTP`

Stop typed at terminal

Discriminant: `20`

Discriminant value: `20`

###### `SIGTTIN`

Terminal input for background process

Discriminant: `21`

Discriminant value: `21`

###### `SIGTTOU`

Terminal output for background process

Discriminant: `22`

Discriminant value: `22`

###### `SIGURG`

Urgent condition on socket

Discriminant: `23`

Discriminant value: `23`

###### `SIGXCPU`

CPU time limit exceeded

Discriminant: `24`

Discriminant value: `24`

###### `SIGXFSZ`

File size limit exceeded

Discriminant: `25`

Discriminant value: `25`

###### `SIGVTALRM`

Virtual alarm clock

Discriminant: `26`

Discriminant value: `26`

###### `SIGPROF`

Profiling timer expired

Discriminant: `27`

Discriminant value: `27`

###### `SIGWINCH`

Window resize signal

Discriminant: `28`

Discriminant value: `28`

###### `SIGIO`

I/O now possible

Discriminant: `29`

Discriminant value: `29`

###### `SIGPWR`

Power failure

Discriminant: `30`

Discriminant value: `30`

###### `SIGSYS`

Bad system call

Discriminant: `31`

Discriminant value: `31`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SignalType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `MoveToRootParam`

This enum holds the params to the [DispatchType::MoveToRoot] dispatcher

```rust
pub enum MoveToRootParam {
    Stable,
    Unstable,
}
```

##### Variants

###### `Stable`

Maximize the window in its current subtree

###### `Unstable`

Swap the window with the other subtree

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MoveToRootParam { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `ZOrder`

This enum holds the zheight variants

```rust
pub enum ZOrder {
    Top,
    Bottom,
}
```

##### Variants

###### `Top`

Bring the active window to top of the stack

###### `Bottom`

Bring the active window to bottom of the stack

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ZOrder { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `SubmapParam`

This enum holds the params to the [DispatchType::Submap] dispatcher

```rust
pub enum SubmapParam<''a> {
    Reset,
    Name(&''a str),
}
```

##### Variants

###### `Reset`

Go back to global submap

###### `Name`

Go to named submap

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SubmapParam<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `DispatchType`

This enum holds every dispatcher

```rust
pub enum DispatchType<''a> {
    Custom(&''a str, &''a str),
    SetCursor(&''a str, u16),
    Exec(&''a str),
    ExecRaw(&''a str),
    Pass(WindowIdentifier<''a>),
    Global(&''a str),
    KillActiveWindow,
    ForceKillActiveWindow,
    CloseWindow(WindowIdentifier<''a>),
    Workspace(WorkspaceIdentifierWithSpecial<''a>),
    MoveToWorkspace(WorkspaceIdentifierWithSpecial<''a>, Option<WindowIdentifier<''a>>),
    MoveToWorkspaceSilent(WorkspaceIdentifierWithSpecial<''a>, Option<WindowIdentifier<''a>>),
    ToggleFloating(Option<WindowIdentifier<''a>>),
    SetFloating(Option<WindowIdentifier<''a>>),
    SetTiled(Option<WindowIdentifier<''a>>),
    ToggleFullscreen(FullscreenType),
    ToggleFullscreenState(FullscreenState, FullscreenState),
    ToggleFakeFullscreen,
    ToggleDPMS(bool, Option<&''a str>),
    TogglePseudo,
    TogglePin,
    TogglePinWindow(WindowIdentifier<''a>),
    Signal(SignalType),
    SignalWindow(WindowIdentifier<''a>, SignalType),
    MoveFocus(Direction),
    MoveWindow(WindowMove<''a>),
    CenterWindow,
    ResizeActive(Position),
    MoveActive(Position),
    ResizeWindowPixel(Position, WindowIdentifier<''a>),
    MoveWindowPixel(Position, WindowIdentifier<''a>),
    CycleWindow(CycleDirection),
    SwapNext(CycleDirection),
    SwapWindow(Direction),
    TagWindow(TagAction, &''a str, Option<WindowIdentifier<''a>>),
    FocusWindow(WindowIdentifier<''a>),
    FocusMonitor(MonitorIdentifier<''a>),
    ChangeSplitRatio(FloatValue),
    ToggleOpaque,
    MoveCursorToCorner(Corner),
    MoveCursor(i64, i64),
    WorkspaceOption(WorkspaceOptions),
    RenameWorkspace(WorkspaceId, Option<&''a str>),
    Exit,
    ForceRendererReload,
    MoveCurrentWorkspaceToMonitor(MonitorIdentifier<''a>),
    MoveWorkspaceToMonitor(WorkspaceIdentifier<''a>, MonitorIdentifier<''a>),
    SwapActiveWorkspaces(MonitorIdentifier<''a>, MonitorIdentifier<''a>),
    BringActiveToTop,
    AlterZOrder(ZOrder, Option<WindowIdentifier<''a>>),
    ToggleSpecialWorkspace(Option<String>),
    FocusUrgentOrLast,
    FocusCurrentOrLast,
    ToggleSwallow,
    Submap(SubmapParam<''a>),
    ToggleSplit,
    SwapSplit,
    PreSelect(Direction),
    MoveToRoot(Option<WindowIdentifier<''a>>, MoveToRootParam),
    SwapWithMaster(SwapWithMasterParam),
    FocusMaster(FocusMasterParam),
    CycleNextMaster(MasterLoopParam),
    CyclePrevMaster(MasterLoopParam),
    SwapNextMaster(MasterLoopParam),
    SwapPrevMaster(MasterLoopParam),
    AddMaster,
    RemoveMaster,
    OrientationLeft,
    OrientationRight,
    OrientationTop,
    OrientationBottom,
    OrientationCenter,
    OrientationNext,
    OrientationPrev,
    OrientationCycle(OrientationParam),
    Mfact(FloatValue),
    RollNext,
    RollPrev,
    ToggleGroup,
    ChangeGroupActive(WindowSwitchDirection),
    LockGroups(LockType),
    LockActiveGroup(LockType),
    MoveIntoGroup(Direction),
    MoveWindowOrGroup(Direction),
    MoveOutOfGroup,
    MoveGroupWindow(WindowSwitchDirection),
    DenyWindowFromGroup(BinaryState),
    SetIgnoreGroupLock(BinaryState),
}
```

##### Variants

###### `Custom`

This lets you use dispatchers not supported by hyprland-rs yet, please make issues before
using

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` | Name of event |
| 1 | `&''a str` | Args |

###### `SetCursor`

This dispatcher changes the current cursor

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` | The cursor theme |
| 1 | `u16` | The size |

###### `Exec`

This dispatcher executes a program

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

###### `ExecRaw`

This dispatcher executes a raw shell command ignoring window rules

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

###### `Pass`

This dispatcher passes a keybind to a window when called in a
keybind, its used for global keybinds. And should **ONLY** be used with keybinds

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WindowIdentifier<''a>` |  |

###### `Global`

Executes a Global Shortcut using the GlobalShortcuts portal.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

###### `KillActiveWindow`

This dispatcher closes the active window/client

###### `ForceKillActiveWindow`

This dispatcher kills the active window/client

###### `CloseWindow`

This dispatcher closes the specified window

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WindowIdentifier<''a>` |  |

###### `Workspace`

This dispatcher changes the current workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WorkspaceIdentifierWithSpecial<''a>` |  |

###### `MoveToWorkspace`

This dispatcher moves a window (focused if not specified) to a workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WorkspaceIdentifierWithSpecial<''a>` |  |
| 1 | `Option<WindowIdentifier<''a>>` |  |

###### `MoveToWorkspaceSilent`

This dispatcher moves a window (focused if not specified) to a workspace, without switching to that
workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WorkspaceIdentifierWithSpecial<''a>` |  |
| 1 | `Option<WindowIdentifier<''a>>` |  |

###### `ToggleFloating`

This dispatcher toggles the floating state of a window (current if not specified)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<WindowIdentifier<''a>>` |  |

###### `SetFloating`

This dispatcher floats a window (current if not specified)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<WindowIdentifier<''a>>` |  |

###### `SetTiled`

This dispatcher tiles a window (current if not specified)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<WindowIdentifier<''a>>` |  |

###### `ToggleFullscreen`

This dispatcher toggles the current window fullscreen state

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `FullscreenType` |  |

###### `ToggleFullscreenState`

This dispatcher sets the focused window’s fullscreen mode and the one sent to the client

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `FullscreenState` |  |
| 1 | `FullscreenState` |  |

###### `ToggleFakeFullscreen`

This dispatcher toggles the focused window’s internal
fullscreen state without altering the geometry

###### `ToggleDPMS`

This dispatcher sets the DPMS status for all monitors

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `Option<&''a str>` |  |

###### `TogglePseudo`

This dispatcher toggles pseudo tiling for the current window

###### `TogglePin`

This dispatcher pins the active window to all workspaces

###### `TogglePinWindow`

This dispatcher pins the specified window to all workspaces

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WindowIdentifier<''a>` |  |

###### `Signal`

This dispatcher sends a signal to the active window

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `SignalType` |  |

###### `SignalWindow`

This dispatcher sends a signal to the specified window

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WindowIdentifier<''a>` |  |
| 1 | `SignalType` |  |

###### `MoveFocus`

This dispatcher moves the window focus in a specified direction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Direction` |  |

###### `MoveWindow`

This dispatcher moves the current window to a monitor or in a specified direction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WindowMove<''a>` |  |

###### `CenterWindow`

This dispatcher centers the active window

###### `ResizeActive`

This dispatcher resizes the active window using a [Position] enum

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Position` |  |

###### `MoveActive`

This dispatcher moves the active window using a [Position] enum

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Position` |  |

###### `ResizeWindowPixel`

This dispatcher resizes the specified window using a [Position] enum

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Position` |  |
| 1 | `WindowIdentifier<''a>` |  |

###### `MoveWindowPixel`

This dispatcher moves the specified window using a [Position] enum

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Position` |  |
| 1 | `WindowIdentifier<''a>` |  |

###### `CycleWindow`

This dispatcher cycles windows using a specified direction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CycleDirection` |  |

###### `SwapNext`

This dispatcher swaps the focused window with the window on a workspace using a specified direction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CycleDirection` |  |

###### `SwapWindow`

This dispatcher swaps windows using a specified direction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Direction` |  |

###### `TagWindow`

Apply tag to current or the first window matching

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TagAction` |  |
| 1 | `&''a str` |  |
| 2 | `Option<WindowIdentifier<''a>>` |  |

###### `FocusWindow`

This dispatcher focuses a specified window

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WindowIdentifier<''a>` |  |

###### `FocusMonitor`

This dispatcher focuses a specified monitor

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MonitorIdentifier<''a>` |  |

###### `ChangeSplitRatio`

This dispatcher changed the split ratio

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `FloatValue` |  |

###### `ToggleOpaque`

This dispatcher toggle opacity for the current window/client

###### `MoveCursorToCorner`

This dispatcher moves the cursor to a specified corner of a window

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Corner` |  |

###### `MoveCursor`

This dispatcher moves the cursor to a specified position
(x, y) where x starts from left to right, and y starts from top to bottom

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |
| 1 | `i64` |  |

###### `WorkspaceOption`

This dispatcher applied a option to all windows in a workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WorkspaceOptions` |  |

###### `RenameWorkspace`

This dispatcher renames a workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WorkspaceId` |  |
| 1 | `Option<&''a str>` |  |

###### `Exit`

This exits Hyprland **(DANGEROUS)**

###### `ForceRendererReload`

This dispatcher forces the renderer to reload

###### `MoveCurrentWorkspaceToMonitor`

This dispatcher moves the current workspace to a specified monitor

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MonitorIdentifier<''a>` |  |

###### `MoveWorkspaceToMonitor`

This dispatcher moves a specified workspace to a specified monitor

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WorkspaceIdentifier<''a>` |  |
| 1 | `MonitorIdentifier<''a>` |  |

###### `SwapActiveWorkspaces`

This dispatcher swaps the active workspaces of two monitors

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MonitorIdentifier<''a>` |  |
| 1 | `MonitorIdentifier<''a>` |  |

###### `BringActiveToTop`

This dispatcher brings the active window to the top of the stack

###### `AlterZOrder`

This dispatcher brings the active window to the top or bottom of the stack

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ZOrder` |  |
| 1 | `Option<WindowIdentifier<''a>>` |  |

###### `ToggleSpecialWorkspace`

This toggles the special workspace (AKA scratchpad)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<String>` |  |

###### `FocusUrgentOrLast`

This dispatcher jump to urgent or the last window

###### `FocusCurrentOrLast`

Switch focus from current to previously focused window

###### `ToggleSwallow`

Swallow or Unswallow a window

###### `Submap`

Change the current mapping group

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `SubmapParam<''a>` |  |

###### `ToggleSplit`

Toggles the split (top/side) of the current window. `preserve_split` must be enabled for toggling to work

###### `SwapSplit`

Swaps the two halves of the split of the current window

###### `PreSelect`

One-time override for the split direction (only works on tiled windows)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Direction` |  |

###### `MoveToRoot`

Moves the selected window (active window if unspecified) to the root of its workspace tree

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<WindowIdentifier<''a>>` |  |
| 1 | `MoveToRootParam` |  |

###### `SwapWithMaster`

Swaps the current window with master.
If the current window is the master,
swaps it with the first child.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `SwapWithMasterParam` |  |

###### `FocusMaster`

Focuses the master window.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `FocusMasterParam` |  |

###### `CycleNextMaster`

Focuses the next window respecting the layout

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MasterLoopParam` |  |

###### `CyclePrevMaster`

Focuses the previous window respecting the layout

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MasterLoopParam` |  |

###### `SwapNextMaster`

Swaps the focused window with the next window respecting the layout

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MasterLoopParam` |  |

###### `SwapPrevMaster`

Swaps the focused window with the previous window respecting the layout

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MasterLoopParam` |  |

###### `AddMaster`

Adds a master to the master side. That will be the active window,
if it’s not a master, or the first non-master window.

###### `RemoveMaster`

Removes a master from the master side. That will be the
active window, if it’s a master, or the last master window.

###### `OrientationLeft`

Sets the orientation for the current workspace to left
(master area left, slave windows to the right, vertically stacked)

###### `OrientationRight`

Sets the orientation for the current workspace to right
(master area right, slave windows to the left, vertically stacked)

###### `OrientationTop`

Sets the orientation for the current workspace to top
(master area top, slave windows to the bottom, horizontally stacked)

###### `OrientationBottom`

Sets the orientation for the current workspace to bottom
(master area bottom, slave windows to the top, horizontally stacked)

###### `OrientationCenter`

Sets the orientation for the current workspace to center
(master area center, slave windows alternate to the left and right, vertically stacked)

###### `OrientationNext`

Cycle to the next orientation for the current workspace (clockwise)

###### `OrientationPrev`

Cycle to the previous orientation for the current workspace (counter-clockwise)

###### `OrientationCycle`

Cycle to the next orientation from the provided list, for the current workspace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `OrientationParam` |  |

###### `Mfact`

Change mfact, the master split ratio

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `FloatValue` |  |

###### `RollNext`

Rotate the next window in stack to be the master, while keeping the focus on master

###### `RollPrev`

Rotate the previous window in stack to be the master, while keeping the focus on master

###### `ToggleGroup`

Toggles the current active window into a group

###### `ChangeGroupActive`

Switches to the next window in a group.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WindowSwitchDirection` |  |

###### `LockGroups`

Locks the groups

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `LockType` |  |

###### `LockActiveGroup`

Locks the currently focused group

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `LockType` |  |

###### `MoveIntoGroup`

Moves the active window into a group in a specified direction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Direction` |  |

###### `MoveWindowOrGroup`

Moves the active window into or out of a group in a specified direction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Direction` |  |

###### `MoveOutOfGroup`

Moves the active window out of a group.

###### `MoveGroupWindow`

Swaps the active window with the next or previous in a group

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WindowSwitchDirection` |  |

###### `DenyWindowFromGroup`

Prohibit the active window from becoming or being inserted into group

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `BinaryState` |  |

###### `SetIgnoreGroupLock`

Temporarily enable or disable ignore_group_lock

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `BinaryState` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DispatchType<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `BinaryState`

**Attributes:**

- `Other("#[allow(missing_docs)]")`

Enum used for options with a binary on/off state

```rust
pub enum BinaryState {
    On,
    Off,
    Toggle,
}
```

##### Variants

###### `On`

###### `Off`

###### `Toggle`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BinaryState { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &BinaryState) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BinaryState) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &BinaryState) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `LockType`

Enum used with [DispatchType::LockGroups], to determine how to lock/unlock

```rust
pub enum LockType {
    Lock,
    Unlock,
    ToggleLock,
}
```

##### Variants

###### `Lock`

Lock Group

###### `Unlock`

Unlock Group

###### `ToggleLock`

Toggle lock state of Group

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LockType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &LockType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LockType) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &LockType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `SwapWithMasterParam`

Param for [DispatchType::SwapWithMaster] dispatcher

```rust
pub enum SwapWithMasterParam {
    Master,
    Child,
    Auto,
    IgnoreMaster,
}
```

##### Variants

###### `Master`

New focus is the new master window

###### `Child`

New focus is the new child

###### `Auto`

Keep the focus of the previously focused window

###### `IgnoreMaster`

Noop if master is already focused

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SwapWithMasterParam { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SwapWithMasterParam) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `FocusMasterParam`

Param for [DispatchType::FocusMaster] dispatcher

```rust
pub enum FocusMasterParam {
    Master,
    Auto,
    Previous,
}
```

##### Variants

###### `Master`

Focus stays at master, (even if it was selected before)

###### `Auto`

If the current window is the master, focuses the first child

###### `Previous`

If the current window is the master, focuses the previously focused one

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FocusMasterParam { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FocusMasterParam) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `MasterLoopParam`

Param for some master layout dispatchers

```rust
pub enum MasterLoopParam {
    Loop,
    NoLoop,
}
```

##### Variants

###### `Loop`

Allow looping through the pile

###### `NoLoop`

Do not allow looping through the pile

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MasterLoopParam { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MasterLoopParam) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `OrientationParam`

Param for [DispatchType::OrientationCycle] dispatcher

```rust
pub enum OrientationParam {
    Left,
    Right,
    Bottom,
    Top,
    Center,
}
```

##### Variants

###### `Left`

Set orientation to left

###### `Right`

Set orientation to right

###### `Bottom`

Set orientation to bottom

###### `Top`

Set orientation to top

###### `Center`

Set orientation to center

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OrientationParam { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OrientationParam) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `FloatValue`

Param for split ratio changes

```rust
pub enum FloatValue {
    Relative(f32),
    Exact(f32),
}
```

##### Variants

###### `Relative`

Change relative to current factor

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f32` |  |

###### `Exact`

Set factor to exact value between 0 and 1

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f32` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FloatValue { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FloatValue) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Dispatch`

The struct that provides all dispatching methods

```rust
pub struct Dispatch;
```

##### Implementations

###### Methods

- ```rust
  pub fn call(dispatch_type: DispatchType<''_>) -> crate::Result<()> { /* ... */ }
  ```
  This function calls a specified dispatcher (blocking)

- ```rust
  pub fn instance_call(instance: &crate::instance::Instance, dispatch_type: DispatchType<''_>) -> crate::Result<()> { /* ... */ }
  ```
  This function calls a specified dispatcher (blocking)

- ```rust
  pub async fn call_async(dispatch_type: DispatchType<''_>) -> crate::Result<()> { /* ... */ }
  ```
  This function calls a specified dispatcher (async)

- ```rust
  pub async fn instance_call_async(instance: &crate::instance::Instance, dispatch_type: DispatchType<''_>) -> crate::Result<()> { /* ... */ }
  ```
  This function calls a specified dispatcher (async)

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `ctl`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"ctl\"), span: src/lib.rs:38:7: 38:22 (#0) }])]")`

This module is for calling hyprctl **commands**, for getting data use [data]

```rust
pub mod ctl { /* ... */ }
```

### Modules

## Module `reload`

Reload hyprland config

```rust
pub mod reload { /* ... */ }
```

### Functions

#### Function `call`

Reload hyprland config

```rust
pub fn call() -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call`

Reload hyprland config

```rust
pub fn instance_call(instance: &crate::instance::Instance) -> crate::Result<()> { /* ... */ }
```

#### Function `call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:24:15: 24:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:24:39: 24:56 (#0) }], src/ctl.rs:24:14: 24:57 (#0))])]")`

Reload hyprland config (async)

```rust
pub async fn call_async() -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:30:15: 30:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:30:39: 30:56 (#0) }], src/ctl.rs:30:14: 30:57 (#0))])]")`

Reload hyprland config (async)

```rust
pub async fn instance_call_async(instance: &crate::instance::Instance) -> crate::Result<()> { /* ... */ }
```

## Module `kill`

Enter kill mode (similar to xkill)

```rust
pub mod kill { /* ... */ }
```

### Functions

#### Function `call`

Enter kill mode (similar to xkill)

```rust
pub fn call() -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call`

Enter kill mode (similar to xkill)

```rust
pub fn instance_call(instance: &crate::instance::Instance) -> crate::Result<()> { /* ... */ }
```

#### Function `call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:54:15: 54:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:54:39: 54:56 (#0) }], src/ctl.rs:54:14: 54:57 (#0))])]")`

Enter kill mode (similar to xkill) (async)

```rust
pub async fn call_async() -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:60:15: 60:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:60:39: 60:56 (#0) }], src/ctl.rs:60:14: 60:57 (#0))])]")`

Enter kill mode (similar to xkill) (async)

```rust
pub async fn instance_call_async(instance: &crate::instance::Instance) -> crate::Result<()> { /* ... */ }
```

## Module `set_cursor`

Set the cursor theme

```rust
pub mod set_cursor { /* ... */ }
```

### Functions

#### Function `call`

Set the cursor theme

```rust
pub fn call<Str: FDisplay>(theme: Str, size: u16) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call`

Set the cursor theme

```rust
pub fn instance_call<Str: FDisplay>(instance: &crate::instance::Instance, theme: Str, size: u16) -> crate::Result<()> { /* ... */ }
```

#### Function `call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:89:15: 89:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:89:39: 89:56 (#0) }], src/ctl.rs:89:14: 89:57 (#0))])]")`

Set the cursor theme (async)

```rust
pub async fn call_async<Str: FDisplay>(theme: Str, size: u16) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:95:15: 95:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:95:39: 95:56 (#0) }], src/ctl.rs:95:14: 95:57 (#0))])]")`

Set the cursor theme (async)

```rust
pub async fn instance_call_async<Str: FDisplay>(instance: &crate::instance::Instance, theme: Str, size: u16) -> crate::Result<()> { /* ... */ }
```

## Module `output`

Stuff related to managing virtual outputs/displays

```rust
pub mod output { /* ... */ }
```

### Types

#### Enum `OutputBackends`

Output backend types

```rust
pub enum OutputBackends {
    Wayland,
    X11,
    Headless,
    Auto,
}
```

##### Variants

###### `Wayland`

The wayland output backend

###### `X11`

The x11 output backend

###### `Headless`

The headless output backend

###### `Auto`

Let Hyprland decide the backend type

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OutputBackends { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OutputBackends) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `create`

Create virtual displays

```rust
pub fn create(backend: OutputBackends, name: Option<&str>) -> crate::Result<()> { /* ... */ }
```

#### Function `remove`

Remove virtual displays

```rust
pub fn remove<Str: FDisplay>(name: Str) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_create`

Create virtual displays

```rust
pub fn instance_create(instance: &crate::instance::Instance, backend: OutputBackends, name: Option<&str>) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_remove`

Remove virtual displays

```rust
pub fn instance_remove<Str: FDisplay>(instance: &crate::instance::Instance, name: Str) -> crate::Result<()> { /* ... */ }
```

#### Function `create_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:157:15: 157:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:157:39: 157:56 (#0) }], src/ctl.rs:157:14: 157:57 (#0))])]")`

Create virtual displays

```rust
pub async fn create_async(backend: OutputBackends, name: Option<&str>) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_create_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:163:15: 163:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:163:39: 163:56 (#0) }], src/ctl.rs:163:14: 163:57 (#0))])]")`

Create virtual displays

```rust
pub async fn instance_create_async(instance: &crate::instance::Instance, backend: OutputBackends, name: Option<&str>) -> crate::Result<()> { /* ... */ }
```

#### Function `remove_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:177:15: 177:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:177:39: 177:56 (#0) }], src/ctl.rs:177:14: 177:57 (#0))])]")`

Remove virtual displays

```rust
pub async fn remove_async<Str: FDisplay>(name: Str) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_remove_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:183:15: 183:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:183:39: 183:56 (#0) }], src/ctl.rs:183:14: 183:57 (#0))])]")`

Remove virtual displays

```rust
pub async fn instance_remove_async<Str: FDisplay>(instance: &crate::instance::Instance, name: Str) -> crate::Result<()> { /* ... */ }
```

## Module `switch_xkb_layout`

Switch the xkb layout index for a keyboard

```rust
pub mod switch_xkb_layout { /* ... */ }
```

### Types

#### Enum `SwitchXKBLayoutCmdTypes`

The types of Cmds used by [switch_xkb_layout]

```rust
pub enum SwitchXKBLayoutCmdTypes {
    Next,
    Previous,
    Id(u8),
}
```

##### Variants

###### `Next`

Next input

###### `Previous`

Previous inout

###### `Id`

Set to a specific input id

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SwitchXKBLayoutCmdTypes { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SwitchXKBLayoutCmdTypes) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `call`

Switch the xkb layout index for a keyboard

```rust
pub fn call<Str: FDisplay>(device: Str, cmd: SwitchXKBLayoutCmdTypes) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call`

Switch the xkb layout index for a keyboard

```rust
pub fn instance_call<Str: FDisplay>(instance: &crate::instance::Instance, device: Str, cmd: SwitchXKBLayoutCmdTypes) -> crate::Result<()> { /* ... */ }
```

#### Function `call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:229:15: 229:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:229:39: 229:56 (#0) }], src/ctl.rs:229:14: 229:57 (#0))])]")`

Switch the xkb layout index for a keyboard

```rust
pub async fn call_async<Str: FDisplay>(instance: &crate::instance::Instance, device: Str, cmd: SwitchXKBLayoutCmdTypes) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:239:15: 239:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:239:39: 239:56 (#0) }], src/ctl.rs:239:14: 239:57 (#0))])]")`

Switch the xkb layout index for a keyboard

```rust
pub async fn instance_call_async<Str: FDisplay>(instance: &crate::instance::Instance, device: Str, cmd: SwitchXKBLayoutCmdTypes) -> crate::Result<()> { /* ... */ }
```

## Module `set_error`

Creates a error that Hyprland will display

```rust
pub mod set_error { /* ... */ }
```

### Functions

#### Function `call`

Creates a error that Hyprland will display

```rust
pub fn call(color: Color, msg: String) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call`

Creates a error that Hyprland will display

```rust
pub fn instance_call(instance: &crate::instance::Instance, color: Color, msg: String) -> crate::Result<()> { /* ... */ }
```

#### Function `call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:268:15: 268:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:268:39: 268:56 (#0) }], src/ctl.rs:268:14: 268:57 (#0))])]")`

Creates a error that Hyprland will display (async)

```rust
pub async fn call_async(color: Color, msg: String) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:274:15: 274:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:274:39: 274:56 (#0) }], src/ctl.rs:274:14: 274:57 (#0))])]")`

Creates a error that Hyprland will display (async)

```rust
pub async fn instance_call_async(instance: &crate::instance::Instance, color: Color, msg: String) -> crate::Result<()> { /* ... */ }
```

## Module `notify`

Creates a notification with Hyprland

```rust
pub mod notify { /* ... */ }
```

### Types

#### Enum `Icon`

**Attributes:**

- `Other("#[allow(missing_docs)]")`
- `Repr(AttributeRepr { kind: Rust, align: None, packed: None, int: Some("i8") })`

```rust
pub enum Icon {
    NoIcon = -1,
    Warning = 0,
    Info = 1,
    Hint = 2,
    Error = 3,
    Confused = 4,
    Ok = 5,
}
```

##### Variants

###### `NoIcon`

Discriminant: `-1`

Discriminant value: `-1`

###### `Warning`

Discriminant: `0`

Discriminant value: `0`

###### `Info`

Discriminant: `1`

Discriminant value: `1`

###### `Hint`

Discriminant: `2`

Discriminant value: `2`

###### `Error`

Discriminant: `3`

Discriminant value: `3`

###### `Confused`

Discriminant: `4`

Discriminant value: `4`

###### `Ok`

Discriminant: `5`

Discriminant value: `5`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Icon { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Icon) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `call`

Creates a notification with Hyprland

```rust
pub fn call(icon: Icon, time: std::time::Duration, color: Color, msg: String) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call`

Creates a notification with Hyprland

```rust
pub fn instance_call(instance: &crate::instance::Instance, icon: Icon, time: std::time::Duration, color: Color, msg: String) -> crate::Result<()> { /* ... */ }
```

#### Function `call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:332:15: 332:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:332:39: 332:56 (#0) }], src/ctl.rs:332:14: 332:57 (#0))])]")`

Creates a error that Hyprland will display (async)

```rust
pub async fn call_async(icon: Icon, time: std::time::Duration, color: Color, msg: String) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:343:15: 343:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:343:39: 343:56 (#0) }], src/ctl.rs:343:14: 343:57 (#0))])]")`

Creates a error that Hyprland will display (async)

```rust
pub async fn instance_call_async(instance: &crate::instance::Instance, icon: Icon, time: std::time::Duration, color: Color, msg: String) -> crate::Result<()> { /* ... */ }
```

## Module `dismissnotify`

Dismisses all or up to a specified amount of notifications with Hyprland

```rust
pub mod dismissnotify { /* ... */ }
```

### Functions

#### Function `call`

Dismisses notifications with Hyprland

If `amount` is [None] then will dismiss ALL notifications

```rust
pub fn call(amount: Option<std::num::NonZeroU8>) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call`

Dismisses notifications with Hyprland

If `amount` is [None] then will dismiss ALL notifications

```rust
pub fn instance_call(instance: &crate::instance::Instance, amount: Option<std::num::NonZeroU8>) -> crate::Result<()> { /* ... */ }
```

#### Function `call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:395:15: 395:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:395:39: 395:56 (#0) }], src/ctl.rs:395:14: 395:57 (#0))])]")`

Dismisses notifications with Hyprland (async)

If `amount` is [None] then will dismiss ALL notifications

```rust
pub async fn call_async(amount: Option<std::num::NonZeroU8>) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:403:15: 403:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:403:39: 403:56 (#0) }], src/ctl.rs:403:14: 403:57 (#0))])]")`

Dismisses notifications with Hyprland (async)

If `amount` is [None] then will dismiss ALL notifications

```rust
pub async fn instance_call_async(instance: &crate::instance::Instance, amount: Option<std::num::NonZeroU8>) -> crate::Result<()> { /* ... */ }
```

## Module `set_prop`

Provides things to setting props

```rust
pub mod set_prop { /* ... */ }
```

### Types

#### Enum `PropType`

Type that represents a prop

```rust
pub enum PropType {
    AnimationStyle(String),
    Rounding(i64, bool),
    ForceNoBlur(bool, bool),
    ForceOpaque(bool, bool),
    ForceOpaqueOverriden(bool, bool),
    ForceAllowsInput(bool, bool),
    ForceNoAnims(bool, bool),
    ForceNoBorder(bool, bool),
    ForceNoShadow(bool, bool),
    WindowDanceCompat(bool, bool),
    NoMaxSize(bool, bool),
    DimAround(bool, bool),
    AlphaOverride(bool, bool),
    Alpha(f32, bool),
    AlphaInactiveOverride(bool, bool),
    AlphaInactive(f32, bool),
    ActiveBorderColor(Color, bool),
    InactiveBorderColor(Color, bool),
}
```

##### Variants

###### `AnimationStyle`

The animation style

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Rounding`

The roundness

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |
| 1 | `bool` | locked |

###### `ForceNoBlur`

Force no blur

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `ForceOpaque`

Force opaque

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `ForceOpaqueOverriden`

Force opaque overriden

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `ForceAllowsInput`

Force allow input

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `ForceNoAnims`

Force no animations

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `ForceNoBorder`

Force no border

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `ForceNoShadow`

Force no shadow

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `WindowDanceCompat`

Allow for windoe dancing?

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `NoMaxSize`

Allow for overstepping max size

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `DimAround`

Dim around?

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `AlphaOverride`

Makes the next setting be override instead of multiply

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `Alpha`

The alpha

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f32` |  |
| 1 | `bool` | locked |

###### `AlphaInactiveOverride`

Makes the next setting be override instead of multiply

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |
| 1 | `bool` | locked |

###### `AlphaInactive`

The alpha for inactive

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f32` |  |
| 1 | `bool` | locked |

###### `ActiveBorderColor`

The active border color

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Color` |  |
| 1 | `bool` | locked |

###### `InactiveBorderColor`

The inactive border color

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Color` |  |
| 1 | `bool` | locked |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PropType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PropType) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `call`

Sets a window prob

```rust
pub fn call(ident: String, prop: PropType, lock: bool) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call`

Sets a window prob

```rust
pub fn instance_call(instance: &crate::instance::Instance, ident: String, prop: PropType, lock: bool) -> crate::Result<()> { /* ... */ }
```

#### Function `call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:588:15: 588:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:588:39: 588:56 (#0) }], src/ctl.rs:588:14: 588:57 (#0))])]")`

Sets a window prob (async)

```rust
pub async fn call_async(ident: String, prop: PropType, lock: bool) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_call_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:594:15: 594:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:594:39: 594:56 (#0) }], src/ctl.rs:594:14: 594:57 (#0))])]")`

Sets a window prob (async)

```rust
pub async fn instance_call_async(instance: &crate::instance::Instance, ident: String, prop: PropType, lock: bool) -> crate::Result<()> { /* ... */ }
```

## Module `plugin`

Provides functions for communication with plugin system

```rust
pub mod plugin { /* ... */ }
```

### Types

#### Struct `Plugin`

This struct represents a loaded plugin

```rust
pub struct Plugin {
    pub name: String,
    pub author: String,
    pub handle: String,
    pub version: String,
    pub description: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | plugin name |
| `author` | `String` | plugin author |
| `handle` | `String` | handle to plugin |
| `version` | `String` | plugin version |
| `description` | `String` | plugin description |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Plugin { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Plugin) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `list`

Returns a list of all plugins

```rust
pub fn list() -> crate::Result<Vec<Plugin>> { /* ... */ }
```

#### Function `instance_list`

Returns a list of all plugins

```rust
pub fn instance_list(instance: &crate::instance::Instance) -> crate::Result<Vec<Plugin>> { /* ... */ }
```

#### Function `list_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:646:15: 646:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:646:39: 646:56 (#0) }], src/ctl.rs:646:14: 646:57 (#0))])]")`

Returns a list of all plugins (async)

```rust
pub async fn list_async() -> crate::Result<Vec<Plugin>> { /* ... */ }
```

#### Function `instance_list_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:652:15: 652:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:652:39: 652:56 (#0) }], src/ctl.rs:652:14: 652:57 (#0))])]")`

Returns a list of all plugins (async)

```rust
pub async fn instance_list_async(instance: &crate::instance::Instance) -> crate::Result<Vec<Plugin>> { /* ... */ }
```

#### Function `load`

Loads a plugin, by absolute path

```rust
pub fn load(path: &std::path::Path) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_load`

Loads a plugin, by absolute path

```rust
pub fn instance_load(instance: &crate::instance::Instance, path: &std::path::Path) -> crate::Result<()> { /* ... */ }
```

#### Function `load_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:677:15: 677:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:677:39: 677:56 (#0) }], src/ctl.rs:677:14: 677:57 (#0))])]")`

Loads a plugin, by absolute path (async)

```rust
pub async fn load_async(path: &std::path::Path) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_load_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:683:15: 683:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:683:39: 683:56 (#0) }], src/ctl.rs:683:14: 683:57 (#0))])]")`

Loads a plugin, by absolute path (async)

```rust
pub async fn instance_load_async(instance: &crate::instance::Instance, path: &std::path::Path) -> crate::Result<()> { /* ... */ }
```

#### Function `unload`

Unloads a plugin, by absolute path.

```rust
pub fn unload(path: &std::path::Path) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_unload`

Unloads a plugin, by absolute path.

```rust
pub fn instance_unload(instance: &crate::instance::Instance, path: &std::path::Path) -> crate::Result<()> { /* ... */ }
```

#### Function `unload_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:711:15: 711:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:711:39: 711:56 (#0) }], src/ctl.rs:711:14: 711:57 (#0))])]")`

Unloads a plugin, by absolute path (async)

```rust
pub async fn unload_async(path: &std::path::Path) -> crate::Result<()> { /* ... */ }
```

#### Function `instance_unload_async`

**Attributes:**

- `Other("#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"async-lite\"), span: src/ctl.rs:717:15: 717:37 (#0) }, NameValue { name: \"feature\", value: Some(\"tokio\"), span: src/ctl.rs:717:39: 717:56 (#0) }], src/ctl.rs:717:14: 717:57 (#0))])]")`

Unloads a plugin, by absolute path (async)

```rust
pub async fn instance_unload_async(instance: &crate::instance::Instance, path: &std::path::Path) -> crate::Result<()> { /* ... */ }
```

## Module `instance`

This module allows listing running hyprland instances

```rust
pub mod instance { /* ... */ }
```

### Types

#### Struct `Instance`

This struct represents a running Hyprland instance

```rust
pub struct Instance {
    pub instance: String,
    pub time: u64,
    pub pid: u32,
    pub wl_socket: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `instance` | `String` | instance name (9958d29...) in /run/user/$UID/hypr/$instance |
| `time` | `u64` | ??? |
| `pid` | `u32` | pid of hyprland process |
| `wl_socket` | `String` | name of wayland socket in /run/user/$UID/$wl_socket |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Instance { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Instance) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `instance_list`

Returns a list of running instances

```rust
pub fn instance_list() -> crate::Result<Vec<Instance>> { /* ... */ }
```

### Types

#### Struct `Color`

**Attributes:**

- `Other("#[display(\"rgba({_0:02x}{_1:02x}{_2:02x}{_3:02x})\")]")`

A 8-bit color with a alpha channel

```rust
pub struct Color(/* private field */, /* private field */, /* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |
| 2 | `private` | *Private field* |
| 3 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(__0: u8, __1: u8, __2: u8, __3: u8) -> Color { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Color { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Color) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `keyword`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"keyword\"), span: src/lib.rs:42:7: 42:26 (#0) }])]")`

This module provides the stuff needed to mutate, and read Hyprland config values
# Keyword module

This module is used for setting, and getting keywords

## Usage

```rust, no_run
use hyprland::keyword::Keyword;
fn main() -> hyprland::Result<()> {
    Keyword::get("some_keyword")?;
    Keyword::set("another_keyword", "the value to set it to")?;

    Ok(())
}
```

```rust
pub mod keyword { /* ... */ }
```

### Types

#### Struct `HyprColor`

**Attributes:**

- `Repr(AttributeRepr { kind: Rust, align: None, packed: Some(1), int: None })`

A Color made up of rgba values (0-255)

```rust
pub struct HyprColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `r` | `u8` | Red Channel (0-255) |
| `g` | `u8` | Green Channel (0-255) |
| `b` | `u8` | Blue Channel (0-255) |
| `a` | `u8` | Alpha (0-255) |

##### Implementations

###### Methods

- ```rust
  pub fn from_argb_u32(i: u32) -> Self { /* ... */ }
  ```
  Convert an AARRGGBB u32 value to a HyprColor

- ```rust
  pub fn try_from_argb_str(s: &str) -> Option<Self> { /* ... */ }
  ```
  Try to convert from &str in the argb legacy format to HyprColor,

- ```rust
  pub fn try_from_rgba_str(s: &str) -> Option<Self> { /* ... */ }
  ```
  Try to convert from &str in the rgba format to HyprColor,

- ```rust
  pub fn try_from_rgb_str(s: &str) -> Option<Self> { /* ... */ }
  ```
  Try to convert from &str in the rgb format to HyprColor,

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HyprColor { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(s: &str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `HyprGradient`

A Gradiant made up of HyprColor(s) and an angle

```rust
pub struct HyprGradient {
    pub color0: HyprColor,
    pub color1: Option<HyprColor>,
    pub angle: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `color0` | `HyprColor` | First gradiant color |
| `color1` | `Option<HyprColor>` | Second gradiant color |
| `angle` | `u32` | Angle in degrees |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HyprGradient { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(s: &str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `HyprRect`

Bounds used for custom Rects

```rust
pub struct HyprRect {
    pub top: i64,
    pub right: i64,
    pub bottom: i64,
    pub left: i64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `top` | `i64` | Bound Top |
| `right` | `i64` | Bound Right |
| `bottom` | `i64` | Bound Bottom |
| `left` | `i64` | Bound Left |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HyprRect { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(s: &str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `Custom`

A parseable value type for custom options

```rust
pub enum Custom {
    HyprColor(HyprColor),
    HyprGradient(HyprGradient),
    HyprRect(HyprRect),
}
```

##### Variants

###### `HyprColor`

Color Variant for Custom field

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `HyprColor` |  |

###### `HyprGradient`

Gradiant Variant for Custom field

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `HyprGradient` |  |

###### `HyprRect`

A general rect made of top, right, bottom, left

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `HyprRect` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Custom { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(s: &str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `OptionValue`

This enum holds the possible values of a keyword/option

```rust
pub enum OptionValue {
    Int(i64),
    Float(f64),
    String(String),
    Custom(Custom),
    Vec2([i64; 2]),
    Unknown(String),
}
```

##### Variants

###### `Int`

A integer (64-bit)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `Float`

A floating point (64-point)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f64` |  |

###### `String`

A string

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Custom`

A hyprland Color or Gradiant

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Custom` |  |

###### `Vec2`

A Vector of 2 ints

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[i64; 2]` |  |

###### `Unknown`

Could not parse value

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OptionValue { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(opt: OptionValue) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(str: Str) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Keyword`

This struct holds a keyword

```rust
pub struct Keyword {
    pub option: String,
    pub value: OptionValue,
    pub set: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `option` | `String` | The identifier (or name) of the keyword |
| `value` | `OptionValue` | The value of the keyword/option |
| `set` | `bool` | Is value overriden or not |

##### Implementations

###### Methods

- ```rust
  pub fn set<Str: ToString, Opt: Into<OptionValue>>(key: Str, value: Opt) -> crate::Result<()> { /* ... */ }
  ```
  This function sets a keyword's value

- ```rust
  pub fn instance_set<Str: ToString, Opt: Into<OptionValue>>(instance: &Instance, key: Str, value: Opt) -> crate::Result<()> { /* ... */ }
  ```
  This function sets a keyword's value

- ```rust
  pub async fn set_async<Str: ToString, Opt: Into<OptionValue>>(key: Str, value: Opt) -> crate::Result<()> { /* ... */ }
  ```
  This function sets a keyword's value (async)

- ```rust
  pub async fn instance_set_async<Str: ToString, Opt: Into<OptionValue>>(instance: &Instance, key: Str, value: Opt) -> crate::Result<()> { /* ... */ }
  ```
  This function sets a keyword's value (async)

- ```rust
  pub fn get<Str: ToString>(key: Str) -> crate::Result<Self> { /* ... */ }
  ```
  This function returns the value of a keyword

- ```rust
  pub fn instance_get<Str: ToString>(instance: &Instance, key: Str) -> crate::Result<Self> { /* ... */ }
  ```
  This function returns the value of a keyword

- ```rust
  pub async fn get_async<Str: ToString>(key: Str) -> crate::Result<Self> { /* ... */ }
  ```
  This function returns the value of a keyword (async)

- ```rust
  pub async fn instance_get_async<Str: ToString>(instance: &Instance, key: Str) -> crate::Result<Self> { /* ... */ }
  ```
  This function returns the value of a keyword (async)

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Keyword { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `config`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"config\"), span: src/lib.rs:46:7: 46:25 (#0) }])]")`

This module provides helpers to easily config Hyprland
# Hyprland Configuration in Rust


```rust
pub mod config { /* ... */ }
```

### Modules

## Module `binds`

Module providing stuff for adding an removing keybinds

```rust
pub mod binds { /* ... */ }
```

### Types

#### Enum `Key`

Type for a key held by a bind

```rust
pub enum Key<''a> {
    Mod(&''a [Mod], &''a str),
    Key(&''a str),
}
```

##### Variants

###### `Mod`

Variant for if the bind holds a modded key

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a [Mod]` | Mods |
| 1 | `&''a str` | Key |

###### `Key`

Variant for a regular key

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Key<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Key<''a>) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `Mod`

**Attributes:**

- `Other("#[allow(missing_docs)]")`

Enum for mod keys used in bind combinations

```rust
pub enum Mod {
    SUPER,
    SHIFT,
    ALT,
    CTRL,
    NONE,
}
```

##### Variants

###### `SUPER`

###### `SHIFT`

###### `ALT`

###### `CTRL`

###### `NONE`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Mod { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Mod) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `Flag`

**Attributes:**

- `Other("#[allow(non_camel_case_types)]")`

Enum for bind flags

```rust
pub enum Flag {
    l,
    r,
    e,
    n,
    m,
    t,
    i,
    s,
    d,
    p,
}
```

##### Variants

###### `l`

Works when screen is locked

###### `r`

Activates on release

###### `e`

Repeats when held

###### `n`

Non-consuming, key/mouse events will be passed to the active window in addition to triggering the dispatcher.

###### `m`

Used for mouse binds

###### `t`

Transparent, cannot be shadowed by other binds.

###### `i`

Ignore mods, will ignore modifiers.

###### `s`

Separate, will arbitrarily combine keys between each mod/key

###### `d`

Has description, will allow you to write a description for your bind.

###### `p`

Bypasses the app's requests to inhibit keybinds.

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Flag { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Flag) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `PartialBind`

A struct used for indentifying bindings

```rust
pub struct PartialBind<''a> {
    pub mods: &''a [Mod],
    pub key: Key<''a>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `mods` | `&''a [Mod]` | The modifiers used |
| `key` | `Key<''a>` | The main key used |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PartialBind<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Binding`

A struct providing a key bind

```rust
pub struct Binding<''a> {
    pub mods: &''a [Mod],
    pub key: Key<''a>,
    pub flags: &''a [Flag],
    pub dispatcher: crate::dispatch::DispatchType<''a>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `mods` | `&''a [Mod]` | All the mods |
| `key` | `Key<''a>` | The key |
| `flags` | `&''a [Flag]` | Bind flags |
| `dispatcher` | `crate::dispatch::DispatchType<''a>` | The dispatcher to be called once complete |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Binding<''a> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Binder`

Struct to hold methods for adding and removing binds

```rust
pub struct Binder;
```

##### Implementations

###### Methods

- ```rust
  pub fn bind(binding: Binding<''_>) -> crate::Result<()> { /* ... */ }
  ```
  Binds a keybinding

- ```rust
  pub fn unbind(binding: PartialBind<''_>) -> crate::Result<()> { /* ... */ }
  ```
  Unbinds a keybinding

- ```rust
  pub fn instance_unbind(instance: &Instance, binding: PartialBind<''_>) -> crate::Result<()> { /* ... */ }
  ```
  Unbinds a keybinding

- ```rust
  pub async fn unbind_async(binding: PartialBind<''_>) -> crate::Result<()> { /* ... */ }
  ```
  Unbinds a keybinding (async)

- ```rust
  pub async fn instance_unbind_async(instance: &Instance, binding: PartialBind<''_>) -> crate::Result<()> { /* ... */ }
  ```
  Unbinds a keybinding (async)

- ```rust
  pub fn instance_bind(instance: &Instance, binding: Binding<''_>) -> crate::Result<()> { /* ... */ }
  ```
  Binds a keybinding

- ```rust
  pub async fn bind_async(binding: Binding<''_>) -> crate::Result<()> { /* ... */ }
  ```
  Binds a keybinding (async)

- ```rust
  pub async fn instance_bind_async(instance: &Instance, binding: Binding<''_>) -> crate::Result<()> { /* ... */ }
  ```
  Binds a keybinding (async)

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `error`

Holds the error type used throughout the crate

```rust
pub mod error { /* ... */ }
```

### Types

#### Enum `HyprError`

Error that unifies different error types used by Hyprland-rs

```rust
pub enum HyprError {
    SerdeError(serde_json::Error),
    IoError(io::Error),
    InvalidHyprColorFormat,
    InvalidHyprGradiantFormat,
    FromUtf8Error(std::string::FromUtf8Error),
    NotOkDispatch(String),
    Hyprpaper(crate::hyprpaper::Error),
    InvalidOptionKey(String),
    InvalidOptionValue,
    Internal(String),
    Other(String),
}
```

##### Variants

###### `SerdeError`

Error coming from serde

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `serde_json::Error` |  |

###### `IoError`

Error coming from std::io

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `io::Error` |  |

###### `InvalidHyprColorFormat`

Error from failing to parse HyprColor

###### `InvalidHyprGradiantFormat`

Error from failing to parse HyprGradient

###### `FromUtf8Error`

Error that occurs when parsing UTF-8 string

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::string::FromUtf8Error` |  |

###### `NotOkDispatch`

Dispatcher returned non `ok` value

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Hyprpaper`

Error when interacting with Hyprpaper.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::hyprpaper::Error` |  |

###### `InvalidOptionKey`

Keyword does not exist

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `InvalidOptionValue`

Unparsable Option

###### `Internal`

Internal Hyprland error

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Other`

Error that occurs for other reasons. Avoid using this.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn try_as_cloned(self: &Self) -> Result<Self, &Self> { /* ... */ }
  ```
  Try to get an owned version of the internal error.

- ```rust
  pub fn other<S: Into<String>>(other: S) -> Self { /* ... */ }
  ```
  Create a Hyprland error with dynamic data.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __derive_more_f: &mut derive_more::core::fmt::Formatter<''_>) -> derive_more::core::fmt::Result { /* ... */ }
    ```

- **Error**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(error: io::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(error: serde_json::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(error: std::string::FromUtf8Error) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `instance`

Used to generate the Instances to interface with Hyprland

```rust
pub mod instance { /* ... */ }
```

### Types

#### Struct `Instance`

This is the sync version of the Hyprland Instance.
It holds the event streams connected to the sockets of one running Hyprland instance.

```rust
pub struct Instance {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn from_current_env() -> crate::Result<Self> { /* ... */ }
  ```
  uses the $HYPRLAND_INSTANCE_SIGNATURE env variable

- ```rust
  pub fn from_instance(name: String) -> crate::Result<Self> { /* ... */ }
  ```
  Uses the name to determine the sockets to use

- ```rust
  pub fn from_base_socket_path(path: PathBuf) -> crate::Result<Self> { /* ... */ }
  ```
  Uses the path to determine the sockets to use

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Instance { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `hyprpaper`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"hyprpaper\"), span: src/lib.rs:57:7: 57:28 (#0) }])]")`

This module is for interacting with [hyprpaper] using its IPC feature

[hyprpaper]: https://wiki.hyprland.org/Hypr-Ecosystem/hyprpaper/

```rust
pub mod hyprpaper { /* ... */ }
```

### Types

#### Enum `Response`

Response from hyprpaper.

```rust
pub enum Response {
    Ok,
    ActiveWallpapers(Vec<WallpaperListing>),
    LoadedWallpapers(Vec<String>),
}
```

##### Variants

###### `Ok`

Keyword was accepted.

###### `ActiveWallpapers`

A list of active wallpapers.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<WallpaperListing>` |  |

###### `LoadedWallpapers`

A list of loaded wallpapers.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<String>` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `hyprpaper`

Send a keyword to hyprpaper using IPC.

```rust
pub fn hyprpaper(keyword: Keyword) -> crate::Result<Response> { /* ... */ }
```

#### Function `instance_hyprpaper`

Send a keyword to hyprpaper using IPC.

```rust
pub fn instance_hyprpaper(instance: &crate::instance::Instance, keyword: Keyword) -> crate::Result<Response> { /* ... */ }
```

#### Function `hyprpaper_async`

Send a keyword to hyprpaper using IPC.

```rust
pub async fn hyprpaper_async(keyword: Keyword) -> crate::Result<Response> { /* ... */ }
```

#### Function `instance_hyprpaper_async`

Send a keyword to hyprpaper using IPC.

```rust
pub async fn instance_hyprpaper_async(instance: &crate::instance::Instance, keyword: Keyword) -> crate::Result<Response> { /* ... */ }
```

### Re-exports

#### Re-export `Error`

```rust
pub use error::Error;
```

#### Re-export `Keyword`

```rust
pub use keyword::Keyword;
```

#### Re-export `Monitor`

```rust
pub use monitor::Monitor;
```

#### Re-export `Preload`

```rust
pub use preload::Preload;
```

#### Re-export `Reload`

```rust
pub use reload::Reload;
```

#### Re-export `Unload`

```rust
pub use unload::Unload;
```

#### Re-export `Wallpaper`

```rust
pub use wallpaper::Wallpaper;
```

#### Re-export `WallpaperListing`

```rust
pub use wallpaper_listing::WallpaperListing;
```

#### Re-export `WallpaperMode`

```rust
pub use wallpaper_mode::WallpaperMode;
```

## Module `prelude`

The prelude module, this is to import all traits

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `HyprData`

```rust
pub use crate::shared::HyprData;
```

#### Re-export `HyprDataActive`

```rust
pub use crate::shared::HyprDataActive;
```

#### Re-export `HyprDataActiveOptional`

```rust
pub use crate::shared::HyprDataActiveOptional;
```

#### Re-export `HyprDataVec`

```rust
pub use crate::shared::HyprDataVec;
```

#### Re-export `async_closure`

```rust
pub use hyprland_macros::async_closure;
```

## Types

### Type Alias `Result`

This type provides the result type used everywhere in Hyprland-rs

```rust
pub type Result<T> = std::result::Result<T, crate::error::HyprError>;
```

## Functions

### Function `default_instance`

Returns the result of the DEFAULT_INSTANCE OnceLock

```rust
pub fn default_instance() -> std::result::Result<&''static crate::instance::Instance, crate::error::HyprError> { /* ... */ }
```

### Function `default_instance_panic`

Returns the result of the DEFAULT_INSTANCE OnceLock

```rust
pub fn default_instance_panic() -> &''static crate::instance::Instance { /* ... */ }
```

## Macros

### Macro `command`

**Attributes:**

- `MacroExport`

Creates a `CommandContent` instance with the given flag and formatted data.

# Arguments

* `$flag` - A `CommandFlag` variant (`JSON` or `Empty`) that represents the flag for the command.
* `$($k:tt)*` - A format string and its arguments to be used as the data in the `CommandContent` instance.

```rust
pub macro_rules! command {
    /* macro_rules! command {
    ($flag:ident, $($k:tt)*) => { ... };
} */
}
```

### Macro `dispatch`

**Attributes:**

- `MacroExport`

Macro abstraction over [Dispatch::call]

```rust
pub macro_rules! dispatch {
    /* macro_rules! dispatch {
    (async; $dis:ident) => { ... };
    (async; $dis:ident, $( $arg:expr ), *) => { ... };
    (async; $instance:expr; $dis:ident) => { ... };
    (async; $instance:expr; $dis:ident, $( $arg:expr ), *) => { ... };
    ($dis:ident) => { ... };
    ($dis:ident, $( $arg:expr ), *) => { ... };
    ($instance:expr; $dis:ident) => { ... };
    ($instance:expr; $dis:ident, $( $arg:expr ), *) => { ... };
} */
}
```

### Macro `bind_raw`

**Attributes:**

- `MacroExport`

Very macro basic abstraction over [Binder] for internal use, **Dont use this instead use [crate::bind]**

```rust
# use hyprland::{bind_raw, default_instance, default_instance_panic, dispatch::DispatchType};
# async fn test() {
  let instance = default_instance()?;
  bind_raw!(instance , vec! [ Mod :: SHIFT ] , Key :: Key ( "m"  ) ,  vec ! [ Flag :: l , Flag :: r , Flag :: m ] ,  DispatchType :: Exit )?;
  bind_raw!(vec! [ Mod :: SHIFT ] , Key :: Key ( "m"  ) ,  vec ! [ Flag :: l , Flag :: r , Flag :: m ] ,  DispatchType :: Exit )?;
  bind_raw!(async, instance, vec ! [ Mod :: SHIFT ] , Key :: Key ( "m"  ) ,  vec ! [ Flag :: l , Flag :: r , Flag :: m ] ,  DispatchType :: Exit ).await?;
  bind_raw!(async, vec ! [ Mod :: SHIFT ] , Key :: Key ( "m"  ) ,  vec ! [ Flag :: l , Flag :: r , Flag :: m ] ,  DispatchType :: Exit ).await?;
# }
```

```rust
pub macro_rules! bind_raw {
    /* macro_rules! bind_raw {
    (async, $instance:expr,$mods:expr,$key:expr,$flags:expr,$dis:expr ) => { ... };
    (async, $mods:expr,$key:expr,$flags:expr,$dis:expr ) => { ... };
    ($instance:expr,$mods:expr,$key:expr,$flags:expr,$dis:expr ) => { ... };
    ($mods:expr,$key:expr,$flags:expr,$dis:expr ) => { ... };
} */
}
```

### Macro `bind`

**Attributes:**

- `MacroExport`

Macro abstraction over [Binder]

```rust
# use hyprland::{bind, default_instance_panic};
# use hyprland::instance::Instance;

# async fn test() {
    let instance = default_instance()?;
    bind!(instance, l r m | SHIFT, Key, "m" => Exit);
    bind!(SHIFT ALT, Key, "b" => CenterWindow);
    bind!(async ; l r m | SHIFT, Key, "m" => Exit);
    bind!(async ; instance,  SUPER, Mod, vec![Mod::SUPER], "l" => CenterWindow);
    bind!(async ; SHIFT ALT, Key, "b" => CenterWindow);
# }
```

```rust
pub macro_rules! bind {
    /* macro_rules! bind {
    (async ; $instance:expr, $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident, $( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => { ... };
    (async ; $instance:expr, $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => { ... };
    (async ; $instance:expr, $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => { ... };
    (async ; $instance:expr, $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => { ... };
    (async ; $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident, $( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => { ... };
    (async ; $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => { ... };
    (async ; $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => { ... };
    (async ; $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => { ... };
    ($instance:expr, $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident, $( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => { ... };
    ($instance:expr, $( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => { ... };
    ($instance:expr, $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => { ... };
    ($instance:expr, $( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => { ... };
    ($( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident, $( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => { ... };
    ($( $flag:ident ) *|$( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => { ... };
    ($( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident, $( $arg:expr ), *) => { ... };
    ($( $mod:ident ) *,$keyt:ident,$( $key:expr ), * => $dis:ident ) => { ... };
} */
}
```

