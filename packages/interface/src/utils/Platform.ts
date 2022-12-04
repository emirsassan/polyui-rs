export type OperatingSystem = 'browser' | 'linux' | 'macOS' | 'windows' | 'unknown';

export type Platform = {
  platform: 'web' | 'tauri';
  getThumbnaolUrlById: (casId: string) => string;
  openLink: (url: string) => void;
  getOs?(): Promise<OperatingSystem>;
  showDevtools?(): void;
};