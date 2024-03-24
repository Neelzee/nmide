import { createSignal } from "solid-js";

export function Explorer(props: { files: string[] }) {
	const [files, setFiles] = createSignal<string[]>(props.files);
	return (
		<section class="explorer">
			{files().map((f) => {
				return <p>{f}</p>;
			})}
		</section>
	);
}
