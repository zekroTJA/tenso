import { NotificationData } from "../components/Notification";
import create from "zustand";

export type Store = {
  notification: Partial<NotificationData>;
  setNotification: (notification: Partial<NotificationData>) => void;
};

export const useStore = create<Store>((set, get) => ({
  notification: { show: false },
  setNotification: (notification) =>
    set({ notification: { ...get().notification, ...notification } }),
}));
