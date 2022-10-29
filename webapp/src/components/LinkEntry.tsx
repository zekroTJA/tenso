import styled, { css } from "styled-components";

import { Link } from "../lib/tenso";

type Props = {
  link: Link;
  onClick: (link: Link) => void;
};

const Container = styled.div<{ disabled: boolean }>`
  background-color: ${(p) => p.theme.background2};
  padding: 1em;
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  cursor: pointer;

  transition: all 0.15s ease;

  > h2 {
    font-weight: 300;
    margin-bottom: 0.6em;
    ${(p) =>
      p.disabled &&
      css`
        text-decoration: line-through;
        opacity: 0.75; ;
      `}
  }

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

export const LinkEntry: React.FC<Props> = ({ link, onClick }) => {
  return (
    <Container onClick={() => onClick(link)} disabled={!link.enabled}>
      <h2>{link.ident}</h2>
      <span>{link.destination}</span>
    </Container>
  );
};
