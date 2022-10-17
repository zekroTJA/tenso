import { Link } from "../lib/tenso";
import styled from "styled-components";

type Props = {
  link: Link;
  onClick: (link: Link) => void;
};

const Container = styled.div`
  background-color: ${(p) => p.theme.background2};
  padding: 1em;
  border-radius: 12px;

  transition: all 0.15s ease;

  > h2 {
    font-weight: 300;
    margin-bottom: 0.6em;
  }

  &:hover {
    transform: scale(1.03);
  }
`;

export const LinkEntry: React.FC<Props> = ({ link, onClick }) => {
  return (
    <Container onClick={() => onClick(link)}>
      <h2>{link.ident}</h2>
      <span>span</span>
    </Container>
  );
};
