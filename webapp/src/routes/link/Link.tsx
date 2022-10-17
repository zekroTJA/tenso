import { useParams } from "react-router-dom";

type Props = {};

export const LinkRoute: React.FC<Props> = ({}) => {
  const { ident } = useParams();

  return <>{ident}</>;
};
