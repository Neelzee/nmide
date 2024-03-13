import { createSignal } from "solid-js/types/server/reactive.js";

export function Explorer(props: { files: string[] }) {
	const [files, setFiles] = createSignal<string[]>([]);

	return (
		<section class="explorer">
			{files().map((f) => {
				return <p>{f}</p>;
			})}
		</section>
	);
}
