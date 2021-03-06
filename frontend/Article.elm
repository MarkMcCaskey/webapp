module Article exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Navigation
import Json.Decode as Decode
import Dict exposing (Dict)
import Array exposing (Array)
import Json.Encode
import Json.Decode exposing (field, Decoder)

apply : Decoder (a -> b) -> Decoder a -> Decoder b
apply f aDecoder = Decode.andThen (\fp -> Decode.map fp aDecoder) f

(|:) : Decoder (a -> b) -> Decoder a -> Decoder b
(|:) = apply

(:=) : String -> Decoder a -> Decoder a
(:=) = field

main =
  Html.program
    { init = init 
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

type alias WordList = Dict String String

type alias Model =
    { articleContent: Maybe ArticleContent }

getTitle : Model -> String
getTitle m = case m.articleContent of
                 Just c -> c.title
                 Nothing -> ""

getWordList : Model -> WordList
getWordList m = case m.articleContent of
                    Just c -> Dict.fromList c.unknown_words
                    Nothing -> Dict.empty

getText : Model -> List String
getText m = case m.articleContent of
                Just c -> Array.toList c.content_in_words
                Nothing -> []


init : (Model, Cmd Msg)
init = (Model Nothing, getArticle "13")
--(Model "Test title" "These are some words that will be split on Elm's end.  This is additional text that is being typed to allow words to appear multiple times." (Dict.fromList [("some", "the definition of some"), ("words", "a unit of language")]), Cmd.none)

type Msg
    = Init
    | Loading (Result Http.Error ArticleContent)
    | Normal
    | WordClicked String
    | DefinitionPopup
          
update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of 
        Init ->
            (model, getArticle "13")

        Loading (Ok articleContent) ->
            ({model | articleContent = Just articleContent}, Cmd.none)

        Loading (Err _) ->
            (model, Cmd.none)

        Normal ->
            (model, Cmd.none)

        WordClicked word_text ->
            (model, Cmd.none)

        DefinitionPopup ->
            (model, Cmd.none)
        

view : Model -> Html Msg
view model =
    div []
        [h1 [] [text (getTitle model)]
        , div [] [p [] (List.intersperse (text " ") (List.map (word (getWordList model)) (getText model)))]
        ]

word unknown_word_dict textContent =
    let word_attributes =
            if Dict.member textContent unknown_word_dict
            then [style [("backgroundColor", "red")]]
            else []
    in span word_attributes [text textContent]

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


type alias ArticleContent =
    { title: String
    , content_in_words: Array String 
    , unknown_words : List (String, String)
    , studying_words: List (String, Int)
    , language: String
    }

getArticle : String -> Cmd Msg 
getArticle articleID =
    let request = Http.get ("http://localhost:8888/article_content/" ++ articleID) decodeArticleContent
    in Http.send Loading <| request 

decodeArticleContent : Json.Decode.Decoder ArticleContent
decodeArticleContent =
    Json.Decode.succeed ArticleContent
        |: ("title" := Json.Decode.string)
        |: ("content_in_words" := Json.Decode.array Json.Decode.string)
        |: ("unknown_words" := Json.Decode.keyValuePairs Json.Decode.string)
        |: ("studying_words" := Json.Decode.keyValuePairs Json.Decode.int)
        |: ("language" := Json.Decode.string)

{--encodeArticleContent : ArticleContent -> Json.Encode.Value
encodeArticleContent record =
    Json.Encode.object
        [ ("unknown words",  Json.Encode.list <| List.map Json.Encode.dict <| record.unknownWords)
        , ("article text",  Json.Encode.list <| List.map Json.Encode.string <| record.articleText)
        ]

--}
