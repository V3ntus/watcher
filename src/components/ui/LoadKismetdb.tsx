import dynamic from 'next/dynamic';
import {useEffect, useState} from "react";
import {FileInput} from "@mantine/core";

import * as wasm_js from '@/../pkg/watcher_lib';

function LoadKismetdbComponent() {
    const [file, setFile] = useState<File | null>(null);

    useEffect(() => {
        // file?.arrayBuffer()?.then((v) => console.log(new Uint8Array(v)));
        fetch('@/../pkg/watcher_lib_bg.wasm').then(response => {
            return response.arrayBuffer();
        }).then(bytes => {
            if (file) {
                wasm_js.initSync(bytes);
                file.arrayBuffer()?.then((v) => console.log(wasm_js.load_kismetdb(new Uint8Array(v))));
            }
        });
    }, [file]);

    return (
        <FileInput
            size="md"
            radius="lg"
            label="Open"
            description="Load Kismetdb file"
            placeholder="example.kismetdb"
            onChange={setFile}
        />
    )
}

const WasmLoadKismetdb = dynamic(() => Promise.resolve(LoadKismetdbComponent), {
    ssr: false
});

export default WasmLoadKismetdb
