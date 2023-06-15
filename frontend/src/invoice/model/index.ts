export type BillingItem = {
  id: string;
  name: string;
  timestamp: Date;
  price: number;
  value: number;
  treated: boolean;
};

export type Billing = BillingDTO & {
  price: number;
};

export type BillingDTO = {
  id: string;
  generatedAt: string;
  items: BillingItem[];
};
