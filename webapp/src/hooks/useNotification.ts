import { NotificationType } from "../components/Notification";
import { useRef } from "react";
import { useStore } from "../services/store";

export const useNotification = () => {
  const [notification, setNotification] = useStore((s) => [
    s.notification,
    s.setNotification,
  ]);
  const timerRef = useRef<ReturnType<typeof setTimeout>>();

  const hideNotification = () => {
    setNotification({ show: false });
  };

  const showNotification = (
    content: string | JSX.Element,
    type: NotificationType = "info",
    duration: number = 3500
  ) => {
    if (timerRef.current) clearTimeout(timerRef.current);
    setNotification({ show: true, content, type });
    timerRef.current = setTimeout(() => hideNotification(), duration);
  };

  return { showNotification, hideNotification };
};
