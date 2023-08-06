$(function(){
    const colours = {
        "true": "green",
        "false": "#aa0000",
    };
    let led_on = false;
    $.get("http://127.0.0.1:8080/api/led/status", status => {
        led_on = status;
        $("#content").css("background-color", colours[status]);
    });

    $(".border-transition").on("click", e => {
        $.post(`http://127.0.0.1:8080/api/led/${e.target.dataset.request}`, status => {
            led_on = status;
            $("#content").css("background-color", colours[status]);
        });
    });
})
