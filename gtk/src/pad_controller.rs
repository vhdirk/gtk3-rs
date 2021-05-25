// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PadActionEntry;
use crate::PadController;
use glib::translate::*;

impl PadController {
    /// This is a convenience function to add a group of action entries on
    /// `self`. See [PadActionEntry](crate::PadActionEntry) and [PadController::set_action](crate::PadController::set_action).
    /// ## `entries`
    /// the action entries to set on `self`
    #[doc(alias = "gtk_pad_controller_set_action_entries")]
    pub fn set_action_entries(&self, entries: &[PadActionEntry]) {
        let n_entries = entries.len() as i32;
        let entry_strings = entries
            .iter()
            .map(|e| (e.label().to_glib_none(), e.action_name().to_glib_none()))
            .collect::<Vec<(Stash<_, _>, Stash<_, _>)>>();
        let entries = entries
            .iter()
            .zip(entry_strings.iter())
            .map(|(e, (label, action_name))| ffi::GtkPadActionEntry {
                type_: e.type_().into_glib(),
                index: e.index(),
                mode: e.mode(),
                label: label.0,
                action_name: action_name.0,
            })
            .collect::<Vec<_>>();
        unsafe {
            ffi::gtk_pad_controller_set_action_entries(
                self.to_glib_none().0,
                entries.as_ptr(),
                n_entries,
            );
        }
    }
}
