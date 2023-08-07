function getAndSetStatus(led_on) {
    $.get("/api/led/status", status => {
      led_on = status;
      $("#content").css("background-color", colours[status]);
    });
}


$(function(){
    const colours = {
        "true": "green",
        "false": "#aa0000",
    };
    let led_on = false;
    getAndSetStatus(led_on);
    setInterval(getAndSetStatus(led_on), 5000);


    $(".border-transition").on("click", e => {
        $.post(`/api/led/${e.target.dataset.request}`, status => {
            led_on = status;
            $("#content").css("background-color", colours[status]);
        });
    });
})
