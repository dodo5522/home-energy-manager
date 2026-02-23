import {QueryClient, QueryClientProvider} from '@tanstack/react-query';
import type {ReactNode} from 'react';

let context:
  | {
  queryClient: QueryClient;
}
  | undefined;

const getContext = () => {
  if (context) {
    return context;
  }

  const queryClient = new QueryClient();

  context = {
    queryClient,
  };

  return context;
};

const TanStackQueryProvider = ({children}: { children: ReactNode }) => {
  const {queryClient} = getContext();

  return (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
};

export {getContext};
export default TanStackQueryProvider;
