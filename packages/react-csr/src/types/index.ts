export type Release = {
  diagramUid: string | null;
  version: string;
  uid: string;
  status: number;
  date: Date;
  description: string;
  openSource: boolean;
  usageCounter: number;
  pins: Pin[];
  supportedResourcesRange: SupportedResourcesRange;
};

export type Pin = {
  uid: string;
  name: string;
  binding: number;
  tokenMultiplicity: number;
  dataMultiplicity: number;
  dataTypeUid: string;
  dataTypeName: string;
  dataStructureUid: string | null;
  dataStructureName: string | null;
  accessTypeUid: string | null;
  accessTypeName: string | null;
};

export type SupportedResourcesRange = {
  minCPUs: number;
  minGPUs: number;
  minMemory: number;
  minStorage: number;
  maxCPUs: number;
  maxGPUs: number;
  maxMemory: number;
  maxStorage: number;
};

export type App = {
  name: string;
  uid: string;
  pClass: string;
  shortDescription: string;
  longDescription: string;
  icon: string;
  isApp: boolean;
  isService: boolean;
  releases: Release[];
  inCockpit?: boolean;
};

export type Shelf = {
  diagramUid: string;
  unit: App;
  version: string;
  uid: string;
  status: number;
  date: Date;
  description: null;
  openSource: boolean;
  usageCounter: number;
  pins: Pin[];
  supportedResourcesRange: SupportedResourcesRange;
};

export type Filter = {
  sort: string;
  search: string;
};
