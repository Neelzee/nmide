/**
 * Client cmd for sending an Event to the Backend
 */
export const EVENT_INVOKER = "event";

/**
 * Client cmd for retrieving the State
 */
export const STATE_INVOKER = "state";

/**
 * Client cmd for retrieving the UI
 */
export const UI_INVOKER = "ui";

/**
 * Listener Tauri.eventName for when modules are installed
 */
export const NMIDE_MODULES_INSTALLED_EVENT = "nmide://modulesInstalled";

/**
 * Listener Tauri.eventName for when the frontend is initialized
 */
export const NMIDE_INITIALIZED = "nmide://initialized";

/**
 * Listener Tauri.eventName for when a runtime module is installed
 */
export const NMIDE_RT_MODULE_INSTALLED_EVENT = "nmide://rtModuleInstalled";

/**
 * Listener Tauri.eventName for when a runtime module is loaded
 */
export const NMIDE_RT_MODULE_PUSHED_EVENT = "nmide://rtModulePushed";

