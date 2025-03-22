use db::orders::OrderId;
use leptos_router::hooks::use_params;

use crate::{components::orders::OrdersRoute, db::DbState, errors::components::Pre, prelude::*};

pub fn ViewOrder() -> impl IntoView {
  #[derive(Debug, thiserror::Error, Clone)]
  enum ComponentError {
    #[error("Connecting to db")]
    DbDisconnected(#[from] GenericError<crate::db::ConnectErr>),

    #[error("Loading order info")]
    LoadingOrderInfo,

    #[error("Invalid route: Missing order id route param")]
    MissingOrderIdRouteParam(#[from] leptos_router::params::ParamsError),

    #[error("Could not find order with id {id:?}")]
    CouldntFindOrder { id: OrderId },
  }

  impl IntoRender for ComponentError {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
      view! {
        <p> { self.to_string() }</p>
        <Pre err=self />
      }
      .into_any()
    }
  }

  let db = DbState::from_context();

  let order_id = use_params::<OrdersRoute>();
  let order = move || -> Result<db::orders::Order, ComponentError> {
    let id: OrderId = order_id.get()?.order_id;
    let orders = db
      .read()
      .conn_old()
      .err_generic_ref()?
      .orders_downgraded()
      .select()
      .read();
    let order = orders
      .as_ref()
      .ok_or(ComponentError::LoadingOrderInfo)?
      .iter()
      .find(|o| o.id() == id.clone())
      .ok_or(ComponentError::CouldntFindOrder { id })?;

    Ok(order.clone())
  };
  move || order().map_view(|order| view! {<Order order=order.clone() />})
}

#[component]
fn Order(order: db::orders::Order) -> impl IntoView {
  let title = format!("Viewing status for order id {}", order.id().key());
  let status = order.status().clone();

  view! {
    <div>
      <h1>{title}</h1>
      <OrderStatus status />
      <PromoteOrderPaid order />
    </div>
  }
}

#[component]
fn OrderStatus(#[prop(into)] status: Signal<db::orders::OrderStatus>) -> impl IntoView {
  let ui = move || {
    match status.get() {
    db::orders::OrderStatus::Unpaid => view! {
      <span>"Unpaid"</span>
      <p>
        "This means this order was never taken to the checkout, which shouldn't normally happen"
      </p>
      <p>
        "If you did pay for this order, we can help! All checkout sessions are recorded by our payments provider, Stripe"
      </p>
      <crate::components::support::CustomerSupportLinks />
    }.into_any(),
    db::orders::OrderStatus::WaitingForPayment(_) => view! {
      <span>"Waiting for payment"</span>
    }.into_any(),
    db::orders::OrderStatus::Paid(_) => view! {
      <span>"Paid!"</span>
      <p>"Your order has been paid for, and will be resolved within the next business day"</p>
    }.into_any(),
    db::orders::OrderStatus::Resolved { .. } => view! {
      <span>"Resolved"</span>
      <p>
        "This means your order has been processed by our inventory management system and all you have to do it wait"
      </p>
    }.into_any()
  }
  };
  view! { <div>{ui}</div> }
}

/// Will only render if the order status is correct
#[component]
fn PromoteOrderPaid(#[prop(into)] order: Signal<db::orders::Order>) -> impl IntoView {
  #[derive(Debug, thiserror::Error, Serialize, Deserialize)]
  enum ComponentError {
    #[error("There was an error talking with the backend")]
    ServerFnErr(ServerFnError),

    #[error("There was an error marking your order as paid: {0}")]
    PromotionErr(#[from] GenericError<payments::PromoteOrderPaidErr>),
  }

  impl IntoRender for ComponentError {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
      view! {
        <p>{self.to_string()}</p>
        <Pre err=self />
      }
      .into_any()
    }
  }

  impl From<ServerFnError> for ComponentError {
    fn from(err: ServerFnError) -> Self {
      ComponentError::ServerFnErr(err)
    }
  }

  async fn promote_order_paid(order: OrderId) -> Result<(), ComponentError> {
    Ok(_server_promote_order_paid(order).await??)
  }

  #[server(
    prefix = "/api/orders",
    endpoint = "/promote-paid",
    input = server_fn::codec::Json,
    output = server_fn::codec::Json
  )]
  async fn _server_promote_order_paid(
    order: OrderId,
  ) -> Result<Result<(), GenericError<payments::errors::PromoteOrderPaidErr>>, ServerFnError> {
    Ok(server_promote_order_paid(order).await.err_generic())
  }

  #[cfg(feature = "ssr")]
  async fn server_promote_order_paid(
    order: OrderId,
  ) -> Result<(), payments::errors::PromoteOrderPaidErr> {
    let stripe = crate::server_state::ServerAxumState::from_context().stripe;
    stripe.promote_order_paid(order).await
  }

  let fallback = || {
    view! {
      <p>"Your payment is being processed in the background ..."</p>
      <p>"Reload the page if this persists"</p>
    }
  };
  let ui = move || {
    Suspend::new(async move {
      promote_order_paid(order.get().id())
        .await
        .map_view(|_: ()| {
          view! {
            <p> "Yay! Your order has successfully updated to the paid status!"</p>
            <p> "Reload to see this change updated" </p>
          }
        })
    })
  };

  // skip if not the right status
  let ui = move || {
    view! {
      <Transition fallback>
        { ui }
      </Transition>
    }
  };
  move || {
    matches!(
      order.read().status(),
      db::orders::OrderStatus::WaitingForPayment(_)
    )
    .then_some(Some(ui))
  }
}
