$(function(){
    const colours = {
        "true": "green",
        "false": "#aa0000",
    };
    let led_on = false;

    $(".border-transition").on("click", e => {
        $.post(`/api/led/${e.target.dataset.request}`, `${led_on}`, status => {
            led_on = status;
            $("#content").css("background-color", colours[status]);
        });
    });
})
