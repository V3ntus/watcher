'use client'

import Navbar from '@/components/ui/Navbar';

import './page.module.css';
import WasmLoadKismetdb from "@/components/ui/LoadKismetdb";

export default function Home() {


    return (
        <div>
            <Navbar/>
            <WasmLoadKismetdb/>
        </div>
    )
}
