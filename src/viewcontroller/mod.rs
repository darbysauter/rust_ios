use ctor::*;
use crate::objc::*;

// This is a simple -drawRect implementation for our class. We could have 
// used a UILabel  or something of that sort instead, but I felt that this 
// stuck with the C-based mentality of the application.
extern "C" fn view_draw_rect(_obj: Id, _cmd: Sel, _rect: CGRect)
{
    // We are simply getting the graphics context of the current view, 
    // so we can draw to it
    let context = unsafe {
         UIGraphicsGetCurrentContext()
    };

    // Then we set it's fill color to white so that we clear the background.
    // Note the cast to (CGFloat []). Otherwise, this would give a warning
    //  saying "invalid cast from type 'int' to 'CGFloat *', or 
    // 'extra elements in initializer'. Also note the assumption of RGBA.
    // If this wasn't a demo application, I would strongly recommend against this,
    // but for the most part you can be pretty sure that this is a safe move 
    // in an iOS application.
    let color: [f64; 4] = [1.0, 1.0, 1.0, 1.0];
    unsafe {
        CGContextSetFillColor(context, color.as_ptr());


        // here, we simply add and draw the rect to the screen
        CGContextAddRect(context, CGRect::new(0.0, 0.0, 320.0, 480.0));
        CGContextFillPath(context);
    }

    // and we now set the drawing color to red, then add another rectangle
    // and draw to the screen
    let color: [f64; 4] = [1.0, 0.0, 0.0, 1.0];
    unsafe {
        CGContextSetFillColor(context, color.as_ptr());
        CGContextAddRect(context, CGRect::new(10.0, 10.0, 20.0, 20.0));
        CGContextFillPath(context);
    }
}

#[ctor]
fn init_view() {
    // Once again, just like the app delegate, we tell the runtime to 
    // create a new class, this time a subclass of 'UIView' and named 'View'.
    let view_class = rust_objc_allocate_class_pair(rust_objc_get_class("UIView") as Id, "View", 0);

    // and again, we tell the runtime to add a function called -drawRect: 
    // to our custom view. Note that there is an error in the type-specification
    // of this method, as I do not know the @encode sequence of 'CGRect' off 
    // of the top of my head. As a result, there is a chance that the rect 
    // parameter of the method may not get passed properly.
    let cast_fn: Imp = unsafe {
        *(&view_draw_rect as *const _ as usize as *const Imp)
    };
    rust_class_add_method(view_class, rust_sel_get_uid("drawRect:"), cast_fn, "v@:");

    // And again, we tell the runtime that this class is now valid to be used. 
    // At this point, the application should run and display the screenshot shown below.
    unsafe {
        objc_registerClassPair(view_class);
    }
}