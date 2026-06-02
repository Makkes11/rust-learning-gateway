export type Device = {
  id: string;
  name: string;
};

export function getDevices(): Promise<Device[]> {
  return Promise.resolve([
    { id: "1", name: "Pump A" },
    { id: "2", name: "Motor B" },
    { id: "3", name: "Sensor C" }
  ]);
}