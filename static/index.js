$(function(){
    const colours = {
        "true": "green",
        "false": "#aa0000",
    };
    let led_on = false;

    $.get("/api/led/status", status => {
      led_on = status;
      $("#content").css("background-color", colours[status]);
    });

    setInterval(() => {
        $.get("/api/led/status", status => {
          led_on = status;
          $("#content").css("background-color", colours[status]);
        }).fail(_ => {
            $("#content").css("background-color", "white");
        });
    }, 5000);


    $(".border-transition").on("click", e => {
        $.post(`/api/led/${e.target.dataset.request}`, status => {
            led_on = status;
            $("#content").css("background-color", colours[status]);
        });
    });
})
