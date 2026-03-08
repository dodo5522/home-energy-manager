import {TanStackDevtools} from '@tanstack/react-devtools';
import {HeadContent, Scripts} from '@tanstack/react-router';
import {TanStackRouterDevtoolsPanel} from '@tanstack/react-router-devtools';
import type {ReactNode} from 'react';
import Header from '#/components/molecules/Header.tsx';
import {
  TanStackQueryDevtools,
  TanStackQueryProvider,
} from '#/integrations/tanstack-query';

const RootDocument = ({children}: { children: ReactNode }) => {
  return (
    <html lang="en">
    <head>
      <HeadContent/>
    </head>
    <body>
    <TanStackQueryProvider>
      <Header/>
      {children}
      <TanStackDevtools
        config={{
          position: 'bottom-right',
        }}
        plugins={[
          {
            name: 'Tanstack Router',
            render: <TanStackRouterDevtoolsPanel/>,
          },
          TanStackQueryDevtools,
        ]}
      />
    </TanStackQueryProvider>
    <Scripts/>
    </body>
    </html>
  );
};

export default RootDocument;
