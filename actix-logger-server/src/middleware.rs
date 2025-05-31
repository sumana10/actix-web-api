use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

// Empty struct used as a middleware factory to create `RequestLoggerMiddleware`.
pub struct RequestLogger;

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    // `S` must be a service that takes `ServiceRequest` and returns a `ServiceResponse<B>`
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    // The future returned by the service must have a 'static lifetime
    S::Future: 'static,
    // The response body type must have a 'static lifetime
    B: 'static,
{
    // Associated types for the Transform trait
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    // The actual middleware type that will be created
    type Transform = RequestLoggerMiddleware<S>;
    // Future type returned when creating the middleware (ready immediately)
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    // Called to create and wrap the next service with the middleware.
    fn new_transform(&self, service: S) -> Self::Future {
    // Return a ready future with the middleware wrapping the provided service
        ready(Ok(RequestLoggerMiddleware { service }))
    }
}
// The actual middleware struct wrapping a service `S`.
pub struct RequestLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    // Boxed future type to erase the concrete future type
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Delegate readiness check to the wrapped service
    forward_ready!(service);

    // Handles each incoming request.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Record the start time to measure request duration
        let start_time = std::time::Instant::now();
        // Clone HTTP method and path for logging after the response is ready
        let method = req.method().clone();
        let path = req.path().to_string();
        
        // Call the next service in the chain asynchronously
        let fut = self.service.call(req);
        
       // Create an async block, wait for response, then log timing
        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            
            println!("{} {} - {}ms", method, path, duration.as_millis());
            
            Ok(res)
        })
    }
}
