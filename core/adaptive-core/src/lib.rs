//! Foundational types for Xdaptive.
//!
//! The identifiers and the Workspace identity that the rest of the workspace
//! shares. The model follows `docs/03-workspace.md`; only the parts the
//! specification already names are defined here, and each type is added as its
//! owning document settles it.

use core::fmt;

/// Defines an opaque newtype identifier over `u64`.
macro_rules! define_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(u64);

        impl $name {
            /// Creates the identifier from its raw value.
            #[must_use]
            pub const fn new(raw: u64) -> Self {
                Self(raw)
            }

            /// Returns the raw value.
            #[must_use]
            pub const fn get(self) -> u64 {
                self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($name), "({})"), self.0)
            }
        }
    };
}

define_id!(
    /// A unique identifier for a [`Workspace`].
    WorkspaceId
);
define_id!(
    /// A unique identifier for a Capability.
    ///
    /// See `docs/04-capability-model.md`.
    CapabilityId
);

/// The independent versions of a Workspace's parts.
///
/// Software configuration, workflow, interface and data are versioned
/// separately, so a change to one does not force a new version of another
/// (`docs/03-workspace.md`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WorkspaceVersion {
    /// The software-configuration version.
    pub software: u32,
    /// The workflow version.
    pub workflow: u32,
    /// The interface version.
    pub interface: u32,
    /// The data version.
    pub data: u32,
}

/// The goal of the task a [`Workspace`] serves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskGoal(String);

impl TaskGoal {
    /// Creates a goal from its description.
    #[must_use]
    pub fn new(description: impl Into<String>) -> Self {
        Self(description.into())
    }

    /// Returns the description.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The association of a Capability to a [`Workspace`].
///
/// A binding records whether a Capability is enabled and how it is configured
/// (`docs/04-capability-model.md`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapabilityBinding {
    /// The Capability that is bound.
    pub capability: CapabilityId,
    /// Whether the Capability is enabled in the Workspace.
    pub enabled: bool,
}

/// A persistent, integrated execution environment for one task.
///
/// This is the identity and composition of a Workspace. The workflow,
/// interface, data and history parts are added as their owning documents settle
/// them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workspace {
    /// The Workspace identifier.
    pub id: WorkspaceId,
    /// The goal of the task.
    pub goal: TaskGoal,
    /// The current independent versions of the parts.
    pub version: WorkspaceVersion,
    /// The Capabilities bound to the Workspace.
    pub capabilities: Vec<CapabilityBinding>,
}

impl Workspace {
    /// Creates an empty Workspace for `goal` at version zero.
    #[must_use]
    pub fn new(id: WorkspaceId, goal: TaskGoal) -> Self {
        Self {
            id,
            goal,
            version: WorkspaceVersion::default(),
            capabilities: Vec::new(),
        }
    }

    /// Binds a Capability, enabled by default.
    pub fn bind(&mut self, capability: CapabilityId) {
        self.capabilities.push(CapabilityBinding {
            capability,
            enabled: true,
        });
    }

    /// Returns the enabled Capabilities.
    pub fn enabled(&self) -> impl Iterator<Item = &CapabilityBinding> {
        self.capabilities.iter().filter(|binding| binding.enabled)
    }
}

#[cfg(test)]
mod tests {
    use super::{CapabilityBinding, CapabilityId, TaskGoal, Workspace, WorkspaceId};

    #[test]
    fn new_workspace_starts_empty_at_version_zero() {
        let workspace = Workspace::new(WorkspaceId::new(1), TaskGoal::new("analyse sales"));

        assert_eq!(workspace.id, WorkspaceId::new(1));
        assert_eq!(workspace.goal.as_str(), "analyse sales");
        assert_eq!(workspace.version, Default::default());
        assert_eq!(workspace.capabilities.len(), 0);
    }

    #[test]
    fn bind_adds_an_enabled_capability() {
        let mut workspace = Workspace::new(WorkspaceId::new(1), TaskGoal::new("analyse sales"));
        workspace.bind(CapabilityId::new(10));

        assert_eq!(workspace.capabilities.len(), 1);
        assert_eq!(workspace.enabled().count(), 1);
    }

    #[test]
    fn disabled_capabilities_are_not_enabled() {
        let mut workspace = Workspace::new(WorkspaceId::new(1), TaskGoal::new("analyse sales"));
        workspace.capabilities.push(CapabilityBinding {
            capability: CapabilityId::new(10),
            enabled: false,
        });

        assert_eq!(workspace.enabled().count(), 0);
    }

    #[test]
    fn identifiers_are_opaque_but_comparable() {
        assert_eq!(WorkspaceId::new(7).get(), 7);
        assert!(WorkspaceId::new(7) < WorkspaceId::new(8));
        assert_eq!(format!("{:?}", WorkspaceId::new(7)), "WorkspaceId(7)");
    }
}
