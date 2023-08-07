
$(function(){
    const colours = {
        "true": "green",
        "false": "#aa0000",
    };
    let led_on = false;
    $.get("/led/status", status => {
      led_on = status;
      $("#content").css("background-color", colours[status]);
    });

    $(".border-transition").on("click", e => {
        $.post(`/led/${e.target.dataset.request}`, status => {
            led_on = status;
            $("#content").css("background-color", colours[status]);
        });
    });
})
