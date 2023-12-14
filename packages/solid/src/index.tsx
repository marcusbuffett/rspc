import * as Solid from "solid-js";
import {
  Client,
  inferMutationInput,
  inferMutationResult,
  inferQueryInput,
  inferQueryResult,
  inferSubscriptionResult,
  ProceduresDef,
  RSPCError,
  _inferInfiniteQueryProcedureHandlerInput,
  _inferProcedureHandlerInput,
} from "@rspc/client";
import {
  QueryClient,
  CreateQueryOptions,
  CreateQueryResult,
  createQuery as __createQuery,
  createInfiniteQuery as __createInfiniteQuery,
  createMutation as __createMutation,
  CreateMutationOptions,
  CreateMutationResult,
  QueryClientProvider,
} from "@tanstack/solid-query";
import { AlphaClient, AlphaRSPCError } from "@rspc/client/v2";

export interface BaseOptions<TProcedures extends ProceduresDef> {
  rspc?: {
    client?: Client<TProcedures>;
  };
}

export interface SubscriptionOptions<TOutput> {
  enabled?: boolean;
  onStarted?: () => void;
  onData: (data: TOutput) => void;
  onError?: (err: RSPCError) => void;
}

interface Context<TProcedures extends ProceduresDef> {
  client: Client<TProcedures>;
  queryClient: QueryClient;
}

type KeyAndInput = [string] | [string, any];

export type HooksOpts<P extends ProceduresDef> = {
  context: Solid.Context<Context<P>>;
};

export function createReactQueryHooks<P extends ProceduresDef>(
  client: AlphaClient<P>,
  opts?: HooksOpts<P>
) {
  type TBaseOptions = BaseOptions<P>;

  const mapQueryKey: (keyAndInput: KeyAndInput) => KeyAndInput =
    (client as any).mapQueryKey || ((x) => x);
  const Context = opts?.context || Solid.createContext<Context<P>>(undefined!);

  function useContext() {
    const ctx = Solid.useContext(Context);
    if (ctx?.queryClient === undefined)
      throw new Error(
        "The rspc context has not been set. Ensure you have the <rspc.Provider> component higher up in your component tree."
      );
    return ctx;
  }

  function createQuery<
    K extends P["queries"]["key"] & string,
    TQueryFnData = inferQueryResult<P, K>,
    TData = inferQueryResult<P, K>
  >(
    keyAndInput: () => [
      key: K,
      ...input: _inferProcedureHandlerInput<P, "queries", K>
    ],
    opts?: Omit<
      CreateQueryOptions<
        TQueryFnData,
        AlphaRSPCError,
        TData,
        () => [K, inferQueryInput<P, K>]
      >,
      "queryKey" | "queryFn"
    > &
      TBaseOptions
  ): CreateQueryResult<TData, RSPCError> {
    const { rspc, ...rawOpts } = opts ?? {};
    let client = rspc?.client;
    if (!client) {
      client = useContext().client;
    }

    return __createQuery({
      queryKey: mapQueryKey(keyAndInput as any) as any,
      queryFn: async () => client!.query(keyAndInput()),
      ...(rawOpts as any),
    });
  }

  // function createInfiniteQuery<
  //   K extends inferInfiniteQueries<TProcedures>["key"] & string
  // >(
  //   keyAndInput: () => [
  //     key: K,
  //     ...input: _inferInfiniteQueryProcedureHandlerInput<TProcedures, K>
  //   ],
  //   opts?: Omit<
  //     CreateInfiniteQueryOptions<
  //       inferInfiniteQueryResult<TProcedures, K>,
  //       RSPCError,
  //       inferInfiniteQueryResult<TProcedures, K>,
  //       inferInfiniteQueryResult<TProcedures, K>,
  //       () => [K, inferQueryInput<TProcedures, K>]
  //     >,
  //     "queryKey" | "queryFn"
  //   > &
  //     TBaseOptions
  // ): CreateInfiniteQueryResult<
  //   inferInfiniteQueryResult<TProcedures, K>,
  //   RSPCError
  // > {
  //   const { rspc, ...rawOpts } = opts ?? {};
  //   let client = rspc?.client;
  //   if (!client) {
  //     client = useContext().client;
  //   }

  //   return __createInfiniteQuery({
  //     queryKey: keyAndInput,
  //     queryFn: async () => {
  //       throw new Error("TODO"); // TODO: Finish this
  //     },
  //     ...(rawOpts as any),
  //   });
  // }

  function createMutation<
    K extends P["mutations"]["key"] & string,
    TContext = unknown
  >(
    key: K | [K],
    opts?: CreateMutationOptions<
      inferMutationResult<P, K>,
      AlphaRSPCError,
      inferMutationInput<P, K> extends never
        ? undefined
        : inferMutationInput<P, K>,
      TContext
    > &
      TBaseOptions
  ): CreateMutationResult<
    inferMutationResult<P, K>,
    AlphaRSPCError,
    inferMutationInput<P, K> extends never
      ? undefined
      : inferMutationInput<P, K>,
    TContext
  > {
    const { rspc, ...rawOpts } = opts ?? {};
    let client = rspc?.client;
    if (!client) {
      client = useContext().client;
    }

    return __createMutation({
      mutationFn: async (input) => {
        const actualKey = Array.isArray(key) ? key[0] : key;
        return client!.mutation([actualKey, input] as any);
      },
      ...(rawOpts as any),
    });
  }

  function createSubscription<
    K extends P["subscriptions"]["key"] & string,
    TData = inferSubscriptionResult<P, K>
  >(
    keyAndInput: () => [
      key: K,
      ...input: _inferProcedureHandlerInput<P, "subscriptions", K>
    ],
    opts: SubscriptionOptions<TData> & TBaseOptions
  ) {
    let client = opts?.rspc?.client;
    if (!client) {
      client = useContext().client;
    }
    const enabled = () => opts?.enabled ?? true;

    Solid.createEffect(() => {
      if (!enabled()) {
        return;
      }

      return client.addSubscription<K, TData>(keyAndInput(), {
        onData: opts.onData,
        onError: opts.onError,
      });
    });
  }

  return {
    _rspc_def: undefined! as P, // This allows inferring the operations type from TS helpers
    Provider: (props: {
      children?: Solid.JSX.Element;
      client: AlphaClient<P>;
      queryClient: QueryClient;
    }) => (
      <Context.Provider
        value={{
          client: props.client,
          queryClient: props.queryClient,
        }}
      >
        <QueryClientProvider client={props.queryClient}>
          {props.children}
        </QueryClientProvider>
      </Context.Provider>
    ),
    useContext,
    createQuery,
    // createInfiniteQuery,
    createMutation,
    createSubscription,
  };
}
