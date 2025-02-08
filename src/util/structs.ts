// noinspection JSUnusedLocalSymbols

import {DLT} from "@/util/dlt_enum";

interface LatLon {
    lat: number;
    lon: number;
}

interface KismetData {
    ts_sec: Date;
    ts_usec: number;
    phyname: string;
    devmac: string;
    latlon: LatLon;
    speed: number;
    heading: number;
    datasource: KismetDataSource;
}

interface KismetDataSource {
    uuid: string;
    typestring: string;
    definition: string;
    name: string;
    interface: string;
}

interface KismetDevice {
    first_time: Date;
    last_time: Date;
    devkey: string;
    phyname: string;
    devmac: string;
    strongest_signal: number;
    min_latlon: LatLon;
    max_latlon: LatLon;
    avg_latlon: LatLon;
    type: string;
}

interface KismetMessage {
    ts_sec: Date;
    latlon: LatLon;
    msgtype: string;
    message: string;
}

interface KistmetPacket {
    ts_sec: Date;
    ts_usec: number;
    phyname: string;
    sourcemac: string;
    destmac: string;
    transmac: string;
    frequency: number;
    devkey: string;
    latlon: LatLon;
    alt: number;
    speed: number;
    heading: number;
    packet_len: number;
    signal: number;
    datasource: KismetDataSource;
    dlt: DLT;
    error: boolean;
    tags: string[];
    datarate: number;
    hash: number;
    packetid: number;
}

interface KismetSnapshot {
    ts_sec: Date;
    ts_usec: number;
    latlon: LatLon;
    snaptype: string;
    json: object;
}
