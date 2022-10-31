import Color from "color";
import styled from "styled-components";
import { useStore } from "../services/store";

export type NotificationType = "info" | "success" | "warn" | "error";

export type NotificationData = {
  type: NotificationType;
  content: string | JSX.Element;
  show: boolean;
};

type Props = {};

const Container = styled.div<{ show?: boolean; type?: NotificationType }>`
  position: fixed;
  top: 0px;
  width: 100%;
  display: flex;
  justify-content: center;
  transition: all 0.2s ease;

  transform: translateY(${(p) => (p.show ? "0px" : "-50px")});
  opacity: ${(p) => (p.show ? 1 : 0)};
  pointer-events: ${(p) => (p.show ? "all" : "none")};

  > div {
    width: 100%;
    max-width: calc(50em - 4em);
    padding: 1em;
    margin: 2em;
    border-radius: 13px;
    box-shadow: 00px 10px 30px 0px rgba(0 0 0 / 50%);

    background-color: ${(p) => {
      switch (p.type) {
        case "success":
          return Color(p.theme.green).darken(0.3).hexa();
        case "warn":
          return Color(p.theme.orange).darken(0.3).hexa();
        case "error":
          return Color(p.theme.red).darken(0.3).hexa();
        default:
          return Color(p.theme.cyan).darken(0.3).hexa();
      }
    }};
  }
`;

export const Notification: React.FC<Props> = ({}) => {
  const [notification, setNotification] = useStore((s) => [
    s.notification,
    s.setNotification,
  ]);
  return (
    <Container show={notification.show} type={notification.type}>
      <div>{notification.content}</div>
    </Container>
  );
};
