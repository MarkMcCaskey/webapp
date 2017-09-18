module CreateAccount exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Navigation
import Json.Decode as Decode
import Json.Encode as Encode


main =
  Html.program
    { init = init 
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

type alias Model =
    { name: String
    , password: String
    , confirmPassword: String
    }

init : (Model, Cmd Msg)
init = (Model "" "" "", Cmd.none)

type Msg
    = Name String
    | Password String
    | ConfirmPassword String
    | Submit 
    | SendName (Result Http.Error String)
    | Done

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Name name ->
            ({ model | name = name }, Cmd.none)

        Password password ->
            ({ model | password = password }, Cmd.none)

        ConfirmPassword cpassword ->
            ({ model | confirmPassword = cpassword }, Cmd.none)

        Submit ->
            (model, sendLoginInfo model.name model.password)

        SendName (Ok val) ->
            ({model | name = val}, Navigation.load "/hello")

        SendName (Err _) ->
            (model, Cmd.none)
        Done ->
            (model, Cmd.none)

view : Model -> Html Msg
view model =
    div []
        [ h1 [] [text "Login"]
        , input [type_ "text", placeholder "Name", onInput Name ] []
        , input [type_ "password", placeholder "Password", onInput Password ] []
        , input [type_ "password", placeholder "Password, again", onInput ConfirmPassword ] []
        , button [onClick Submit] [text "Login"]
        , viewValidation model
        ]

viewValidation : Model -> Html msg
viewValidation model =
    let
        (color, message) =
            if String.length model.password < 8 then
                ("red", "Password too short")
            else if model.password == model.confirmPassword then
                ("green", "OK")
            else
                ("red", "Passwords do not match")
    in
        div [ style [("color", color)] ] [ text message ]

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


sendLoginInfo : String -> String -> Cmd Msg
sendLoginInfo name password =
    Http.send SendName (loginRequest name password)


loginRequest : String -> String -> Http.Request String
loginRequest name password = Http.request
                    { method = "POST"
                    , headers = []
                    , url = "/create_account"
                    , body = Http.jsonBody (Encode.object [("username", Encode.string name), ("password", Encode.string password)])
                    , expect = Http.expectStringResponse (\v -> Ok v.status.message)
                    , timeout = Nothing
                    , withCredentials = True
                    }

