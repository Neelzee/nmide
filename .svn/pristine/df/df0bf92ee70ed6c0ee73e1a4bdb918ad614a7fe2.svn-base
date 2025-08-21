import {
    Core,
    Event,
    CoreModification,
    emptyCm,
} from "@nmide/js-utils";
import { invoke } from "@tauri-apps/api/core";
import { handlerRegistration } from "./lib/handlerRegistration.ts";
import { eventThrower } from "./lib/eventThrower.ts";

export const tsHandler = async (evt: Event) => {
    if (typeof evt === "object" && "event" in evt) {
        const { event, args } = evt.event;
        const core: Core = {
            // @ts-expect-error This will succeed
            state: invoke<object>("state").catch(err => console.error(err)),
            // @ts-expect-error This will succeed
            ui: invoke<object>("ui").catch(err => console.error(err)),
            registerHandler: handlerRegistration,
            throwEvent: eventThrower,
        };

        const event_modules = window.__nmideConfig__.handlerRegister.event.get(event);
        const modules = event_modules === undefined ? [] : event_modules;
        // TODO: Add proper validation/handling
        const modifications: CoreModification[] = await Promise.all(
            modules
                .map(m => window.__nmideConfig__.modules.get(m))
                .filter(m => m !== undefined)
                .map(m => m.handler({ event: { event, args } }, core))
        );

        return modifications;
    }
    return [];
}

export const tsInit = async () => {

    const core: Core = {
        // @ts-expect-error This will succeed
        state: invoke<object>("state").catch(err => console.error(err)),
        // @ts-expect-error This will succeed
        ui: invoke<object>("ui").catch(err => console.error(err)),
        registerHandler: handlerRegistration,
        throwEvent: eventThrower,
    };

    // TODO: Figure out a way to sort modules by runtime
    const modules = Array.from(window.__nmideConfig__.modules.values());
    // TODO: Add proper validation/handling
    const modifications: CoreModification[] = await Promise.all(
        modules.map(m => m.init(core))
    );
    return modifications;
}
