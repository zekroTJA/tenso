import styled, { useTheme } from "styled-components";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

import { ReactComponent as ArrowRightIcon } from "../../assets/arrow-right.svg";
import { ReactComponent as BackIcon } from "../../assets/back.svg";
import { ReactComponent as ExtLinkIcon } from "../../assets/ext-link.svg";
import { ReactComponent as EyeCheckIcon } from "../../assets/eye-check.svg";
import { ReactComponent as EyeOffIcon } from "../../assets/eye-off.svg";
import { Floatbar } from "../../components/Floatbar";
import { Link } from "../../lib/tenso";
import { ReactComponent as PencilIcon } from "../../assets/pencil.svg";
import { ReactComponent as TrashIcon } from "../../assets/trash.svg";
import { useApi } from "../../hooks/useApi";

type Props = {};

const Container = styled.div`
  padding: 1em;
  max-width: 50em;
  margin: 0 auto;
`;

const Destination = styled.div`
  display: flex;
  gap: 0.5em;
  align-items: center;
  margin-bottom: 1em;

  > a {
    color: ${(p) => p.theme.cyan};
    text-decoration: underline;
  }
`;

const Property = styled.section`
  display: flex;
  gap: 0.5em;
  align-items: center;
  margin-top: 0.5em;
`;

export const LinkRoute: React.FC<Props> = ({}) => {
  const theme = useTheme();
  const { id } = useParams();
  const fetch = useApi();
  const nav = useNavigate();
  const [link, setLink] = useState<Link>();

  const deleteLink = () => {
    if (!id) return;
    fetch((c) => c.linkDelete(id)).then(() => nav(-1));
  };

  useEffect(() => {
    if (!id) return;
    fetch((c) => c.link(id)).then((link) => setLink(link));
  }, [id]);

  return (
    <div>
      <Container>
        {link && (
          <>
            <h2>{link.ident}</h2>
            <Destination>
              <ArrowRightIcon />
              <a href={link.destination} target="_blank">
                {link.destination}
              </a>
            </Destination>
            {(link.enabled && (
              <Property>
                <EyeCheckIcon />
                <span>Enabled</span>
              </Property>
            )) || (
              <Property>
                <EyeOffIcon />
                <span>Disabled</span>
              </Property>
            )}
            <Property>
              <ExtLinkIcon />
              <span>
                {link.permanent_redirect
                  ? "Permanent Redirect"
                  : "Temporary Redirect"}
              </span>
            </Property>
          </>
        )}
      </Container>

      <Floatbar
        buttons={[
          {
            content: (
              <>
                <BackIcon />
                Back
              </>
            ),
            color: theme.orange,
            action: () => nav(-1),
          },
          {
            content: (
              <>
                <PencilIcon />
                Edit
              </>
            ),
            color: theme.accent,
            action: () => nav(`/${id}/edit`),
          },
          {
            content: (
              <>
                <TrashIcon />
                Delete
              </>
            ),
            color: theme.red,
            action: deleteLink,
          },
        ]}
      />
    </div>
  );
};
