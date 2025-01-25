/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Device {
  name: string
  description: string
  state: string
  id: string
}
export declare class WaveFormatStruct {
  constructor(storebits: number, validbits: number, sampleType: number, samplerate: number, channels: number)
  init(): void
  start(callback: (arg0: Buffer) => void): void
  getStatus(): number
  setStatus(val: number): void
  static getStatic(): number
  getDevice(): Device
}
