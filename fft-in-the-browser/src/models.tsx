export interface HostedWavFile {
  id: number;
  name: string;
  url: string;
  description: string;
}

export interface WavSpec {
  sampleRate: number;
  bitsPerSample: number;
  channels: number;
}
