import styled, { css } from "styled-components";

import { ReactComponent as ClipboardIcon } from "../assets/clipboard.svg";
import { Link } from "../lib/tenso";
import { useNotification } from "../hooks/useNotification";

type Props = {
  link: Link;
  onClick: (link: Link) => void;
};

const Container = styled.div`
  background-color: ${(p) => p.theme.background2};
  padding: 1em;
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  cursor: pointer;

  transition: all 0.15s ease;

  > span {
    width: 100%;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    opacity: 0.7;
  }

  &:hover {
    background-color: ${(p) => p.theme.background3};
  }
`;

const Ident = styled.div<{ disabled: boolean }>`
  display: flex;
  align-items: center;
  gap: 0.5em;
  margin-bottom: 0.6em;

  width: fit-content;
  border-radius: 5px;

  transition: all 0.2s ease;

  > h2 {
    font-weight: 300;
    margin: 0;
    ${(p) =>
      p.disabled &&
      css`
        text-decoration: line-through;
        opacity: 0.75;
      `}
  }

  > svg {
    opacity: 0;
    transition: all 0.2s ease;
  }

  &:hover {
    background-color: rgba(255 255 255 / 20%);

    > svg {
      opacity: 1;
    }
  }
`;

const Destination = styled.span``;

export const LinkEntry: React.FC<Props> = ({ link, onClick }) => {
  const { showNotification } = useNotification();

  const onIdentClick = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
    e.stopPropagation();

    navigator.clipboard
      .writeText(`${window.origin}/${link.ident}`)
      .then(() =>
        showNotification("Short link has been copied to clipboard.", "success")
      )
      .catch(() =>
        showNotification("Failed copying link to clipboard.", "error")
      );
  };

  return (
    <Container onClick={() => onClick(link)}>
      <Ident disabled={!link.enabled} onClick={onIdentClick}>
        <h2>{link.ident}</h2>
        <ClipboardIcon />
      </Ident>
      <Destination>{link.destination}</Destination>
    </Container>
  );
};
