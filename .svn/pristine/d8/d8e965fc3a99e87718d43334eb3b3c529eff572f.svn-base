import type { NamedDependency } from "./rsm-grapher/rsm-invoker/bindings/NamedDependency";
import type { Event } from "./rsm-grapher/rsm-invoker/bindings/Event";
import * as fs from "fs";


const data: NamedDependency[] = JSON.parse(fs.readFileSync("./build/result.json", "utf-8"));

const s_t = data.flatMap(nd => {
    const consumer = nd.name;
    return nd.consuming.map(({ event_name, module_name }) => {
        return {
            consumer,
            consuming: {
                event_name,
                module_name
            }
        };
    });
});

const providers = data.flatMap(nd => {
    const provider = nd.name;
    return nd.providing.map(evt => {
        return {
            provider,
            providing: {
                ...evt
            }
        };
    });
});

const links: { source: string, target: string, event: Event }[] = [];

s_t.forEach(({ consumer, consuming }) => {
    const source = consumer;
    providers.forEach(({ provider, providing }) => {
        const target = provider;
        if (consuming.event_name === providing.event) {
            links.push({ source, target, event: providing });
        }

        if (consuming.module_name === providing.module) {
            links.push({ source, target, event: providing });
        }
    });
});


console.log(links);