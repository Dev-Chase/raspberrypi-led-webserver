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
        console.log("happneing");
        $.get("/api/led/status", status => {
          led_on = status;
          $("#content").css("background-color", colours[status]);
        });
    }, 5000);


    $(".border-transition").on("click", e => {
        $.post(`/api/led/${e.target.dataset.request}`, status => {
            led_on = status;
            $("#content").css("background-color", colours[status]);
        });
    });
})
